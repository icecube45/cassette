#!/usr/bin/env python

import asyncio
import random
import websockets
import json
import time
import threading
import numpy as np
import pyaudio
import config
import dsp
from scipy.ndimage.filters import gaussian_filter1d
from scipy.signal import butter,filtfilt, iircomb
width = 30
height = 10
num_pixels = width*height
pixels = [None]*(num_pixels)

def wheel(pos):
    # Input a value 0 to 255 to get a color value.
    # The colours are a transition r - g - b - back to r.
    if pos < 0 or pos > 255:
        r = g = b = 0
    elif pos < 85:
        r = int(pos * 3)
        g = int(255 - pos * 3)
        b = 0
    elif pos < 170:
        pos -= 85
        r = int(255 - pos * 3)
        g = 0
        b = int(pos * 3)
    else:
        pos -= 170
        r = 0
        g = int(pos * 3)
        b = int(255 - pos * 3)
    return (r, g, b)


def strip_to_matrix():
    for i in range(9):
        j=i+1
        pixels[j*30:j*30+30] = pixels[:30]


async def consumer(message):
    # while True:
    #     val = await queue.get()
    #     print('{} get a val: {}'.format(id, val))
    #     await asyncio.sleep(1)
    #     queue.task_done()   # indicate complete task
    pass


step = 0


def rgb_to_pixel_entry(rgb, patched):
    print(patched)
    return {
        "r": rgb[0],
        "g": rgb[1],
        "b": rgb[2],
        "patched": patched
    }



async def rainbow(matrix=False):
    global step
    num_pixels_override=0
    if(matrix):
        num_pixels_override = num_pixels
    else:
        num_pixels_override = width

    for i in range(num_pixels):
        pixel_index = (i * 256 // num_pixels_override) + step
        patch = True
        # if(i%3 == 0):
        #     patch = False
        pixels[i] = rgb_to_pixel_entry(wheel(pixel_index & 255), patch)
    # await asyncio.sleep(0.005)
    step += 8
    if(step == 256):
        step = 0

    if(matrix):
        strip_to_matrix()
   


    # while True:
    #     try:
    #         y = np.frombuffer(stream.read(frames_per_buffer, exception_on_overflow=False), dtype=np.int16)
    #         y = y.astype(np.float32)
    #         stream.read(stream.get_read_available(), exception_on_overflow=False)
    #         callback(y)
    #     except IOError:
    #         overflows += 1
    #         if time.time() > prev_ovf_time + 1:
    #             prev_ovf_time = time.time()
    #             print('Audio buffer has overflowed {} times'.format(overflows))
    # stream.stop_stream()
    # stream.close()
    # p.terminate()

p = pyaudio.PyAudio()
frames_per_buffer = int(config.MIC_RATE / config.FPS)
stream = p.open(format=pyaudio.paInt16,
                channels=1,
                rate=44100,
                input=True,
                frames_per_buffer=frames_per_buffer)
overflows = 0
prev_ovf_time = time.time()

async def producer():
    global stream
    # await rainbow()
    # for pixel in pixels:
    #     #get rand between 0 and 1
    #     if(random.random() > 0.9):
    #         pixel['patched'] = False

    y = np.frombuffer(stream.read(frames_per_buffer, exception_on_overflow=False), dtype=np.int16)
    y = y.astype(np.float32)
    stream.read(stream.get_read_available(), exception_on_overflow=False)

    ret = microphone_update(y)
    await asyncio.sleep(1/60)


    return json.dumps(ret)







async def handler(websocket, path):
    while True:
        listener_task = asyncio.ensure_future(websocket.recv())
        producer_task = asyncio.ensure_future(producer())
        done, pending = await asyncio.wait(
            [listener_task, producer_task],
            return_when=asyncio.FIRST_COMPLETED)

        if listener_task in done:
            message = listener_task.result()
            await consumer(message)
        else:
            listener_task.cancel()

        if producer_task in done:
            message = producer_task.result()
            # print(message)
            await websocket.send(message)
        else:
            producer_task.cancel()





fft_plot_filter = dsp.ExpFilter(np.tile(1e-1, config.N_FFT_BINS),
                         alpha_decay=0.5, alpha_rise=0.99)
mel_gain = dsp.ExpFilter(np.tile(1e-1, config.N_FFT_BINS),
                         alpha_decay=0.01, alpha_rise=0.99)
mel_smoothing = dsp.ExpFilter(np.tile(1e-1, config.N_FFT_BINS),
                         alpha_decay=0.5, alpha_rise=0.99)
volume = dsp.ExpFilter(config.MIN_VOLUME_THRESHOLD,
                       alpha_decay=0.02, alpha_rise=0.02)
gain = dsp.ExpFilter(np.tile(0.01, config.N_FFT_BINS),
                     alpha_decay=0.001, alpha_rise=0.99)
fft_window = np.hamming(int(config.MIC_RATE / config.FPS) * config.N_ROLLING_HISTORY)

prev_fps_update = time.time()
samples_per_frame = int(config.MIC_RATE / config.FPS)
# samples_per_frame=512
y_roll = np.random.rand(config.N_ROLLING_HISTORY, samples_per_frame) / 1e16

energy_roll = np.zeros((config.SUBBANDS, config.N_ENERGY_HISTORY))
bpm_roll = np.zeros(config.N_BPM_HISTORY)
oldBeats = [False]*config.SUBBANDS
energy_data = [0]*60*5

count = 0
newBeat = False
oldBeat = False

def butter_lowpass_filter(data, cutoff, fs, order):
    normal_cutoff = cutoff / (fs/2)
    # Get the filter coefficients 
    b, a = butter(order, normal_cutoff, btype='low', analog=False)
    y = filtfilt(b, a, data)
    return y


def bins_to_struct(bins, beat):
    bin_return = {"min": config.MIN_FREQUENCY, "max": config.MAX_FREQUENCY, "bins": [], "beat": beat}
    for bin in bins:
        bin_return["bins"].append(bin)
    return bin_return

def microphone_update(audio_samples):
    global y_roll, prev_rms, prev_exp, prev_fps_update, count, oldBeat, bpm_roll, energy_roll, newBeat, oldBeats, oldBeat, energy_data
    # Normalize samples between 0 and 1
    y = audio_samples / 2.0**15
    # Construct a rolling window of audio samples
    y_roll[:-1] = y_roll[1:]
    y_roll[-1, :] = np.copy(y)
    y_data = np.concatenate(y_roll, axis=0).astype(np.float32)
    
    vol = np.max(np.abs(y_data))
    if vol < config.MIN_VOLUME_THRESHOLD:
        print('No audio input. Volume below threshold. Volume:', vol)

    else:
        
        # Transform audio input into the frequency domain
        N = len(y_data)
        N_zeros = 2**int(np.ceil(np.log2(N))) - N
        # Pad with zeros until the next power of two
        y_data *= fft_window
        y_padded = np.pad(y_data, (0, N_zeros), mode='constant')
        YS = np.abs(np.fft.rfft(y_padded)[:N // 2])
        
        # Construct a Mel filterbank from the FFT data
        mel = np.atleast_2d(YS).T * dsp.mel_y.T
        # Scale data to values more suitable for visualization
        # mel = np.sum(mel, axis=0)
        mel = np.sum(mel, axis=0)
        mel = mel**2.0
        # Gain normalization
        mel_gain.update(np.max(gaussian_filter1d(mel, sigma=1.0)))
        mel /= mel_gain.value
        mel = mel_smoothing.update(mel)
        # Map filterbank output onto LED strip



        if config.USE_GUI:
            # Plot filterbank output
            x = np.linspace(config.MIN_FREQUENCY, config.MAX_FREQUENCY, len(mel))
            mel_curve.setData(x=x, y=fft_plot_filter.update(mel))


            x = np.linspace(0, len(y_data), len(y_data))
            sample_bar_graph.setOpts(x=x, height=y_data*80)


            y_data = audio_samples
            # filter audio samples
            y_data = butter_lowpass_filter(y_data, 12000, config.MIC_RATE, 5)

            YS = np.abs(np.fft.rfft(y_data, norm="forward"))* 100

            # take first half of YS


            x = np.linspace(0, len(YS), len(YS))
            freqs = np.fft.rfftfreq(len(y_data))
            YS_clone = YS
            # normalize
            YS_clone /= np.max(YS_clone)
            fft_bar_graph.setOpts(x=x, height=YS_clone)
            freqs = freqs*config.MIC_RATE
            
            freqs = freqs[:len(YS)//2]
            YS = YS[:len(YS)//2]
            # get the predominant frequency
            max_freq = np.argmax(YS)

            dom_freq_height = [0]*len(freqs//2)
            dom_freq_height[max_freq] = 1
            max_freq = freqs[max_freq]

            x = np.linspace(0, int(config.MIC_RATE/4), len(freqs))
            dom_freq.setOpts(x=x, height=dom_freq_height)




            freqs = [freqs[0], freqs[-1]]
            x = [0, len(YS)]
            # print(freqs)
            ticks = [list(zip(x, [str(a) for a in freqs]))]
            # only keep first and last 10 frequencies
            # ticks = ticks[:1] + ticks[-1:]
            
            # print(ticks)
            fft_plot.getAxis('bottom').setTicks(ticks)

            
            # create comb filter for the dominant frequency
            fs = config.MIC_RATE
            fc = float(max_freq)
            # round fc to nearest int
            fc = int(round(fc))


            
            # print(fc)
            Q = 30
            energy  = 0
            if(fc != 0):
                divisors = [i for i in range(1,fs+1) if fs % (i) == 0]
                # find divisor that fc is closest to
                closest_divisor = min(divisors, key=lambda x:abs(x-fc))

                fc = closest_divisor
                print("fs = ", fs, "fc = ", fc, "Q = ", Q)
                b, a = iircomb(fc, Q, ftype='peak', fs=fs)
                y = filtfilt(b, a, y_data, padlen = 500)
            
                # calculate energy
                energy = np.sum(y**2)
                
                # plot energy
            x = np.linspace(0, len(energy_data), len(energy_data))

            energy_data[:-1] = energy_data[1:]
            energy_data[-1] = energy

            energy_curve.setData(x=x, y=energy_data)









            # energy_curve.setData(x=x, y=energy_roll)

            # historical_energy_avg_curve.setData(x=x, y=np.ones(len(energy_roll)) * historical_energy_avg)

            # threshold_curve.setData(x=x, y=np.ones(len(energy_roll)) * historical_energy_avg * config.BEAT_THRESHOLD)


    if config.USE_GUI:
        app.processEvents()
    
    y = np.copy(mel)
    gain.update(y)
    y /= gain.value


    return bins_to_struct(y, newBeat)





def start_stream(callback):
    p = pyaudio.PyAudio()
    frames_per_buffer = int(44100 / 60)
    stream = p.open(format=pyaudio.paInt16,
                    channels=1,
                    rate=44100,
                    input=True,
                    frames_per_buffer=frames_per_buffer)
    overflows = 0
    prev_ovf_time = time.time()
    while True:
        try:
            y = np.frombuffer(stream.read(frames_per_buffer, exception_on_overflow=False), dtype=np.int16)
            y = y.astype(np.float32)
            stream.read(stream.get_read_available(), exception_on_overflow=False)
            callback(y)
        except IOError:
            overflows += 1
            if time.time() > prev_ovf_time + 1:
                prev_ovf_time = time.time()
                print('Audio buffer has overflowed {} times'.format(overflows))
    stream.stop_stream()
    stream.close()
    p.terminate()


def start_fft():
    

    start_stream(microphone_update)
    


def start_fft_thread():
    thread = threading.Thread(target=start_fft)
    thread.start()

if __name__ == '__main__':
    print("Start!")
    if config.USE_GUI:
        import pyqtgraph as pg
        from pyqtgraph.Qt import QtGui, QtCore
        # Create GUI window
        app = QtGui.QApplication([])
        view = pg.GraphicsView()
        layout = pg.GraphicsLayout(border=(100,100,100))
        view.setCentralItem(layout)
        view.show()
        view.setWindowTitle('Visualization')
        view.resize(800,600)
        # Mel filterbank plot
        fft_plot = layout.addPlot(title='Filterbank Output', colspan=3)
        fft_plot.setRange(yRange=[-0.1, 1.2])
        fft_plot.disableAutoRange(axis=pg.ViewBox.YAxis)
        x_data = np.array(range(1, config.N_FFT_BINS + 1))
        mel_curve = pg.PlotCurveItem()
        mel_curve.setData(x=x_data, y=x_data*0)
        fft_plot.addItem(mel_curve)

        # Visualization plot
        layout.nextRow()
        sample_plot = layout.addPlot(title='Raw Samples', colspan=3)
        x_data = np.array(range(1, int(config.MIC_RATE/config.FPS+1)))
        sample_bar_graph = pg.BarGraphItem(x=x_data, height=x_data*5, width=0.5, brush='r')
        sample_plot.disableAutoRange(axis=pg.ViewBox.YAxis)
        sample_plot.setRange(yRange=[-1.2, 1.2])
        sample_plot.addItem(sample_bar_graph)



        # Visualization plot
        layout.nextRow()
        fft_plot = layout.addPlot(title='FFT (raw)', colspan=3)
        x_data = np.array(range(1, int(config.MIC_RATE/config.FPS+1)))
        fft_bar_graph = pg.BarGraphItem(x=x_data, height=x_data*5, width=0.5, brush='r')
        fft_plot.disableAutoRange(axis=pg.ViewBox.YAxis)
        fft_plot.setRange(yRange=[-0.1, 1.2])
        fft_plot.addItem(fft_bar_graph)
        # Visualization plot
        layout.nextRow()
        dom_freq_plot = layout.addPlot(title='Dom. Freq', colspan=3)
        x_data = np.array(range(1, int(config.MIC_RATE/config.FPS+1)))
        dom_freq = pg.BarGraphItem(x=x_data, height=x_data*5, width=0.5, brush='r')
        dom_freq_plot.disableAutoRange(axis=pg.ViewBox.YAxis)
        dom_freq_plot.setRange(yRange=[0, 1])
        dom_freq_plot.addItem(dom_freq)

        # Visualization plot
        layout.nextRow()
        energy_plot = layout.addPlot(title='Energy', colspan=3)
        x_data = np.array(range(1, int(config.MIC_RATE/config.FPS+1)))
        energy_curve = pg.PlotCurveItem()
        energy_curve.setData(x=x_data, y=x_data*0)
        energy_plot.addItem(energy_curve)


       
        # Frequency range label
        freq_label = pg.LabelItem('')
        # Frequency slider
        def freq_slider_change(tick):
            minf = freq_slider.tickValue(0)**2.0 * (config.MIC_RATE / 2.0)
            maxf = freq_slider.tickValue(1)**2.0 * (config.MIC_RATE / 2.0)
            t = 'Frequency range: {:.0f} - {:.0f} Hz'.format(minf, maxf)
            freq_label.setText(t)
            config.MIN_FREQUENCY = minf
            config.MAX_FREQUENCY = maxf
            dsp.create_mel_bank()
        freq_slider = pg.TickSliderItem(orientation='bottom', allowAdd=False)
        freq_slider.tickMoveFinished = freq_slider_change
        freq_slider.addTick((config.MIN_FREQUENCY / (config.MIC_RATE / 2.0))**0.5)
        freq_slider.addTick((config.MAX_FREQUENCY / (config.MIC_RATE / 2.0))**0.5)
        freq_label.setText('Frequency range: {} - {} Hz'.format(
            config.MIN_FREQUENCY,
            config.MAX_FREQUENCY))
        # Effect selection
        active_color = '#16dbeb'
        inactive_color = '#FFFFFF'
        start_fft()
    # start_fft_thread()
    start_server = websockets.serve(handler, 'localhost', 3000)

    asyncio.get_event_loop().run_until_complete(start_server)
    asyncio.get_event_loop().run_forever()

