use std::sync::{Arc};

use axum::extract::ws::{WebSocket, Message};
use cpal::{StreamConfig};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

use hecs::World;
use parking_lot::{Mutex, RwLock};
use realfft::{RealFftPlanner};
use rustfft::num_complex::Complex;
use rustfft::num_traits::Zero;
use tokio::runtime::Runtime;


pub struct DSPWrapper {
    pub dsp: Arc<Mutex<DSP>>,
    pub stream: cpal::Stream
}

impl DSPWrapper {
    pub fn new(world: Arc<RwLock<World>>) -> DSPWrapper {
        let ret = DSP::new(world);
        DSPWrapper { dsp: ret.0, stream: ret.1 }
    }
}

pub struct DSP{
    fft_planner: RealFftPlanner::<f64>,
    buffer_size: usize,
    pub spectrum: Vec<Complex<f64>>,
    world: Arc<RwLock<World>>

}


impl DSP{
    pub fn new(world: Arc<RwLock<World>>) -> (Arc<Mutex<DSP>>, cpal::Stream) {
        
        let host = cpal::default_host();
        let output_device = host.default_output_device().unwrap();
        let mut default_config: StreamConfig = output_device.default_output_config().expect("Failed to get default output config").into();

        // for each supported config
        // for config in input_device.supported_input_configs().expect("Failed to get supported input configs") {
        //     // println!("{:?}", config);
        // }

     
        // let mut real_planner = RealFftPlanner::<f64>::new();

        let dsp = DSP{
            fft_planner: RealFftPlanner::<f64>::new(),
            // fft: Arc::new(RealFftPlanner::new().plan_r2c(buffer_size)),
            buffer_size: 0,
            spectrum: vec![Complex::zero(); 0],
            world
        };

        let dsp_arc: Arc<Mutex<DSP>> = Arc::new(Mutex::new(dsp));

        let send_to_callback = dsp_arc.clone();

        let input_stream = output_device.build_input_stream(
            &default_config, 
            {
                move |data, _: &_| DSP::input_data_fn(send_to_callback.clone(), data)
            },
            DSP::err_fn)
            .unwrap();

        
        
        input_stream.play().unwrap();
        // println!("Created new DSP");
        return (dsp_arc, input_stream);
    }


    fn err_fn(err: cpal::StreamError) {
        eprintln!("an error occurred on stream: {}", err);
    }
    
    fn input_data_fn(dsp: Arc<Mutex<DSP>>, data: &[f32]) {
        let mut dsp = dsp.lock();
        dsp.process_data(data);
     
        // println!("DSP");
        // println!("{:?}", data);
        // print length of data
        // println!("{:?}", data.len());
    }

    pub fn process_data(&mut self, data: &[f32]) {
        
        let r2c = self.fft_planner.plan_fft_forward(data.len());
        
        let mut in_data = r2c.make_input_vec();
        // copy data to in_data while also converting f32 to f64
        for (i, x) in data.iter().enumerate() {
            in_data[i] = *x as f64;
        }


        let mut spectrum = r2c.make_output_vec();
        r2c.process(&mut in_data, &mut spectrum).unwrap();
        self.spectrum = spectrum;
        // println!("{:?}", spectrum);


        // build a json string from spectrum values
        let mut json_string = String::new();
        json_string.push_str("[");
        for i in 0..self.spectrum.len() {
            json_string.push_str(&format!("{:?},", self.spectrum[i].re));
        }
        json_string.pop();
        json_string.push_str("]");
        let world = self.world.read();

        world.query::<&Arc<Mutex<WebSocket>>>().iter().for_each(|(entity, socket)| {
            let rt = Runtime::new().unwrap();
            rt.block_on(async {
                if socket.lock()
                    // .send(Message::Text(String::from(json_string.clone())))
                    .send(Message::Text("hello".to_string()))
                    .await
                    .is_err() 
                {
                    println!("Failed to send message");
                    // if world.despawn(entity).is_err() {
                    //     println!("Error despawning entity");
                    // }
                }
            });
        });
    }
}