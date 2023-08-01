use anyhow::Error;
use esp_idf_hal::gpio::{self, Gpio16, Gpio4, InputOutput, PinDriver};
use esp_idf_hal::prelude::*;
use log::error;
use sh1106::prelude::*;

pub use self::display::Display;
pub use self::i2c::I2cDriver;
mod display;
mod i2c;

pub type Dht11Pin = PinDriver<'static, Gpio4, InputOutput>;
pub type Dht22Pin = PinDriver<'static, Gpio16, InputOutput>;

pub struct Hardware {
    pub led: gpio::Gpio2,
    pub i2c: I2cDriver,
    pub disp: Option<Display>,
    pub dht11_pin: Dht11Pin,
    pub dht22_pin: Dht22Pin,
}

pub fn get_hardware() -> Hardware {
    let p = Peripherals::take()
        .ok_or(Error::msg("could not take Peripherals"))
        .unwrap();

    let i2c = p.i2c0;
    let sda = p.pins.gpio21;
    let scl = p.pins.gpio22;

    let i2c = I2cDriver::new(i2c, sda, scl);

    let disp: GraphicsMode<_> = sh1106::Builder::new()
        .with_size(DisplaySize::Display128x64)
        .connect_i2c(i2c.clone())
        .into();
    let disp = Display::new(disp);
    let disp = match disp.init() {
        Ok(_) => {
            disp.clear();
            disp.flush().unwrap();
            Some(disp)
        }
        Err(sh1106::Error::Comm(e)) => {
            error!("could not connect to display: {e}");
            None
        }
        Err(sh1106::Error::Pin(_)) => {
            error!("could not set pins");
            None
        }
    };

    let dht11_pin = PinDriver::input_output_od(p.pins.gpio4).unwrap();
    let dht22_pin = PinDriver::input_output_od(p.pins.gpio16).unwrap();

    Hardware {
        led: p.pins.gpio2,
        i2c,
        disp,
        dht11_pin,
        dht22_pin,
    }
}
