use std::sync::{Arc};


use axum::extract::ws::{WebSocket, Message};
use cpal::{StreamConfig};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

use hecs::{World, Entity};
use ndarray::{ArrayBase, Data, Dimension, Array, Axis, Slice, s, Array1, Array2};
use ndarray_ndimage::gaussian_filter1d;
use ndarray_stats::QuantileExt;
use parking_lot::{Mutex, RwLock};
use realfft::{RealFftPlanner};
use rustfft::num_complex::Complex;
use rustfft::num_traits::Zero;
use tokio::runtime::Runtime;
use aubio::{Onset, Tempo};
use tokio::sync::mpsc::{Sender};

use crate::mel_filter;




const ROLLING_HISTORY_COUNT: usize = 2;
const FPS: usize = 60;
const NUM_FFT_BINS: usize = 30;
const MIN_FREQ: f64 = 200.0;
const MAX_FREQ: f64 = 12000.0;
const VOLUME_THRESHOLD: f64 = 1e-7;

pub trait TempoCallback {
    fn on_tempo(&mut self);
}
pub struct DSPWrapper {
    pub dsp: Arc<Mutex<DSP>>,
    pub stream: cpal::Stream
}

impl DSPWrapper {
    pub fn new(world: Arc<RwLock<World>>) -> DSPWrapper {
        let ret = DSP::new(world);
        DSPWrapper { dsp: ret.0, stream: ret.1 }
    }
}
pub struct DSP{
    fft_planner: RealFftPlanner::<f64>,
    buffer_size: usize,
    sample_buffer: Vec<f32>,
    rolling_sample_window: Vec<f64>,
    pub spectrum: Vec<Complex<f64>>,
    world: Arc<RwLock<World>>,
    mel_gain: ExpFilter,
    mel_smoothing: ExpFilter,
    sample_rate: u32,
    samples_per_frame: usize,
    // dispatch_count: usize,
    // first_time: Instant,
    hamming_window: Array1<f64>,
    transposed_melmat: Array2<f64>,
    pub mel_spectrum: Array1<f64>,
    pub num_fft_bins: usize,
    onset: Arc<Mutex<Onset>>,
    tempo: Arc<Mutex<Tempo>>,
    tempo_callback_channels: Vec<Sender<bool>>,
}

unsafe impl Send for DSP {}

impl DSP{
    pub fn new(world: Arc<RwLock<World>>) -> (Arc<Mutex<DSP>>, cpal::Stream) {
        
        let host = cpal::default_host();
        let output_device = host.default_output_device().unwrap();
        let mut default_config: StreamConfig = output_device.default_output_config().expect("Failed to get default output config").into();

        // for each supported config
        // for config in input_device.supported_input_configs().expect("Failed to get supported input configs") {
        //     // println!("{:?}", config);
        // }

        println!("Running with Audio Device: {:?}", output_device.name());
        println!("Running with Configuration: {:?}", default_config);
        // let mut real_planner = RealFftPlanner::<f64>::new();

        let mut samples_per_frame = (default_config.sample_rate.0 as f32 / FPS as f32) as usize;

        // find the power of two that is closes to the number of samples
        let mut power_of_two_sample_size = 1;
        while power_of_two_sample_size < samples_per_frame {
            power_of_two_sample_size *= 2;
        }
        power_of_two_sample_size;



        let mut window = ndarray::Array::linspace(0.0, 0.0, samples_per_frame*ROLLING_HISTORY_COUNT);
        for i in 0..window.len() {
            window[[i]] = 0.54 - 0.46 * (2.0 * std::f64::consts::PI * i as f64 / window.len() as f64).cos();
        }
        let num_fft_bins = samples_per_frame*ROLLING_HISTORY_COUNT/2;
        let melmat = mel_filter::compute_melmat(NUM_FFT_BINS as u32, MIN_FREQ, MAX_FREQ, num_fft_bins as u32, default_config.sample_rate.0).0;
        let transposed_melmat = melmat.reversed_axes();

        

        
        let mut onset = Onset::new(aubio::OnsetMode::SpecDiff, 2048, 800, default_config.sample_rate.0 as u32).expect("Failed to create onset");
        onset.set_awhitening(false);
        onset.set_threshold(0.1);
        onset.set_minioi_ms(0.1);
        let tempo = Tempo::new(aubio::OnsetMode::Mkl, 2048, 800, default_config.sample_rate.0 as u32).expect("Failed to create tempo");
        // create ARCs from the above
        let onset_arc = Arc::new(Mutex::new(onset));
        let tempo_arc = Arc::new(Mutex::new(tempo));


        let dsp = DSP{
            fft_planner: RealFftPlanner::<f64>::new(),
            // fft: Arc::new(RealFftPlanner::new().plan_r2c(buffer_size)),
            buffer_size: 0,
            rolling_sample_window: vec![0.0; samples_per_frame*ROLLING_HISTORY_COUNT],
            spectrum: vec![Complex::zero(); 0],
            world,
            mel_gain: ExpFilter::new(0.99, 0.01, samples_per_frame),
            mel_smoothing: ExpFilter::new(0.99, 0.5, samples_per_frame),
            sample_rate: default_config.sample_rate.0 as u32,
            samples_per_frame: samples_per_frame,
            sample_buffer: vec![0.0; 0],
            // dispatch_count: 0,
            // first_time: Instant::now(),
            hamming_window: window,
            transposed_melmat: transposed_melmat,
            mel_spectrum: Array::zeros([num_fft_bins]),
            num_fft_bins: num_fft_bins,
            onset: onset_arc.clone(),
            tempo: tempo_arc.clone(),
            tempo_callback_channels: vec![],
           
        };

        let dsp_arc: Arc<Mutex<DSP>> = Arc::new(Mutex::new(dsp));

        let send_to_callback = dsp_arc.clone();

        let input_stream = output_device.build_input_stream(
            &default_config, 
            {
                move |data, _: &_| DSP::input_data_fn(send_to_callback.clone(), data)
            },
            DSP::err_fn)
            .unwrap();

        
        
        input_stream.play().unwrap();
        // println!("Created new DSP");
        return (dsp_arc, input_stream);
    }

    pub fn add_tempo_channel(&mut self, channel: Sender<bool>) {
        self.tempo_callback_channels.push(channel);
    }


    fn err_fn(err: cpal::StreamError) {
        eprintln!("an error occurred on stream: {}", err);
    }
    
    fn input_data_fn(dsp: Arc<Mutex<DSP>>, data: &[f32]) {
        let mut dsp = dsp.lock();
        dsp.buffer_and_dispatch_samples(data);
     
        // println!("DSP");
        // println!("{:?}", data);
        // print length of data
        // println!("{:?}", data.len());
    }



    pub fn buffer_and_dispatch_samples(&mut self, data: &[f32]) {
        // add data to buffer
        self.sample_buffer.extend_from_slice(data);

        // dispatch samples
        while self.sample_buffer.len() >= self.samples_per_frame {
            let samples = self.sample_buffer.drain(..self.samples_per_frame).collect::<Vec<f32>>();
            self.process_data(&samples);
        }
    }







    pub fn process_data(&mut self, data: &[f32]) {
        // if self.dispatch_count % FPS == 0 {
        //     // calculate dispatches per second
        //     let elapsed = self.first_time.elapsed();
        //     let elapsed_ms = elapsed.as_secs() * 1000 + elapsed.subsec_millis() as u64;
        //     let dispatches_per_second = (self.dispatch_count as f32 / (elapsed_ms as f32 / 1000.0)) as u32;
        //     println!("{} dispatches per second", dispatches_per_second);
        //     self.first_time = Instant::now();
        //     self.dispatch_count = 0;
        // }
        // self.dispatch_count += 1;

        // lock onset and tempo
        let mut onset = self.onset.lock();
        let mut tempo = self.tempo.lock();

        // get maximum sample value
        let max_sample = data.iter().fold(0.0, |acc, &x| if x > acc { x } else { acc });
        if max_sample < VOLUME_THRESHOLD as f32 {
            return;
        }




        let mut in_data = vec![0.0; data.len()];

        // let mut in_data = r2c.make_input_vec();
        // copy data to in_data while also converting f32 to f64
        for (i, x) in data.iter().enumerate() {
            // normalize samples between 0 and 1
            in_data[i] = *x as f64 / 32768.0;
            // in_data[i] = *x as f64;
        }
        // add new data to rolling sample window
        self.rolling_sample_window.extend_from_slice(in_data.as_slice());
        // remove old data from rolling sample window
        if self.rolling_sample_window.len() > in_data.len()*ROLLING_HISTORY_COUNT {
            self.rolling_sample_window.drain(0..in_data.len());
        }

        
        
        
        let N = self.rolling_sample_window.len();
        let N_zeros = 2_i32.pow((N as f32).log2().ceil() as u32) as i32 - N as i32;
        let rolling_sample_copy = self.rolling_sample_window.clone();
        let mut data_sample_array = ndarray::Array::from_vec(rolling_sample_copy);
        
        
        // apply window to data
        data_sample_array = &self.hamming_window * &data_sample_array;
        
        // pad with N_zeros
        let mut padded_array = pad_with_zeros(&mut data_sample_array, vec![[0,N_zeros as usize]]);
        let aubio_data = padded_array.clone().to_vec();
        
        // create copy of rolling_sample_window that is f32
        let mut aubio_data_f32 = vec![0.0; aubio_data.len()];
        for (i, x) in aubio_data.iter().enumerate() {
            aubio_data_f32[i] = (*x * 32768.0) as f32;
        }

        // println!("len of aubio_data is {}, len of incoming samples is {}", aubio_data.len(), data.len());
        // do tempo detection
        // tempo.do_result(&aubio_data);
        // do onset detection
        // create onset vector
        let mut onset_vec = vec![0.0; aubio_data.len()];
        let result = onset.do_result(&aubio_data_f32).expect("Failed to do onset detection");
        let tempo_result = tempo.do_result(&aubio_data_f32).expect("Failed to do tempo detection");

        // println!("{}", onset_vec[0]);
        // println!("{}", onset.get_threshold());
        // onset.set_threshold(0.3);
        // println!("{:?}", result);

        let beat = result > 1.0;

        if beat {
            // call each tempo callback
            for channel in self.tempo_callback_channels.iter() {
                if channel.capacity()!=0 {
                    channel.blocking_send(true);
                }
            }
        }
        // let mut beat = false;
        // // println!("{:?}", tempo.get_last());
        // if(onset.get_last() != self.last_beat){
        //     self.last_beat = onset.get_last();
        //     beat = true;
        //     // println!("----------X");
        // }
        // else{
        //     // println!("-");
        // }
        

        // let beat = onset_vec[0] >= 1.2;
        // println!("{:?}", beat);
        // let beat = false;
        // println!("{:?}", onset.get_onset());


        // perform fft
        let r2c = self.fft_planner.plan_fft_forward(padded_array.len());
        let mut spectrum = r2c.make_output_vec();
        // convert padded_array to &mut [f64]
        let mut padded_array_mutf64 = padded_array.as_slice_mut().unwrap();


        r2c.process(&mut padded_array_mutf64, &mut spectrum).expect("Failed to process fft");

        // convert spectrum to ndarray because it's easier to work with
        let spectrum_ndarray_full = ndarray::Array::from_vec(spectrum);
        // create a new spectrum ndarray with only the first half of the spectrum
        let spectrum_ndarray = spectrum_ndarray_full.view().split_at(Axis(0), N/2).0;

        // normalize spectrum
        let spectrum_ndarray_norm = spectrum_ndarray.mapv(|x| x.norm());

        
        let twod_spectrum_ndarray_norm = spectrum_ndarray_norm.into_shape([1, spectrum_ndarray.len()]).unwrap();
        // println!("{:?}", twod_spectrum_ndarray_norm);
        let transposed_twod_spectrum = twod_spectrum_ndarray_norm.reversed_axes();

        let mel = &transposed_twod_spectrum * &self.transposed_melmat;

        // mel = np.sum(mel, axis=0)
        // mel = mel**2.0

        let mut mel = mel.sum_axis(Axis(0));
        mel = mel.mapv(|x| x.powf(2.0));

        // get max value of array

        self.mel_gain.update_with_value(*gaussian_filter1d(&mel, 1.0, 4.0, Axis(0)).max().expect("Failed to get max value of mel"));

        for i in 0..mel.len() {
            mel[[i]] = mel[[i]] / self.mel_gain.get(i);
        }
        
        
        let mel = self.mel_smoothing.update_with_array(&mel);
         
        // only keep 0..NUM_FFT_BINS of mel spectrum
        self.mel_spectrum = mel.slice_move(s![..NUM_FFT_BINS]);


        // build a json string from spectrum values
        let mut json_string = String::new();
        json_string.push_str("{\"min\":");
        json_string.push_str(&format!("{}", MIN_FREQ));
        json_string.push_str(",\"max\":");
        json_string.push_str(&format!("{}", MAX_FREQ));

        if beat {
            json_string.push_str(",\"beat\":true");
        } else {
            json_string.push_str(",\"beat\":false");
        }

        json_string.push_str(",\"bins\":[");

        // let bin_count = 64;
        // let mut bin_size = self.spectrum.len()/2/bin_count; // we only use the first half of the since it's FFT - so mirrored
        // for i in 0..bin_count {
        //     let mut bin_sum = 0.0;
        //     for j in 0..bin_size {
        //         bin_sum += self.spectrum[i*bin_size + j].norm()/50.0;
        //     }
        //     bin_sum /= bin_size as f64;
        //     json_string.push_str(&format!("{},", bin_sum));
        // }

        for i in 0..NUM_FFT_BINS {
            json_string.push_str(&format!("{},", self.mel_spectrum[[i]]));
        }


        // for i in self.spectrum.len()/2..self.spectrum.len() {
        //     json_string.push_str(&format!("{},", self.spectrum[i].re*0.05));
        // }
        json_string.pop();
        json_string.push_str("]}");
        // println!("{}", json_string);
        let world = self.world.read();
        let mut query = world.query::<&Arc<Mutex<WebSocket>>>();

        // this is hacky but ok...

        let mut websockets: Vec<(Entity, Arc<Mutex<WebSocket>>)> = Vec::new();
        query.iter().for_each(|(entity, socket)| {
            websockets.push((entity.clone(), socket.clone()));
        });
        drop(query);
        drop(world);
        
        
        for (entity, socket) in websockets {
            let rt = Runtime::new().unwrap();
            rt.block_on(async {
                if socket.lock()
                    .send(Message::Text(String::from(json_string.clone())))
                    // .send(Message::Text("hello".to_string()))
                    .await
                    .is_err() 
                {
                    println!("Failed to send message");
                    let mut world = self.world.write();
                    if world.despawn(entity).is_err() {
                        println!("Error despawning entity");
                    }
                }
            });
        }

    }
}


fn pad_with_zeros<A, S, D>(arr: &ArrayBase<S, D>, pad_width: Vec<[usize; 2]>) -> Array<A, D>
where
    A: Clone + Zero,
    S: Data<Elem = A>,
    D: Dimension,
{
    assert_eq!(
        arr.ndim(),
        pad_width.len(),
        "Array ndim must match length of `pad_width`."
    );

    // Compute shape of final padded array.
    let mut padded_shape = arr.raw_dim();
    for (ax, (&ax_len, &[pad_lo, pad_hi])) in arr.shape().iter().zip(&pad_width).enumerate() {
        padded_shape[ax] = ax_len + pad_lo + pad_hi;
    }

    let mut padded = Array::zeros(padded_shape);
    {
        // Select portion of padded array that needs to be copied from the
        // original array.
        let mut orig_portion = padded.view_mut();
        for (ax, &[pad_lo, pad_hi]) in pad_width.iter().enumerate() {
            // FIXME: This has a bug when `pad_hi` is 0. See @fzyzcjy's comment below.
            orig_portion
                .slice_axis_inplace(Axis(ax), Slice::from(pad_lo as isize..-(pad_hi as isize)));
        }
        // Copy the data from the original array.
        orig_portion.assign(arr);
    }
    padded
}



// exponential smoothing filter
#[derive(Debug)]
pub struct ExpFilter{
    alpha_rise: f64,
    alpha_decay: f64,
    value: Array1<f64>,
}

impl ExpFilter{
    pub fn new(alpha_rise: f64, alpha_decay: f64, length: usize) -> ExpFilter{
        ExpFilter{
            alpha_rise,
            alpha_decay,
            value: Array1::from_elem(length, 0.00000001),
        }
    }

    pub fn update_with_array(&mut self, new_values: &Array1<f64>) -> Array1<f64>{
        for i in 0..new_values.len(){
            if new_values[i] - self.value[i] > 0.0 {
                self.value[i] = self.alpha_rise * new_values[i] + (1.0 - self.alpha_rise) * self.value[i];
            }
            else{
                self.value[i] = self.alpha_decay * new_values[i] + (1.0 - self.alpha_decay) * self.value[i];
            }
        }
        return self.value.clone();
    }

    pub fn update_with_value(&mut self, new_value: f64) -> Array1<f64>{
        for i in 0..self.value.len(){
            if new_value - self.value[i] > 0.0 {
                self.value[i] = self.alpha_rise * new_value + (1.0 - self.alpha_rise) * self.value[i];
            }
            else{
                self.value[i] = self.alpha_decay * new_value + (1.0 - self.alpha_decay) * self.value[i];
            }
        }
        return self.value.clone();
    }

    pub fn get_values(&self) -> Array1<f64>{
        self.value.clone()
    }

    pub fn get(&self, index: usize) -> f64{
        self.value[index]
    }
}