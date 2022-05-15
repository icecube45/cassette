use crate::animation_pipeline::frame::Frame;

use super::Mix;
pub(crate) struct Overlay;

impl Mix for Overlay {
    fn mix(&mut self,weight:f32,channel_a: &Frame,channel_b: &Frame) -> Frame {
        todo!()
    }

}