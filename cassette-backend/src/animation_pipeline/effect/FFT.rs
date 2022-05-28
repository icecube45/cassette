use crate::{animation_pipeline::pixel::Pixel, dsp::{DSP, ExpFilter}};

use super::{Frame, Animate};
use parking_lot::{Mutex, RwLock};
use std::{sync::{Arc}, cmp::{max, min}};

pub struct FFTAnimation {
    dsp: Arc<Mutex<DSP>>,
    separated: bool,
    bin_filter: ExpFilter,
}

impl FFTAnimation {
    pub fn new(dsp: Arc<Mutex<DSP>>) -> Self {
        FFTAnimation {
            dsp,
            separated: false,
            bin_filter: ExpFilter::new(0.99, 0.1, 30),
        }
    }
        
    pub fn animate_fft(&mut self, frame: &mut Frame) {
        let mut dsp = self.dsp.lock();
        
        // check if frame width is more than or equal to fft bin count, including one pixel seperation if enabled
        let mut fft_bin_count = dsp.num_fft_bins;
        let frame_width = frame.width();
        let frame_height = frame.height();
        
        if frame_width < fft_bin_count + fft_bin_count-1*self.separated as usize {
            // if the frame width is less than the fft bin count, we need to average the fft bins to fit the frame width
            fft_bin_count = frame_width;
            if self.separated {
                fft_bin_count /=2;
                fft_bin_count -= 1;
            }
        }
        let to_subtract = (self.separated as usize) * (fft_bin_count-1);
        let adj_frame_width = frame_width - to_subtract;
        let bin_width = adj_frame_width / fft_bin_count;
        let bin_step = if self.separated {
            bin_width + 1
        } else {
            bin_width
        };
        
        let bin_height = dsp.mel_spectrum.mapv(|x| x*frame_height as f64);
        let bin_height_filtered = self.bin_filter.update_with_array(&bin_height);

        for i in 0..fft_bin_count {

            // dsp.mel_spectrum[i]
            let mut bin_height = bin_height_filtered[i] as i32;
            if(bin_height == 0) {
                bin_height = 1;
            }
            // let green = min(64_i32 + (dsp.mel_spectrum[i] * (255.0 - 64.0)) as i32, 255_i32) as u8;
            let green = 255;

            frame.draw_rect(
                i as i32 * bin_step as i32,
                frame_height as i32 - bin_height,
                bin_width as i32,
                bin_height,
                Pixel::from_rgb(
                    0,
                    green,
                    0,
                ),
            );
        }
    }
}

impl Animate for FFTAnimation {
    fn animate(&mut self, frame: &mut Frame) {
        self.animate_fft(frame)
    }
}

//     let mut num_pixels_override = frame.pixels.len();
//     let height = frame.pixels.shape()[1];
//     let width  = frame.pixels.shape()[0];
//     let matrix = height > 0;
    
//     for i in 0..num_pixels_override {
//         let pixel_index = (i*256/num_pixels_override) + step as usize;
//         let pixel = wheel(pixel_index as u8);
//         if !matrix {
//             frame.pixels[[0,i]] = pixel;
//         } else {
//             frame.pixels[[i/width, i%width]] = pixel;
//         }
//     }
//     if step == 255 {
//         step = 0;
//     } else {
//         step += 1;
//     }


