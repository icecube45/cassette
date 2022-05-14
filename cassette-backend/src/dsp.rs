use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::Data;

struct DSP{
    stream: cpal::Stream,
}

impl DSP{
    pub fn new() -> DSP{
        
        let host = cpal::default_host();
        let input_device = host.default_input_device().unwrap();
        let config: cpal::StreamConfig = input_device.default_input_config()?.into();
        let input_stream = input_device.build_input_stream(&config, input_data_fn, err_fn)?;

        let dsp = DSP{
            stream: stream,
        };

        return dsp;
    }


    fn err_fn(err: cpal::StreamError) {
        eprintln!("an error occurred on stream: {}", err);
    }
}


let input_data_fn = move |data: &[f32], _: &cpal::InputCallbackInfo| {
    let mut output_fell_behind = false;
    for &sample in data {
        if producer.push(sample).is_err() {
            output_fell_behind = true;
        }
    }
    if output_fell_behind {
        eprintln!("output stream fell behind: try increasing latency");
    }
};