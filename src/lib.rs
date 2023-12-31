pub mod error;
pub mod i2c;
pub mod plugin;

pub use plugin::{Aardvark, AardvarkApi, AardvarkError};

pub type AardvarkResult<T> = Result<T, AardvarkError>;

#[derive(Clone, Debug)]
pub struct AardvarkHandle(Aardvark);

use libc::c_int;
use plugin::AA_PORT_NOT_FREE;

pub fn aa_open(device: u16) -> AardvarkResult<AardvarkHandle> {
    let api = unsafe { plugin::AardvarkApi::try_load("./dynamic-lib/aardvark.so").unwrap() };
    let status = api.aa_open(device as c_int);
    if status < 0 {
        return Err(AardvarkError::new(status));
    }
    Ok(AardvarkHandle(status))
}
pub fn find_aardvark_devices() -> AardvarkResult<Vec<u16>> {
    let api = unsafe { plugin::AardvarkApi::try_load("./dynamic-lib/aardvark.so").unwrap() };
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

pub fn find_unused_aardvark_devices() -> AardvarkResult<Vec<u16>> {
    let devices = find_aardvark_devices()?;
    let unused_devices = devices
        .into_iter()
        .filter(|device| (*device & AA_PORT_NOT_FREE as u16) == 0);
    Ok(unused_devices.collect())
}

pub fn find_and_open_first_unused_device() -> AardvarkResult<AardvarkHandle> {
    let devices = find_unused_aardvark_devices()?;

    if let Some(device) = devices.first() {
        return aa_open(*device);
    }
    Err(AardvarkError::UNABLE_TO_FIND_UNUSED_DEVICE)
}
