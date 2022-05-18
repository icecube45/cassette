use std::sync::{Mutex, Arc};

use hecs::{Entity, Bundle};
use serde::{Serialize, Deserialize};

use super::{effect::EffectComponent, mixer::MixerComponent};

#[derive(Serialize)]
pub struct EntityResponse {
    id: u64
}

#[derive(Bundle, Deserialize, Debug, Serialize)]
pub struct Output {
    pub(crate) name: String,
    pub width: u32,
    pub height: u32,
}

// pub struct Output {
//     name: String,
//     entity: Entity,
//     effects_a: Vec<Arc<Mutex<EffectComponent>>>,
//     effects_b: Vec<Arc<Mutex<EffectComponent>>>,
//     output_mixer: Arc<Mutex<MixerComponent>>,
//     mixer_a: Arc<Mutex<MixerComponent>>,
//     mixer_b: Arc<Mutex<MixerComponent>>
// }