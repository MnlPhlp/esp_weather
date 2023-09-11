use std::sync::{Arc, Mutex};

use embedded_hal::blocking::i2c::{Read, WriteRead};
use esp_idf_hal::delay::BLOCK;
use esp_idf_hal::gpio::{Gpio21, Gpio22};
use esp_idf_hal::i2c::{self, I2cConfig, I2C0};
use esp_idf_sys::EspError;

#[derive(Clone)]
pub struct I2cDriver(Arc<Mutex<i2c::I2cDriver<'static>>>);

impl I2cDriver {
    pub fn new(i2c: I2C0, sda: Gpio21, scl: Gpio22) -> Self {
        let config = I2cConfig::new().baudrate(100000.into());
        let i2c = i2c::I2cDriver::new(i2c, sda, scl, &config).unwrap();
        I2cDriver(Arc::new(Mutex::new(i2c)))
    }
}

impl embedded_hal::blocking::i2c::Write for I2cDriver {
    type Error = EspError;

    fn write(&mut self, addr: u8, bytes: &[u8]) -> Result<(), Self::Error> {
        let mut i2c = self.0.lock().unwrap();
        i2c.write(addr, bytes, BLOCK)
    }
}

impl WriteRead for I2cDriver {
    type Error = EspError;

    fn write_read(
        &mut self,
        address: u8,
        bytes: &[u8],
        buffer: &mut [u8],
    ) -> Result<(), Self::Error> {
        let mut i2c = self.0.lock().unwrap();
        i2c.write_read(address, bytes, buffer, BLOCK)
    }
}

impl Read for I2cDriver {
    type Error = EspError;

    fn read(&mut self, address: u8, buffer: &mut [u8]) -> Result<(), Self::Error> {
        let mut i2c = self.0.lock().unwrap();
        i2c.read(address, buffer, BLOCK)
    }
}
