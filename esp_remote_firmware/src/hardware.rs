use std::convert::Infallible;
use std::sync::{Arc, Mutex};

use anyhow::Error;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::{prelude::*, primitives};
use esp_idf_hal::delay::BLOCK;
use esp_idf_hal::gpio::{self, PinDriver};
use esp_idf_hal::i2c::{self, I2cConfig};
use esp_idf_hal::prelude::*;
use esp_idf_sys::EspError;
use sh1106::prelude::*;

#[derive(Clone)]
pub struct I2cDriver(Arc<Mutex<i2c::I2cDriver<'static>>>);

impl embedded_hal::blocking::i2c::Write for I2cDriver {
    type Error = EspError;

    fn write(&mut self, addr: u8, bytes: &[u8]) -> Result<(), Self::Error> {
        self.0.lock().unwrap().write(addr, bytes, BLOCK)
    }
}

#[derive(Clone)]
pub struct Display {
    disp: Arc<Mutex<GraphicsMode<I2cInterface<I2cDriver>>>>,
    bounding_box_value: primitives::Rectangle,
}
impl Display {
    pub fn clear(&self) {
        self.disp.lock().unwrap().clear();
    }
    pub fn flush(&self) -> Result<(), sh1106::Error<EspError, ()>> {
        self.disp.lock().unwrap().flush()
    }
    pub fn init(&self) -> Result<(), sh1106::Error<EspError, ()>> {
        self.disp.lock().unwrap().init()
    }
}
impl Dimensions for Display {
    fn bounding_box(&self) -> primitives::Rectangle {
        self.bounding_box_value
    }
}
impl DrawTarget for Display {
    type Color = BinaryColor;

    type Error = Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        self.disp.lock().unwrap().draw_iter(pixels)
    }
}

pub struct Hardware {
    pub led: gpio::Gpio2,
    pub i2c: I2cDriver,
    pub disp: Display,
    pub temp_in_addr: i32,
    pub temp_out_addr: i32,
}

pub fn get_hardware() -> Hardware {
    let p = Peripherals::take()
        .ok_or(Error::msg("could not take Peripherals"))
        .unwrap();
    let i2c = p.i2c0;
    let sda = p.pins.gpio21;
    let scl = p.pins.gpio22;

    let config = I2cConfig::new().baudrate(100.kHz().into());
    let i2c = i2c::I2cDriver::new(i2c, sda, scl, &config).unwrap();
    let i2c = I2cDriver(Arc::new(Mutex::new(i2c)));

    let disp: GraphicsMode<_> = sh1106::Builder::new()
        .with_size(DisplaySize::Display128x64)
        .connect_i2c(i2c.clone())
        .into();
    let disp = Display {
        bounding_box_value: disp.bounding_box(),
        disp: Arc::new(Mutex::new(disp)),
    };
    disp.init().unwrap();
    disp.clear();
    disp.flush().unwrap();

    let mut dht = PinDriver::input_output(p.pins.gpio19).unwrap();
    dht.set_high().unwrap();

    let temp_in_addr = 0;
    let temp_out_addr = 0;

    Hardware {
        led: p.pins.gpio2,
        i2c,
        disp,
        temp_in_addr,
        temp_out_addr,
    }
}
