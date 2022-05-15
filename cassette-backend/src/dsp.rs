use std::sync::{Arc, Mutex};

use cpal::{BufferSize, StreamConfig};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

use realfft::{RealFftPlanner, RealToComplex};
use rustfft::num_complex::Complex;
use rustfft::num_traits::Zero;



pub struct DSP{
    stream: cpal::Stream,
    // fft: Arc<dyn RealToComplex<f64>>,
    buffer_size: usize,
}

impl DSP{
    pub fn new() -> DSP{
        
        let host = cpal::default_host();
        let input_device = host.default_input_device().unwrap();
        let mut config: StreamConfig = input_device.default_input_config().expect("Failed to get default input config").into();

        // for each supported config
        for config in input_device.supported_input_configs().expect("Failed to get supported input configs") {
            println!("{:?}", config);
        }

     
        // let mut real_planner = RealFftPlanner::<f64>::new();
        let real_planner = Arc::new(Mutex::new(RealFftPlanner::<f64>::new()));

        let input_stream = input_device.build_input_stream(
            &config, 
            {
                let real_planner = real_planner.clone();
                move |data, _: &_| DSP::input_data_fn(real_planner, data)
            },
            DSP::err_fn)
            .unwrap();

        
        
        
        input_stream.play().unwrap();
        let dsp = DSP{
            stream: input_stream,
            // fft: Arc::new(RealFftPlanner::new().plan_r2c(buffer_size)),
            buffer_size: 0,
        };
        println!("Created new DSP");
        return dsp;
    }


    fn err_fn(err: cpal::StreamError) {
        eprintln!("an error occurred on stream: {}", err);
    }
    
    fn input_data_fn(real_planner: Arc<std::sync::Mutex<RealFftPlanner<f64>>>, data: &[f32]) {
        let mut real_planner = real_planner.lock().unwrap();
        let r2c = real_planner.plan_fft_forward(1024);



        println!("DSP");
        // println!("{:?}", data);
        // print length of data
        println!("{:?}", data.len());
    }
}