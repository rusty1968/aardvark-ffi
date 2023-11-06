use crate::plugin::Aardvark;
use crate::plugin::AardvarkError;
use crate::plugin::AardvarkStatus_AA_UNABLE_TO_LOAD_DRIVER;
use crate::AardvarkHandle;
use embedded_hal::blocking::i2c::SevenBitAddress;
use embedded_hal::blocking::i2c::{Read, Write, WriteRead};
pub struct I2CDevice {
    handle: AardvarkHandle,
}

impl I2CDevice {
    pub fn new(handle: AardvarkHandle) -> Self {
        Self { handle }
    }
}

impl Write<SevenBitAddress> for I2CDevice {
    type Error = AardvarkError;

    fn write(&mut self, _address: SevenBitAddress, _bytes: &[u8]) -> Result<(), Self::Error> {
        unimplemented!()
    }
}

impl WriteRead<SevenBitAddress> for I2CDevice {
    type Error = AardvarkError;

    fn write_read(
        &mut self,
        _address: SevenBitAddress,
        _bytes: &[u8],
        _buffer: &mut [u8],
    ) -> Result<(), Self::Error> {
        unimplemented!()
    }
}

impl Read<SevenBitAddress> for I2CDevice {
    type Error = AardvarkError;

    fn read(&mut self, _address: SevenBitAddress, _buffer: &mut [u8]) -> Result<(), Self::Error> {
        unimplemented!()
    }
}
