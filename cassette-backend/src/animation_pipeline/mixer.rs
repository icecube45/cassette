pub mod progressive;
pub mod linear;
pub mod shape;
pub mod intensity;
pub mod overlay;
pub mod border;

use axum::Json;

use crate::api;
use crate::api::option::{ApiOption, EffectOptionType};

use self::progressive::Progressive;
use self::linear::Linear;
use self::shape::Shape;
use self::intensity::Intensity;
use self::overlay::Overlay;
use self::border::Border;

use super::frame::Frame;


pub struct ApiRepresentation {
    id: usize,
    options: Vec<api::option::ApiOption>,
}



pub struct MixerComponent {
    // pub(crate) name:          String,
    // pub(crate) entity:        Entity,
    id: usize,
    mixer_type:    MixMode,
    pub(crate) weight:        f32,
    swap: bool,
    // pub(crate) channel_a:     Arc<Mutex<Frame>>,
    // pub(crate) channel_b:     Arc<Mutex<Frame>>,
    // pub(crate) output:        Arc<Mutex<Frame>>,
}

impl MixerComponent {
    pub fn new(id: usize) -> Self {
        MixerComponent {
            id,
            mixer_type: MixMode::Progressive(Progressive{}),
            weight: 0.5,
            swap: false,
        }
    }

    pub fn set_mixer_mode(&mut self, mode: MixMode){
        self.mixer_type = match mode {
            MixMode::Border(_) => MixMode::Border(Border{}),
            MixMode::Progressive(_) => MixMode::Progressive(Progressive{}),
            MixMode::Linear(_) => MixMode::Linear(Linear{}),
            MixMode::Shape(_) => MixMode::Shape(Shape{}),
            MixMode::Intensity(_) => MixMode::Intensity(Intensity{}),
            MixMode::Overlay(_) => MixMode::Overlay(Overlay{}),

        };
    }

    pub fn mix(&mut self, channel_a: &Frame, channel_b: &Frame) -> Frame {
        if self.swap {
            return self.mixer_type.mix(self.weight, channel_b, channel_a);
        }
        return self.mixer_type.mix(self.weight, channel_a, channel_b);
    }

    pub fn get_api_representation(&mut self) -> ApiRepresentation {
        ApiRepresentation { id: self.id , options: vec![
            ApiOption{
                id: 1,
                option_type: EffectOptionType::select,
                options: vec!["Progressive".to_string(),
                "Linear".to_string(),
                "Left Shape".to_string(),
                "Right Shape".to_string(),
                "Left Intensity".to_string(),
                "Right Intensity".to_string(),
                "Left Overlay".to_string(),
                "Right Overlay".to_string(),
                "Left Overlay (Border)".to_string(),
                "Right Overlay (Border)".to_string()],
                select_value: match self.mixer_type {
                    MixMode::Border(_) => if self.swap {"Right Overlay (Border)".to_string()} else {"Left Overlay (Border)".to_string()},
                    MixMode::Progressive(_) => "Progressive".to_string(),
                    MixMode::Linear(_) => "Linear".to_string(),
                    MixMode::Shape(_) => {if self.swap {"Right Shape".to_string()} else {"Left Shape".to_string()}},
                    MixMode::Intensity(_) => if self.swap {"Right Intensity".to_string()} else {"Left Intensity".to_string()},
                    MixMode::Overlay(_) => if self.swap {"Right Overlay".to_string()} else {"Left Overlay".to_string()},
                },
                min: 0,
                max: 0,
                name: "".to_string(),
                analog_value: 0,
                boolean_value: false,
                color_value: "".to_string(),
            },
            ApiOption{
                id: 2,
                name: "Weight".to_string(),
                option_type: EffectOptionType::analog,
                min: 0,
                max: 100,
                analog_value: 50,
                options: vec![],
                boolean_value: false,
                color_value: "".to_string(),
                select_value: "".to_string(),
            }
        ] }
    }


    // The reason it's implemented this way is to add some checks later for frame of given size...
    // pub fn new(name: String, entity: Entity, weight: Option<f32>, channel_a: Option<Arc<Mutex<Frame>>>, channel_b: Option<Arc<Mutex<Frame>>>, output: Option<Arc<Mutex<Frame>>>) -> Result<Self, &'static str> {
    //     // checks for properly sized inputs
    //     Ok(MixerComponent {
    //         name,
    //         entity,
    //         mixer_type: todo!(),
    //         weight: match weight {
    //             Some(w) => w,
    //             None => 0.0,
    //         },
    //         channel_a: match channel_a {
    //             Some(c) => c,
    //             None => todo!(),
    //         },
    //         channel_b: match channel_b {
    //             Some(c) => c,
    //             None => todo!(),
    //         },
    //         output: match output {
    //             Some(c) => c,
    //             None => todo!(),
    //         },
    //     })
    // }
    // pub fn mix(&mut self) {
    //     match self.output.lock() {
    //         Ok(mut output) => *output = self.mixer_type.mix(self.weight, &self.channel_a.lock().unwrap(), &self.channel_b.lock().unwrap()),
    //         Err(_) => todo!(),
    //     }
    // }
}

#[enum_dispatch]

pub trait Mix {
    fn mix(&mut self, weight: f32, channel_a: &Frame, channel_b: &Frame) -> Frame;
}

#[enum_dispatch(Mix)]
pub enum MixMode {
    Progressive,
    Linear,
    Shape,
    Intensity,
    Overlay,
    Border
}
