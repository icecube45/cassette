use std::{sync::{Arc, RwLock}, rc::Rc};

use hecs::{Bundle, Entity};
use serde::{Deserialize, Serialize};
use ndarray::{Axis, Array2};

#[derive(Bundle, Deserialize, Debug, Serialize)]
pub struct Pixel {
    pub(crate) r: u8,
    pub(crate) g: u8,
    pub(crate) b: u8,
}

// output contains references to mixer and effects
#[derive(Bundle, Deserialize, Debug, Serialize)]
pub struct Output {
    pub(crate) name: String,
    // other metadata
    //pub(crate) mixer: 
}

pub struct Frame {
    pub pixels: Array2<Pixel>,
    timestamp: u128
}
