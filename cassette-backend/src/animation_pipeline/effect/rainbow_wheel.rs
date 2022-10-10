use crate::animation_pipeline::pixel::Pixel;

use super::{Frame, Animate};
#[derive(Debug)]
pub struct RainbowWheel {
    step: u8,
}

impl RainbowWheel {
    pub fn new() -> Self {
        RainbowWheel {
            step: 0,
        }
    }
}

impl Animate for RainbowWheel {
    fn animate(&mut self, frame: &mut Frame) {
        animate_rainbow(self.step, frame)
    }
}

pub fn animate_rainbow(mut step: u8, frame: &mut Frame) {
    // TODO: de-test this. stuff like num_override and matrix bool were used in my test python script but aren't needed here
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