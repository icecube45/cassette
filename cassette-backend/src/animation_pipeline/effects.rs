use ndarray::Array2;

use super::components::{Pixel, Output};

pub struct Frame {
    pub pixels: Array2<Pixel>,
    timestamp: u128
}


enum Effect {
    RainbowWheel { step: u8, current_frame: Frame },
    ExpandingSquares { step: u8, current_frame: Frame}
}

impl Effect {
    pub fn animate(&mut self) {
        match self {
            Effect::RainbowWheel { step, current_frame } => animate_rainbow(*step, current_frame),
            Effect::ExpandingSquares { step, current_frame } => todo!(),
        }
    }
}

fn animate_rainbow(mut step: u8, frame: &mut Frame) {
    let mut num_pixels_override = frame.pixels.len();
    let height = frame.pixels.shape()[1];
    let width  = frame.pixels.shape()[0];
    let matrix = height > 0;
    
    for i in 0..num_pixels_override {
        let pixel_index = (i*256/num_pixels_override) + step as usize;
        let pixel = wheel(pixel_index as u8);
        if !matrix {
            frame.pixels[[0,i]] = pixel;
        } else {
            frame.pixels[[i/width, i%width]] = pixel;
        }
    }
    if step == 255 {
        step = 0;
    } else {
        step += 1;
    }
}


fn wheel(n: u8) -> Pixel {
    let mut n = n;
    let mut r: u8 = 0;
    let mut g: u8 = 0;
    let mut b: u8 = 0;
    
    if n < 85 {
        r = n * 3;
        g = 255-n*3;
        b = 0;
    }
    else if n < 170 {
        n = n-85;
        r = 255 - n*3;
        g = 0;
        b = n*3;
    }
    else {
        n = n-170;
        r = 0;
        g = n*3;
        b = 255 - n*3;
    }
    return Pixel {r, g, b};
}       