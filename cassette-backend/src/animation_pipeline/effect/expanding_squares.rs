use rand::Rng;

use crate::animation_pipeline::pixel::Pixel;

use super::{Frame, Animate};

pub(crate) struct ExpandingSquares {
    eo_count: u8,
    eo_size: u8,
    eo_growth: u8,
    eo_objects_pos_x: Vec<u8>,
    eo_objects_pos_y: Vec<u8>,
    eo_objects_expand: Vec<u8>,
    eo_objects_fade: Vec<u8>,
    eo_objects_col: Vec<Pixel>,
} 

impl ExpandingSquares {
    pub fn new() -> Self {
        let mut squares = ExpandingSquares{
            eo_count: 5,
            eo_size: 10,
            eo_growth: 5,
            eo_objects_pos_x: vec![0; 5],
            eo_objects_pos_y: vec![0; 5],
            eo_objects_expand: vec![0; 5],
            eo_objects_fade: vec![0; 5],
            eo_objects_col: vec![Pixel::from_rgb(0, 0, 0); 5]
            };
        let mut rng = rand::thread_rng();
        for i in 0..squares.eo_count as usize{
            squares.eo_objects_pos_x[i] = rng.gen_range(0..30 as u8);
            squares.eo_objects_pos_y[i] = rng.gen_range(0..10 as u8);
            squares.eo_objects_expand[i] = rng.gen_range(0..squares.eo_growth);
            squares.eo_objects_fade[i] = 0;
            squares.eo_objects_col[i] = Pixel::from_rgb(255, 255, 255);
        }
        squares
    }
}

impl Animate for ExpandingSquares {
    fn animate(&mut self, frame: &mut Frame) {
        animate_expanding_squares(self.eo_count, self.eo_size, self.eo_growth, &mut self.eo_objects_pos_x, &mut self.eo_objects_pos_y, &mut self.eo_objects_expand, &mut self.eo_objects_fade, &mut self.eo_objects_col, frame)
    }
}

pub fn animate_expanding_squares(eo_count: u8, eo_size: u8, eo_growth: u8, eo_objects_pos_x: &mut Vec<u8>, eo_objects_pos_y: &mut Vec<u8>, eo_objects_expand: &mut Vec<u8>, eo_objects_fade: &mut Vec<u8>, eo_objects_col: &mut Vec<Pixel>, frame: &mut Frame) {
    for j in 0..eo_count as usize{
        let r = (eo_objects_col[j].r as f32 * (1f32 - eo_objects_fade[j] as f32 / eo_growth as f32)) as u8;
        let g = (eo_objects_col[j].g as f32 * (1f32 - eo_objects_fade[j] as f32 / eo_growth as f32)) as u8;
        let b = (eo_objects_col[j].b as f32 * (1f32 - eo_objects_fade[j] as f32 / eo_growth as f32)) as u8;
        let pixel = Pixel{ r, g, b};
        let diameter = eo_size + eo_objects_expand[j];
        let height = frame.pixels.shape()[1];
        let width  = frame.pixels.shape()[0];

        if (eo_objects_expand[j] < eo_growth) {
            let offset = diameter / 2;
            frame.drawRect((eo_objects_pos_x[j] as i32-offset as i32), (eo_objects_pos_y[j] as i32 - offset as i32), diameter as i32, diameter as i32, pixel);
        }
        eo_objects_expand[j] += 2;
        eo_objects_fade[j] += 2;
        if(eo_objects_fade[j] >= eo_growth){
            eo_objects_fade[j] = eo_growth;
        }
        // get random int in range
        let mut rng = rand::thread_rng();
        if(eo_objects_expand[j] >= eo_growth && rng.gen_range(0..100 as u8) < 10){
            eo_objects_expand[j] = 0;
            eo_objects_fade[j] = 0;
            eo_objects_pos_x[j] = rng.gen_range(0..width as u8);
            eo_objects_pos_y[j] = rng.gen_range(0..height as u8);
        }
    }
}