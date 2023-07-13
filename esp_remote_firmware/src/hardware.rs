use anyhow::Error;
use esp_idf_hal::gpio::{self, Gpio15, Gpio19, OutputPin, PinDriver};
use esp_idf_hal::i2c::{I2cConfig, I2cDriver};
use esp_idf_hal::prelude::*;

pub struct Hardware {
    pub led: gpio::Gpio2,
    pub i2c: I2cDriver<'static>,
    pub dht: PinDriver<'static, Gpio19, gpio::InputOutput>,
}

pub fn get_hardware() -> Hardware {
    let p = Peripherals::take()
        .ok_or(Error::msg("could not take Peripherals"))
        .unwrap();
    let i2c = p.i2c0;
    let sda = p.pins.gpio21;
    let scl = p.pins.gpio22;

    let config = I2cConfig::new().baudrate(100.kHz().into());
    let i2c = I2cDriver::new(i2c, sda, scl, &config).unwrap();

    let dht = PinDriver::input_output(p.pins.gpio19).unwrap();

    Hardware {
        led: p.pins.gpio2,
        i2c,
        dht,
    }
}
