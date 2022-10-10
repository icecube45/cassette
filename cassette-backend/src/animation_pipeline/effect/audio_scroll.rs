use crate::{animation_pipeline::pixel::Pixel, dsp::{DSP, ExpFilter}};

use super::{Frame, Animate};
use ndarray::{Axis, s, Array2};
use ndarray_ndimage::gaussian_filter1d;
use parking_lot::{Mutex};
use std::{sync::{Arc}};
use ndarray_stats::QuantileExt;


// an implementation of visualize_scroll from https://github.com/scottlawsonbc/audio-reactive-led-strip/blob/master/python/visualization.py#L105=

pub struct AudioScroll {
    dsp: Arc<Mutex<DSP>>,
    gain_filter: ExpFilter,
    p: Array2<f64>,
}

impl AudioScroll {
    pub fn new(dsp: Arc<Mutex<DSP>>) -> Self {
        let dsp_clone = dsp.clone();
        let mut num_fft_bins = 0;
        {
            let dsp_locked = dsp_clone.lock();
            num_fft_bins = dsp_locked.num_fft_bins;
        }
        AudioScroll {
            dsp,
            gain_filter: ExpFilter::new(0.99, 0.001, num_fft_bins),
            p: Array2::zeros((3, 50)),
        }
    }
        
    pub fn animate_scroll(&mut self, frame: &mut Frame) {
        let mut dsp = self.dsp.lock();
        
        let mut squared_mel = dsp.mel_spectrum.clone();
        for i in 0..squared_mel.len() {
            squared_mel[i] = squared_mel[i] * squared_mel[i];
        }

        let gain = self.gain_filter.update_with_array(&squared_mel);

        for i in 0..squared_mel.len() {
            squared_mel[i] = squared_mel[i] / gain[i];
            squared_mel[i] = squared_mel[i] * 255.0;
        }
        let first_third = squared_mel.slice(s![..(squared_mel.len() / 3)]);
        let second_third = squared_mel.slice(s![(squared_mel.len() / 3)..(squared_mel.len() / 3)*2]);
        let third_third = squared_mel.slice(s![(squared_mel.len() / 3) * 2..]);
        let r = first_third.max().expect("no max");
        let g = second_third.max().expect("no max");
        let b = third_third.max().expect("no max");
       
        
        // shift p array pixels to the right by one, dropping the last one
        for i in (1..self.p.len_of(Axis(1))).rev() {
            self.p[(0, i)] = self.p[(0, i-1)] * 0.98;
            self.p[(1, i)] = self.p[(1, i-1)] * 0.98;
            self.p[(2, i)] = self.p[(2, i-1)] * 0.98;
        }
        // wipe out first pixel
        self.p[(0, 0)] = 0.0;
        self.p[(1, 0)] = 0.0;
        self.p[(2, 0)] = 0.0;
        self.p = gaussian_filter1d(&self.p, 0.2, 4.0, Axis(1));
        // add new pixel
        self.p[(0, 0)] = *r;
        self.p[(1, 0)] = *g;
        self.p[(2, 0)] = *b;
        
        // loop over frame width, and for each pixel, set the color to the corresponding value in p
        for x in 0..frame.width()/2 {
            for y in 0..frame.height() {
                let px = Pixel::from_rgb(
                    self.p[(0, x as usize)] as u8,
                    self.p[(1, x as usize)] as u8,
                    self.p[(2, x as usize)] as u8
                );
                frame.set_pixel(x as u32 + frame.width() as u32/2, y as u32, px);
                frame.set_pixel(frame.width() as u32/2 - x as u32, y as u32, px);

            }
        }
    }
}

impl Animate for AudioScroll {
    fn animate(&mut self, frame: &mut Frame) {
        self.animate_scroll(frame)
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


