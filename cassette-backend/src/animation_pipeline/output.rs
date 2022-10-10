use std::sync::{Arc};
use parking_lot::Mutex;

use serde::{Serialize};

use crate::dsp::DSP;

use super::{effect::{Animate, Effect}, mixer::{MixerComponent}, patcher::Patcher, frame::Frame};

#[derive(Serialize)]
pub struct EntityResponse {
    id: u64
}

pub struct Output {
    pub(crate) name: String,
    patcher: Patcher,
    width: u32,
    height: u32,
    dsp: Arc<Mutex<DSP>>,
    channelAEffects: Vec<Effect>,
    channelBEffects: Vec<Effect>,
    channelCEffects: Vec<Effect>,
    channelDEffects: Vec<Effect>,
    channelACurrentEffectIndex: usize,
    channelBCurrentEffectIndex: usize,
    channelCCurrentEffectIndex: usize,
    channelDCurrentEffectIndex: usize,
    mixer1: MixerComponent,
    mixer2: MixerComponent,
    masterMixer: MixerComponent

}


impl Output {
    pub fn new(width: u32, height: u32, dsp: Arc<Mutex<DSP>>) -> Self {
        let mut out = Output {
            name: "unnamed".to_string(),
            patcher: Patcher::new(),
            width: width,
            height: height,
            dsp: dsp.clone(),
            channelAEffects: Effect::new_effects_set(dsp.clone()),
            channelBEffects: Effect::new_effects_set(dsp.clone()),
            channelCEffects: Effect::new_effects_set(dsp.clone()),
            channelDEffects: Effect::new_effects_set(dsp.clone()),
            channelACurrentEffectIndex: 0,
            channelBCurrentEffectIndex: 2,
            channelCCurrentEffectIndex: 0,
            channelDCurrentEffectIndex: 4,
            mixer1: MixerComponent::new(),
            mixer2: MixerComponent::new(),
            masterMixer: MixerComponent::new()

        };
        out
    }

    pub fn process(&mut self) -> Frame{
        // TODO: maybe don't make a new frame each loop lmao

        // Process all effects
        let mut channel_a_frame = Frame::new(self.width, self.height);
        let mut channel_b_frame = Frame::new(self.width, self.height);
        let mut channel_c_frame = Frame::new(self.width, self.height);
        let mut channel_d_frame = Frame::new(self.width, self.height);

        self.channelAEffects[self.channelACurrentEffectIndex].animate(&mut channel_a_frame);
        self.channelBEffects[self.channelBCurrentEffectIndex].animate(&mut channel_b_frame);
        self.channelCEffects[self.channelCCurrentEffectIndex].animate(&mut channel_c_frame);
        self.channelDEffects[self.channelDCurrentEffectIndex].animate(&mut channel_d_frame);

        // Process all mixers
        let sub_frame_1 = self.mixer1.mix(&channel_a_frame, &channel_b_frame);
        let sub_frame_2 = self.mixer2.mix(&channel_c_frame, &channel_d_frame);

        let resultant_frame = self.masterMixer.mix(&sub_frame_1, &sub_frame_2);

        // Output to device(s)
        // TODO
        // for now just returning frame
        resultant_frame
    }

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