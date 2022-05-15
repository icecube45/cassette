use crate::animation_pipeline::frame::Frame;

use super::Mix;
pub(crate) struct Linear;

impl Mix for Linear {
    fn mix(&mut self,weight:f32,channel_a: &Frame,channel_b: &Frame) -> Frame {
        todo!()
    }

}