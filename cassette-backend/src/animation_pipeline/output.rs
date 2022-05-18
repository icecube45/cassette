use std::sync::{Mutex, Arc};

use hecs::Entity;

use super::{effect::EffectComponent, mixer::MixerComponent};


pub struct Output {
    name: String,
    entity: Entity,
    effects_a: Vec<Arc<Mutex<EffectComponent>>>,
    effects_b: Vec<Arc<Mutex<EffectComponent>>>,
    output_mixer: Arc<Mutex<MixerComponent>>,
    mixer_a: Arc<Mutex<MixerComponent>>,
    mixer_b: Arc<Mutex<MixerComponent>>
}