mod progressive;
mod linear;
mod shape;
mod intensity;
mod overlay;
mod border;

use std::sync::{Arc, Mutex};

use super::components::Frame;

use self::progressive::Progressive;
use self::linear::Linear;
use self::shape::Shape;
use self::intensity::Intensity;
use self::overlay::Overlay;
use self::border::Border;


pub struct MixerComponent {
    pub(crate) name:          String,
    pub(crate) mixer_type:    MixMode,
    pub(crate) weight:        f32,
    pub(crate) channel_a:     Arc<Mutex<Frame>>,
    pub(crate) channel_b:     Arc<Mutex<Frame>>,
    pub(crate) output:        Arc<Mutex<Frame>>,
}

impl MixerComponent {
    // The reason it's implemented this way is to add some checks later for frame of given size...
    pub fn new(name: String, mixer_type: MixMode, weight: Option<f32>, channel_a: Option<Arc<Mutex<Frame>>>, channel_b: Option<Arc<Mutex<Frame>>>, output: Option<Arc<Mutex<Frame>>>) -> Result<Self, &'static str> {
        Ok(MixerComponent {
            name,
            mixer_type,
            weight: match weight {
                Some(w) => w,
                None => 0.0,
            },
            channel_a: match channel_a {
                Some(c) => c,
                None => todo!(),
            },
            channel_b: match channel_b {
                Some(c) => c,
                None => todo!(),
            },
            output: match output {
                Some(c) => c,
                None => todo!(),
            },
        })
    }
    pub fn mix(&self) {
        match self.output.lock() {
            Ok(output) => *output = self.mixer_type.mix(self.weight, &self.channel_a.lock().unwrap(), &self.channel_b.lock().unwrap()),
            Err(_) => todo!(),
        }
    }
}

#[enum_dispatch]

pub trait Mix {
    fn mix(&mut self, weight: f32, channel_a: &Frame, channel_b: &Frame) -> Frame;
}

#[enum_dispatch(Mix)]
enum MixMode {
    Progressive,
    Linear,
    Shape,
    Intensity,
    Overlay,
    Border
}
