use std::error::Error;

use cpal::traits::HostTrait;
use cpal::Device;

pub fn init_output_derive() -> Result<Device, Box<dyn Error>> {
    let host = cpal::default_host();

    let device = host
        .default_output_device()
        .expect("Failed to find a default output device");
    Ok(device)
}
