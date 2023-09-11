use std::{
    convert::Infallible,
    sync::{Arc, Mutex},
};

use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::{Dimensions, DrawTarget},
    primitives, Pixel,
};
use esp_idf_sys::EspError;
use sh1106::prelude::*;

use super::i2c::I2cDriver;

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

    pub fn new(disp: GraphicsMode<I2cInterface<I2cDriver>>) -> Self {
        Self {
            bounding_box_value: disp.bounding_box(),
            disp: Arc::new(Mutex::new(disp)),
        }
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
