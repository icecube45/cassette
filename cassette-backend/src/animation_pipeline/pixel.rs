use core::fmt;
use std::ops::Mul;

#[derive(Debug, Clone, Copy)]
pub struct Pixel {
    pub(crate) r: u8,
    pub(crate) g: u8,
    pub(crate) b: u8,
}

impl fmt::Display for Pixel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{\"r\":{}, \"g\":{}, \"b\":{}, \"patched\":true}}", self.r, self.g, self.b)
    }
}

impl Pixel{
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Pixel {
        Pixel{
            r,
            g,
            b,
        }
    }
    pub fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Pixel {
        // treat black as alpha, scalre r g and b accordingly
        let r = (r as f32 * (a as f32 / 255.0)) as u8;
        let g = (g as f32 * (a as f32 / 255.0)) as u8;
        let b = (b as f32 * (a as f32 / 255.0)) as u8;
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