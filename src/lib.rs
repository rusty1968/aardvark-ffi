pub mod error;
pub mod i2c;
pub mod plugin;

pub use plugin::{AardvarkApi, AardvarkError};

pub type AardvarkResult<T> = Result<T, AardvarkError>;

pub fn find_aardvark_devices() -> AardvarkResult<Vec<u16>> {
    let api = unsafe { plugin::AardvarkApi::try_load("./dynamic-lib/libaardvark.so").unwrap() };
    let mut devices: [u16; 16] = [0; 16];
    let num_devices = api.aa_find_devices(devices.len() as i32, devices.as_mut_ptr());
    if num_devices < 0 {
        // Return empty vector if no devices are found
        return Err(AardvarkError::new(num_devices));
    }

    let num_devices = (num_devices as usize).min(devices.len());
    // Truncate array to number of devices found or the size of the devices array
    Ok(devices[0..num_devices].to_vec())
}
