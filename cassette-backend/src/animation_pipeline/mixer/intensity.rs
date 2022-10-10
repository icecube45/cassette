use crate::animation_pipeline::frame::Frame;

use super::Mix;
pub struct Intensity;

impl Mix for Intensity {
    fn mix(&mut self,weight:f32,channel_a: &Frame,channel_b: &Frame) -> Frame {
        let mut result = Frame::new(channel_a.width() as u32, channel_a.height() as u32);
        for i in 0..channel_a.height() {
            for j in 0..channel_a.width() {
                result.pixels[[i,j]] = channel_b.pixels[[i,j]]*channel_a.pixels[[i,j]].get_intensity();
            }
        }
        result.update_timestamp();
        return result;
    }
}