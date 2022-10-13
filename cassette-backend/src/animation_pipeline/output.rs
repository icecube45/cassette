use std::{sync::{Arc}, default};
use axum::extract::ws::{WebSocket, Message};
use parking_lot::Mutex;

use serde::{Serialize};
use tokio::{runtime::{Runtime, Handle}};

use crate::dsp::DSP;

use super::{effect::{Animate, Effect}, mixer::{MixerComponent}, patcher::Patcher, frame::Frame};

#[derive(Serialize)]
pub struct ApiRepresentation {
    pub id: u64,
    pub name: String,
    pub output_type: String,
    pub active: bool,

}

pub struct Output {
    pub(crate) name: String,
    patcher: Patcher,
    enabled: bool,
    output_type: String,
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
    pub mixer1: MixerComponent,
    pub mixer2: MixerComponent,
    pub masterMixer: MixerComponent,
    display_websockets: Vec<Arc<Mutex<WebSocket>>>,

}


impl Output {
    pub fn new(width: u32, height: u32, dsp: Arc<Mutex<DSP>>, index: usize) -> Self {
        let mut out = Output {
            name: "unnamed".to_string(),
            output_type: "matrix".to_string(),
            patcher: Patcher::new(),
            enabled: false,
            width: width,
            height: height,
            dsp: dsp.clone(),
            channelAEffects: Effect::new_effects_set(dsp.clone()),
            channelBEffects: Effect::new_effects_set(dsp.clone()),
            channelCEffects: Effect::new_effects_set(dsp.clone()),
            channelDEffects: Effect::new_effects_set(dsp.clone()),
            channelACurrentEffectIndex: 0,
            channelBCurrentEffectIndex: index,
            channelCCurrentEffectIndex: 0,
            channelDCurrentEffectIndex: 0,
            mixer1: MixerComponent::new(1),
            mixer2: MixerComponent::new(2),
            masterMixer: MixerComponent::new(3),
            display_websockets: Vec::new()

        };
        out
    }

    pub fn add_websocket(&mut self, socket: Arc<Mutex<WebSocket>>) {
        self.display_websockets.push(socket);
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn enabled(&mut self) -> bool {
        return self.enabled;
    }

    pub fn get_name(&mut self) -> String {
        return self.name.clone();
    }

    pub fn get_output_type(&mut self) -> String {
        return self.output_type.clone();
    }

    pub fn get_api_representation(&mut self) -> ApiRepresentation {
        ApiRepresentation { id: 0, name: self.name.clone(), output_type: self.output_type.clone(), active: self.enabled }
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

        
        
        if self.display_websockets.len() != 0 {
            let mut json_frame: String = "[".to_string();
            for pixel in resultant_frame.pixels.iter() {
                json_frame.push_str(&format!("{},", pixel));
            }
            json_frame.pop();
            json_frame.push(']');
            


            self.display_websockets.retain(|socket| {
                let handle = Handle::current();
                handle.enter();
                return futures::executor::block_on(async {
                    if socket.lock()
                        .send(Message::Text(String::from(json_frame.clone())))
                        // .send(Message::Text("hello".to_string()))
                        .await
                        .is_err() 
                    {
                        println!("Failed to send frame to display");
                        return false;
                    }
                    else {
                        return true;
                    }
                });
            });


        }
            
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