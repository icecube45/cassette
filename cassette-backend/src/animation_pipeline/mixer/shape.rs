use crate::animation_pipeline::components::Frame;

use super::Mix;
pub(crate) struct Shape;

impl Mix for Shape {
    fn mix(&mut self, weight:f32, channel_a: &Frame, channel_b: &Frame) -> Frame {
        todo!()
    }

}