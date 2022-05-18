use crate::animation_pipeline::frame::Frame;
use crate::animation_pipeline::pixel::Pixel;
use super::Mix;

pub struct Progressive;

impl Mix for Progressive {
    fn mix(&mut self, weight:f32, channel_a: &Frame, channel_b: &Frame) -> Frame {
         //crossfade that mixes each frame at 100% when the weight is at 50
         let mut result = Frame::new(channel_a.width() as u32, channel_a.height() as u32);
         for j in 0..channel_a.height() {
             for i in 0..channel_a.width() {
                 let first_pixel = channel_a.pixels[[j, i]];
                 let second_pixel = channel_b.pixels[[j, i]];
                 let mut first_mix_weight = 0.0;
                 let mut second_mix_weight = 0.0;
                 if weight < 0.5 {
                     first_mix_weight =  1.0;
                     second_mix_weight = weight/100.0 * 2.0;
                 } else {
                     first_mix_weight = (1.0 - weight / 100.0) * 2.0;
                     second_mix_weight = 1.0;
                 }
                 let mut r = first_pixel.r as f32 * first_mix_weight + second_pixel.r as f32 * second_mix_weight;
                 let mut g = first_pixel.g as f32 * first_mix_weight + second_pixel.g as f32 * second_mix_weight;
                 let mut b = first_pixel.b as f32 * first_mix_weight + second_pixel.b as f32 * second_mix_weight;
                 if r > 255.0 {
                     r = 255.0;
                 }
                 if g > 255.0 {
                     g = 255.0;
                 }
                 if b > 255.0 {
                     b = 255.0;
                 }
                 let r = r as u8;
                 let g = g as u8;
                 let b = b as u8;
                 let pixel = Pixel{r, g, b};
                 result.pixels[[j,i]] = pixel;
             }
         }
         result.update_timestamp();
         return result;
    }
}

