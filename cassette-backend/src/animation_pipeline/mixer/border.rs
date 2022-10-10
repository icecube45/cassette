use crate::animation_pipeline::frame::Frame;
use crate::animation_pipeline::pixel::Pixel;
use super::Mix;

pub struct Border;

impl Mix for Border {
    fn mix(&mut self, weight:f32, channel_a: &Frame, channel_b: &Frame) -> Frame {
        let mut result = Frame::new(channel_a.width() as u32, channel_a.height() as u32);
        for i in 0..channel_a.height() {
            for j in 0..channel_a.width() {
                if !channel_a.pixels[[i,j]].is_transparent() {
                    let pixel = channel_a.pixels[[i,j]];
                    result.pixels[[i,j]] = pixel;
                } else {
                    // check surrounding pixels to see if they are not transparent
                    let mut border = false;
                    if i > 0 {
                        if !channel_a.pixels[[i-1,j]].is_transparent() {
                            border = true;
                        } else if i<channel_a.height()-2 {
                            if !channel_a.pixels[[i+1,j]].is_transparent() {
                                border = true;
                            }
                        }
                    }
                    if j > 0 {
                        if !channel_a.pixels[[i,j-1]].is_transparent() {
                            border = true;
                        } else if j<channel_a.width()-2 {
                            if !channel_a.pixels[[i,j+1]].is_transparent() {
                                border = true;
                            }
                        }
                    }
                    if border {
                        result.pixels[[i,j]] = Pixel::black();
                    } else {
                        result.pixels[[i,j]] = channel_b.pixels[[i,j]];
                    }
                }
            }
        }
        result.update_timestamp();
        return result;
    }
}