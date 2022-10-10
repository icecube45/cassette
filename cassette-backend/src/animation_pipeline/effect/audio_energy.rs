use crate::{animation_pipeline::pixel::Pixel, dsp::{DSP, ExpFilter}};

use super::{Frame, Animate};
use ndarray::{Axis, s, Array1};
use ndarray_ndimage::gaussian_filter1d;
use parking_lot::{Mutex};
use std::{sync::{Arc}};



// an implementation of visualize_energy from https://github.com/scottlawsonbc/audio-reactive-led-strip/blob/master/python/visualization.py#L127=

pub struct AudioEnergy {
    dsp: Arc<Mutex<DSP>>,
    gain_filter: ExpFilter,
    p_r_filter: ExpFilter,
    p_g_filter: ExpFilter,
    p_b_filter: ExpFilter,
    p_r: Array1<f64>,
    p_g: Array1<f64>,
    p_b: Array1<f64>,
}

impl AudioEnergy {
    pub fn new(dsp: Arc<Mutex<DSP>>) -> Self {
        let mut num_fft_bins = 0;
        {
            let dsp = dsp.lock();
            num_fft_bins = dsp.num_fft_bins;
        }
        AudioEnergy {
            dsp,
            gain_filter: ExpFilter::new(0.99, 0.001, num_fft_bins),
            p_r_filter: ExpFilter::new(0.99, 0.09, 50),
            p_g_filter: ExpFilter::new(0.99, 0.09, 50),
            p_b_filter: ExpFilter::new(0.99, 0.09, 50),
            p_r: Array1::zeros(50),
            p_g: Array1::zeros(50),
            p_b: Array1::zeros(50)
        }
    }
        
    pub fn animate_energy(&mut self, frame: &mut Frame) {
        let dsp = self.dsp.lock();
        let mut mel_copy = dsp.mel_spectrum.clone();


        let gain = self.gain_filter.update_with_array(&mel_copy);

        for i in 0..mel_copy.len() {
            mel_copy[i] = mel_copy[i] / gain[i];
            mel_copy[i] = mel_copy[i] * ((frame.width()/2) as f64 + 3.0);
            // mel_copy[i] = mel_copy[i].powf(0.9);
        }
        let first_third = mel_copy.slice(s![..(mel_copy.len() / 3)]);
        let second_third = mel_copy.slice(s![(mel_copy.len() / 3)..(mel_copy.len() / 3)*2]);
        let third_third = mel_copy.slice(s![(mel_copy.len() / 3) * 2..]);
        let r = first_third.mean().expect("no mean") as usize;
        let g = second_third.mean().expect("no mean") as usize;
        let b = third_third.mean().expect("no mean") as usize;

        for i in 0..self.p_r.len() {
            if i < r {
                self.p_r[i] = 255.0;
            }
            else{
                self.p_r[i] = 0.0;
            }
            if i < g {
                self.p_g[i] = 255.0;
            }
            else{
                self.p_g[i] = 0.0;
            }
            if i < b {
                self.p_b[i] = 255.0;
            }
            else{
                self.p_b[i] = 0.0;
            }
        }

        self.p_r_filter.update_with_array(&self.p_r);
        self.p_g_filter.update_with_array(&self.p_g);
        self.p_b_filter.update_with_array(&self.p_b);

        self.p_r = self.p_r_filter.get_values();
        self.p_g = self.p_g_filter.get_values();
        self.p_b = self.p_b_filter.get_values();
        


        self.p_r = gaussian_filter1d(&self.p_r, 4.0, 3.0, Axis(0));
        self.p_g = gaussian_filter1d(&self.p_g, 4.0, 3.0, Axis(0));
        self.p_b = gaussian_filter1d(&self.p_b, 4.0, 3.0, Axis(0));

        for x in 0..frame.width()/2 {
            for y in 0..frame.height() {
                let px = Pixel::from_rgb(
                    self.p_r[x as usize] as u8,
                    self.p_g[x as usize] as u8,
                    self.p_b[x as usize] as u8
                );
                frame.set_pixel(x as u32 + frame.width() as u32/2, y as u32, px);
                frame.set_pixel(frame.width() as u32/2 - x as u32, y as u32, px);

            }
        }
    }
}

impl Animate for AudioEnergy {
    fn animate(&mut self, frame: &mut Frame) {
        self.animate_energy(frame)
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


