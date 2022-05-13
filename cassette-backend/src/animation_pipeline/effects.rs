use ndarray::Array2;

use super::components::{Pixel, EffectSettings};

mod rainbow_wheel;
mod expanding_squares;

pub struct Frame {
    pub pixels: Array2<Pixel>,
    timestamp: u128
}

trait Animation {
    fn animate(&mut self, frame: &mut Frame);
}

struct Effect<T: Animation> {
    frame: Frame,
    effect: T,
}

fn test() {
    let test = Effect{
        frame: Frame{
            pixels: todo!(),
            timestamp: 0,
        },
        effect: rainbow_wheel::RainbowWheel{
            step: 0,
        },
    };
}

