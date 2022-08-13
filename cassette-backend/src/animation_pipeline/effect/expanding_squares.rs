use std::{sync::Arc, time::SystemTime};

use parking_lot::Mutex;
use rand::Rng;
use tokio::sync::mpsc::{self, Sender, Receiver};

use crate::{animation_pipeline::pixel::Pixel, dsp::{DSP, TempoCallback}};

use super::{Frame, Animate};

pub struct ExpandingSquares {
    eo_count: u32,
    eo_size: u32,
    eo_growth: u32,
    eo_objects_pos_x: Vec<u32>,
    eo_objects_pos_y: Vec<u32>,
    eo_objects_expand: Vec<u32>,
    eo_objects_fade: Vec<u32>,
    eo_objects_col: Vec<Pixel>,
    tempo_event: bool,
    sync_to_tempo: bool,
    dsp: Arc<Mutex<DSP>>,
    tempo_channel_rx: mpsc::Receiver<bool>,
    last_frame: Frame,
    last_ran_at: SystemTime,
    delay: u128,
    color: Pixel,
} 

impl ExpandingSquares {
    pub fn new(dsp: Arc<Mutex<DSP>>) -> Self {
        let (tx, rx): (Sender<bool>, Receiver<bool>) = mpsc::channel(10);
        let mut squares = ExpandingSquares{
            eo_count: 5,
            eo_size: 3,
            eo_growth: 10,
            eo_objects_pos_x: vec![0; 5],
            eo_objects_pos_y: vec![0; 5],
            eo_objects_expand: vec![0; 5],
            eo_objects_fade: vec![0; 5],
            eo_objects_col: vec![Pixel::from_rgb(0, 0, 0); 5],
            tempo_event: false,
            sync_to_tempo: true,
            dsp,
            tempo_channel_rx: rx,
            last_frame: Frame::new(0, 0),
            last_ran_at: SystemTime::UNIX_EPOCH,
            delay: 20,
            color: Pixel::from_rgb(255, 0, 255),
            };
        let mut rng = rand::thread_rng();
        for i in 0..squares.eo_count as usize{
            squares.eo_objects_pos_x[i] = rng.gen_range(0..30 as u32);
            squares.eo_objects_pos_y[i] = rng.gen_range(0..10 as u32);
            squares.eo_objects_expand[i] = rng.gen_range(0..squares.eo_growth);
            squares.eo_objects_fade[i] = 0;
            squares.eo_objects_col[i] = squares.color;
        }
        if(squares.sync_to_tempo) {
            let mut dsp = squares.dsp.lock();
            // create arc mutex of squares
            // create tempo callback
            dsp.add_tempo_channel(tx.clone());
        }
    squares
    }

}


impl Animate for ExpandingSquares {
    fn animate(&mut self, frame: &mut Frame) {
        
        self.animate_expanding_squares(frame);
    }
}

impl ExpandingSquares{
    pub fn animate_expanding_squares(&mut self, frame: &mut Frame){
        // receive all tempo_channel_rx events
        if(self.sync_to_tempo) {
            while(self.tempo_channel_rx.try_recv().is_ok()) {
                self.tempo_event = true;
            }
        }

        // check if current time minus last_ran_at is greater than delay
        // get current unix time
        let now = std::time::SystemTime::now();
        if(now.duration_since(self.last_ran_at).unwrap().as_millis() < self.delay){
            frame.copy_from(&self.last_frame);
            return;
        }
        self.last_ran_at = now;


        for j in 0..self.eo_count as usize{
            let r = (self.eo_objects_col[j].r as f32 * (1f32 - self.eo_objects_fade[j] as f32 / self.eo_growth as f32)) as u8;
            let g = (self.eo_objects_col[j].g as f32 * (1f32 - self.eo_objects_fade[j] as f32 / self.eo_growth as f32)) as u8;
            let b = (self.eo_objects_col[j].b as f32 * (1f32 - self.eo_objects_fade[j] as f32 / self.eo_growth as f32)) as u8;
            let pixel = Pixel{ r, g, b};
            let diameter = self.eo_size + self.eo_objects_expand[j];
            let height = frame.height();
            let width  = frame.width();
        
            if self.eo_objects_expand[j] < self.eo_growth {
                let offset = diameter / 2;
                frame.draw_rect(self.eo_objects_pos_x[j] as i32-offset as i32, self.eo_objects_pos_y[j] as i32 - offset as i32, diameter as i32, diameter as i32, pixel);
            }
            self.eo_objects_expand[j] += 2;
            self.eo_objects_fade[j] += 2;
            if self.eo_objects_fade[j] >= self.eo_growth {
                self.eo_objects_fade[j] = self.eo_growth;
            }
            if(!self.sync_to_tempo){
                    // get random int in range
                    let mut rng = rand::thread_rng();
                    if self.eo_objects_expand[j] >= self.eo_growth && rng.gen_range(0..100 as u8) < 10 {
                        self.eo_objects_expand[j] = 0;
                        self.eo_objects_fade[j] = 0;
                        self.eo_objects_pos_x[j] = rng.gen_range(0..width as u32);
                        self.eo_objects_pos_y[j] = rng.gen_range(0..height as u32);
                    }
            }
        }
        if self.tempo_event {
            let height = frame.height();
            let width  = frame.width();
            for j in 0..self.eo_count as usize{
                if(self.eo_objects_expand[j] >= self.eo_growth){
                    self.eo_objects_expand[j] = 0;
                    self.eo_objects_fade[j] = 0;
                    let mut rng = rand::thread_rng();
                    self.eo_objects_pos_x[j] = rng.gen_range(0..width as u32);
                    self.eo_objects_pos_y[j] = rng.gen_range(0..height as u32);
                    break;
                }
            }
        }
    self.tempo_event = false;
    self.last_frame = frame.clone();
    }


}
