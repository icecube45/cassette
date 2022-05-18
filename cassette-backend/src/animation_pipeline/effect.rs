pub mod rainbow_wheel;
pub mod expanding_squares;

use std::sync::{Arc, Mutex};

use self::rainbow_wheel::RainbowWheel;
use self::expanding_squares::ExpandingSquares;

use super::frame::Frame;

#[enum_dispatch]

pub trait Animate {
    fn animate(&mut self, frame: &mut Frame);
}

pub struct EffectComponent {
    name: String,
    effect: Effect,
    current_frame: Arc<Mutex<Frame>>,
}

impl EffectComponent {
    pub fn new(name: String, effect: Effect) -> EffectComponent {
        EffectComponent {
            name,
            effect,
            current_frame: Arc::new(Mutex::new(Frame::new(32,32))),
        }
    }
    // is this right? no idea! but it allows for the effect to decide how it wants to animate
    pub fn animate(&mut self) {
        self.effect.animate(&mut self.current_frame.lock().unwrap());
    }
}

#[enum_dispatch(Animate)]
pub enum Effect {
    ExpandingSquares,
    RainbowWheel,
    // ...
}

impl Effect {
    pub fn new() -> Vec<Effect> {
        vec![
            Effect::ExpandingSquares(ExpandingSquares::new()),
            Effect::RainbowWheel(RainbowWheel::new()),
        ]
    }
}
