mod rainbow_wheel;
mod expanding_squares;

use ndarray::Array2;

use super::components::{Pixel, Frame};

use self::rainbow_wheel::RainbowWheel;
use self::expanding_squares::ExpandingSquares;

#[enum_dispatch]

pub trait Animate {
    fn animate(&mut self, frame: &mut Frame);
}
pub struct EffectComponent {
    name: String,
    effect: Effects,
}

impl EffectComponent {
    pub fn new(name: String, effect: Effects) -> EffectComponent {
        EffectComponent {
            name,
            effect,
        }
    }
}

#[enum_dispatch(Animate)]
enum Effects {
    RainbowWheel,
    ExpandingSquares,
    // ...
}