use ndarray::Array2;

use super::components::{Pixel};

mod rainbow_wheel;
mod expanding_squares;

pub struct Frame {
    pub pixels: Array2<Pixel>,
    timestamp: u128
}

pub struct ExpandingSquares {
    eo_count: u8,
    eo_size: u8,
    eo_growth: u8,
    eo_objects_pos_x: Vec<u8>,
    eo_objects_pos_y: Vec<u8>,
    eo_objects_expand: Vec<u8>,
    eo_objects_fade: Vec<u8>,
    eo_objects_col: Vec<Pixel>,
    current_frame: Frame
} 

pub struct RainbowWheel {
    step: u8,
    current_frame: Frame
}

enum Effect {
    RainbowWheel(RainbowWheel),
    ExpandingSquares(ExpandingSquares)
}

impl Effect {
    pub fn animate(&mut self) {
        match self {
            Effect::RainbowWheel(rainbow_wheel) => rainbow_wheel::animate_rainbow(rainbow_wheel.step, &mut rainbow_wheel.current_frame),
            Effect::ExpandingSquares(_) => todo!(),
        }
    }
}

