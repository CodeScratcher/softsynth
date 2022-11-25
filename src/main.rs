mod waves;

use cpal::{Data, Sample, SampleFormat};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

fn main() -> Result<(), String> {
    let host = cpal::default_host();
    let device = host.default_output_device().expect("no output device available");
    
    let mut supported_configs_range = device.supported_output_configs()
        .expect("error while querying configs");
    let supported_config = supported_configs_range.next()
        .expect("no supported config?!")
        .with_max_sample_rate();
    let config = supported_config.into();
    
    let mut time = 0.0;

    let stream = device.build_output_stream(
        &config,
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            for sample in data.iter_mut() {
                let sample_rate = config.sample_rate.0 as f64;
                
                time = (time + 1.0) % sample_rate;
                
                let val: f32 = waves::sine(440.0 / sample_rate, 1.0, time) as f32;
                println!("amp: {}", val);
                *sample = Sample::from(&val);
            }
        },
        move |err| {
            // react to errors here.
        },
    ).unwrap();
    
    stream.play().unwrap();

    std::thread::sleep(std::time::Duration::from_millis(10000));
    Ok(())
}
