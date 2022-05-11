use hecs::Bundle;
use serde::{Deserialize, Serialize};
use ndarray::{Axis, Array2};

#[derive(Bundle, Deserialize, Debug, Serialize)]
pub struct Pixel {
    pub(crate) r: u8,
    pub(crate) g: u8,
    pub(crate) b: u8,
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