use std::sync::{Arc, Mutex};

use cpal::{StreamConfig};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

use realfft::{RealFftPlanner};
use rustfft::num_complex::Complex;
use rustfft::num_traits::Zero;


pub struct DSPWrapper {
    pub dsp: Arc<Mutex<DSP>>,
    pub stream: cpal::Stream
}

impl DSPWrapper {
    pub fn new() -> DSPWrapper {
        let ret = DSP::new();
        DSPWrapper { dsp: ret.0, stream: ret.1 }
    }
}

pub struct DSP{
    fft_planner: RealFftPlanner::<f64>,
    buffer_size: usize,
    pub spectrum: Vec<Complex<f64>>,

}


impl DSP{
    pub fn new() -> (Arc<Mutex<DSP>>, cpal::Stream) {
        
        let host = cpal::default_host();
        let input_device = host.default_input_device().unwrap();
        let mut config: StreamConfig = input_device.default_input_config().expect("Failed to get default input config").into();

        // for each supported config
        for config in input_device.supported_input_configs().expect("Failed to get supported input configs") {
            // println!("{:?}", config);
        }

     
        // let mut real_planner = RealFftPlanner::<f64>::new();

        let dsp = DSP{
            fft_planner: RealFftPlanner::<f64>::new(),
            // fft: Arc::new(RealFftPlanner::new().plan_r2c(buffer_size)),
            buffer_size: 0,
            spectrum: vec![Complex::zero(); 0],
        };

        let dsp_arc: Arc<Mutex<DSP>> = Arc::new(Mutex::new(dsp));

        let send_to_callback = dsp_arc.clone();

        let input_stream = input_device.build_input_stream(
            &config, 
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
        let mut dsp = dsp.lock().unwrap();
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

    }
}