use ndarray::Array2;

use super::components::{Pixel};


pub mod rainbow_wheel;
pub mod expanding_squares;
pub struct Frame {
    pub pixels: Array2<Pixel>,
    timestamp: u128
}

impl Frame {
    pub fn new(width: u32, height: u32) -> Frame {
        Frame{
            pixels: Array2::from_elem((height as usize, width as usize), Pixel::black()),
            timestamp: 0,
        }
    }
    pub fn set_pixel(&mut self, x: u32, y: u32, pixel: Pixel) {
        self.pixels[[y as usize, x as usize]] = pixel;
    }
    pub fn update_timestamp(&mut self) {
        self.timestamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis();
    }
    pub fn timestamp(&self) -> u128 {
        self.timestamp
    }
    pub fn width(&self) -> usize {
        self.pixels.shape()[1] as usize
    }
    pub fn height(&self) -> usize {
        self.pixels.shape()[0] as usize
    }
    pub fn draw_rect(&mut self, x: i32, y: i32, width: i32, height: i32, color: Pixel) {
        for j in y..(y + height) {
            for i in x..(x + width) {
                if(i<0 || j<0 || i>=self.width() as i32 - 1 || j>=self.height() as i32 - 1) {
                    continue;
                }
                self.pixels[[j as usize, i as usize]] = color;
            }
        }
    }
}





enum Effect {
    RainbowWheel(rainbow_wheel::RainbowWheel),
    ExpandingSquares(expanding_squares::ExpandingSquares),
}

impl Effect {
    pub fn animate(&mut self) {
        match self {
            Effect::RainbowWheel(rainbow_wheel) => rainbow_wheel::animate_rainbow(rainbow_wheel.step, &mut rainbow_wheel.current_frame),
            Effect::ExpandingSquares(expanding_squares) => expanding_squares::animate_expanding_squares(expanding_squares.eo_count, expanding_squares.eo_size, expanding_squares.eo_growth, &mut expanding_squares.eo_objects_pos_x, &mut expanding_squares.eo_objects_pos_y, &mut expanding_squares.eo_objects_expand, &mut expanding_squares.eo_objects_fade, &mut expanding_squares.eo_objects_col, &mut expanding_squares.current_frame)
        }
    }
}

