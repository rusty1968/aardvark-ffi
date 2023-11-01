use crate::plugin::Aardvark;
use crate::plugin::AardvarkError;
use embedded_hal::blocking::i2c::SevenBitAddress;
use embedded_hal::blocking::i2c::{Read, Write, WriteRead};
pub struct I2CDevice {
    port: Aardvark,
}

impl I2CDevice {
    pub fn new(port: Aardvark) -> Self {
        Self { port }
    }
}

impl Write<SevenBitAddress> for I2CDevice {
    type Error = AardvarkError;

    fn write(&mut self, _address: SevenBitAddress, _bytes: &[u8]) -> Result<(), Self::Error> {
        Err(AardvarkError::UNABLE_TO_LOAD_LIBRARY)
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
