use crate::animation_pipeline::frame::Frame;

use super::Mix;

pub struct Progressive;

impl Mix for Progressive {
    fn mix(&mut self, weight:f32, channel_a: &Frame, channel_b: &Frame) -> Frame {
        todo!()
    }
}

