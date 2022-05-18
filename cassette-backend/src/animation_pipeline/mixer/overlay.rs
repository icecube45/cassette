use crate::animation_pipeline::frame::Frame;

use super::Mix;
pub(crate) struct Overlay;

impl Mix for Overlay {
    fn mix(&mut self,weight:f32,channel_a: &Frame,channel_b: &Frame) -> Frame {
        let mut result = Frame::new(channel_a.width() as u32, channel_a.height() as u32);
        for i in 0..channel_a.height() {
            for j in 0..channel_a.width() {
                if(!channel_a.pixels[[i,j]].is_transparent()){
                    let pixel = channel_a.pixels[[i,j]];
                    result.pixels[[i,j]] = pixel;
                } else {
                    result.pixels[[i,j]] = channel_b.pixels[[i,j]];
                }
            }
        }
        result.update_timestamp();
        return result;
    }

}