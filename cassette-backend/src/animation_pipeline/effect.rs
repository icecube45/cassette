pub mod rainbow_wheel;
pub mod expanding_squares;

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
}

impl EffectComponent {
    pub fn new(name: String, effect: Effect) -> EffectComponent {
        EffectComponent {
            name,
            effect,
        }
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
