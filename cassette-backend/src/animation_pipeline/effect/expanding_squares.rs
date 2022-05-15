use crate::animation_pipeline::components::Pixel;

use super::{Frame, Animate};

pub struct ExpandingSquares {
    eo_count: u8,
    eo_size: u8,
    eo_growth: u8,
    eo_objects_pos_x: Vec<u8>,
    eo_objects_pos_y: Vec<u8>,
    eo_objects_expand: Vec<u8>,
    eo_objects_fade: Vec<u8>,
    eo_objects_col: Vec<Pixel>,
} 

impl Animate for ExpandingSquares {
    fn animate(&mut self, frame: &mut Frame) {
        todo!();
    }
}

fn animate_expanding_squares(eo_count: u8, eo_size: u8, eo_growth: u8, eo_objects_pos_x: &mut Vec<u8>, eo_objects_pos_y: &mut Vec<u8>, eo_objects_expand: &mut Vec<u8>, eo_objects_fade: &mut Vec<u8>, eo_objects_col: &mut Vec<Pixel>, frame: &mut Frame) {
    todo!();
}