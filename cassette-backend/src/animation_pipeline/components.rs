use hecs::Bundle;
use serde::{Deserialize, Serialize};
use ndarray::{Axis, Array2};
use std::{ops::Mul};

#[derive(Bundle, Deserialize, Debug, Serialize, Clone, Copy)]
pub struct Pixel {
    pub(crate) r: u8,
    pub(crate) g: u8,
    pub(crate) b: u8,
}

impl Pixel{
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Pixel {
        Pixel{
            r,
            g,
            b,
        }
    }
    pub fn black() -> Pixel {
        Pixel{
            r: 0,
            g: 0,
            b: 0,
        }
    }
    pub fn red() -> Pixel {
        Pixel{
            r: 255,
            g: 0,
            b: 0,
        }
    }
    pub fn green() -> Pixel {
        Pixel{
            r: 0,
            g: 255,
            b: 0,
        }
    }
    pub fn blue() -> Pixel {
        Pixel{
            r: 0,
            g: 0,
            b: 255,
        }
    }
    pub fn get_intensity(&self) -> f32 {
        ((self.r as u32 + self.g as u32 + self.b as u32) as f32 / 3f32) / 255f32
    }
    pub fn is_transparent(&self) -> bool {
        self.r == 0 && self.g == 0 && self.b == 0
    }
}

impl Mul for Pixel {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Pixel{
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b,
        }
    }
}

impl Mul<f32> for Pixel {
    type Output = Self;
    fn mul(self, other: f32) -> Self {
        Pixel{
            r: (self.r as f32 * other) as u8,
            g: (self.g as f32 * other) as u8,
            b: (self.b as f32 * other) as u8,
        }
    }
}


#[derive(Bundle, Deserialize, Debug, Serialize)]
pub struct Output {
    pub(crate) name: String,
    pub width: u32,
    pub height: u32,
}

// //A mixer has an output, 
// struct mixer {
//     outputs: Vec<output>,
//     //inputs: Vec<input>,
// }


// This is the struct that gets copied from stage to stage...


struct EffectSettings {
    //pub(crate) active_effect: Effect,
    
}

// struct Animation {
//     pub current_frame: Frame,
//     pub next_frame: Frame,
// }

enum Animation2 {
    Animation,

}




// Animation is a struct that contains an animation function of a given type and returns