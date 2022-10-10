use crate::animation_pipeline::frame::Frame;
use crate::animation_pipeline::pixel::Pixel;
use super::Mix;
pub struct Linear;

impl Mix for Linear {
    fn mix(&mut self, weight:f32, channel_a: &Frame, channel_b: &Frame) -> Frame {
        let mut result = Frame::new(channel_a.width() as u32, channel_a.height() as u32);
        for i in 0..channel_a.height()as usize {
            for j in 0..channel_a.width()as usize {
                let first_pixel = channel_a.pixels[[i,j]];
                let second_pixel = channel_b.pixels[[i,j]];
                let r = (first_pixel.r as f32 * (weight as f32 / 100.0)) + (second_pixel.r as f32 * (1.0 - (weight as f32 / 100.0)));
                let g = (first_pixel.g as f32 * (weight as f32 / 100.0)) + (second_pixel.g as f32 * (1.0 - (weight as f32 / 100.0)));
                let b = (first_pixel.b as f32 * (weight as f32 / 100.0)) + (second_pixel.b as f32 * (1.0 - (weight as f32 / 100.0)));
                let r = r as u8;
                let g = g as u8;
                let b = b as u8;
                let pixel = Pixel{r, g, b};
                result.pixels[[i,j]] = pixel;
            }
        }
        result.update_timestamp();
        return result;
    }

}