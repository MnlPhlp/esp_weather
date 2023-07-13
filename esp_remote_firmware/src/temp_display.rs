use std::fmt::Display;

use anyhow::Error;
use dht_sensor::*;
use embassy_time::{Duration, Instant, Timer};
use embedded_graphics::{
    mono_font::{iso_8859_1::FONT_7X14, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Alignment, Baseline, Text, TextStyleBuilder},
};
use esp_idf_hal::i2c::I2cDriver;
use log::error;
use sh1106::{prelude::*, Builder};

use crate::tasks::delay_task;

pub async fn task_temp_display<E>(
    delay: Duration,
    i2c: I2cDriver<'static>,
    mut sensor: impl InputOutputPin<E>,
) {
    let mut disp: GraphicsMode<_> = Builder::new()
        .with_size(DisplaySize::Display128x64)
        .connect_i2c(i2c)
        .into();
    disp.init().unwrap();

    // pull the sensor hight in the beginning
    sensor
        .set_high()
        .map_err(|_| Error::msg("Error setting sensor pin high"))
        .unwrap();
    Timer::after(Duration::from_secs(1)).await;

    loop {
        let start = Instant::now();

        let (temp, hum) = read_sensor(&mut sensor);
        print_temp(&mut disp, temp, hum);

        delay_task(delay, start).await;
    }
}

fn print_temp(
    disp: &mut GraphicsMode<I2cInterface<I2cDriver>>,
    temp: impl Display,
    hum: impl Display,
) {
    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_7X14)
        .text_color(BinaryColor::On)
        .build();
    let left_aligned = TextStyleBuilder::new()
        .alignment(Alignment::Left)
        .baseline(Baseline::Bottom)
        .build();
    disp.clear();

    Text::with_text_style(
        &format!("Temp: {temp}°C \n Hum: {hum}%"),
        Point { x: 24, y: 32 },
        text_style,
        left_aligned,
    )
    .draw(disp)
    .unwrap();

    disp.flush().unwrap();
}

struct DelayU8;

impl embedded_hal::blocking::delay::DelayMs<u8> for DelayU8 {
    fn delay_ms(&mut self, ms: u8) {
        esp_idf_hal::delay::FreeRtos::delay_ms(ms as u32);
    }
}
impl embedded_hal::blocking::delay::DelayUs<u8> for DelayU8 {
    fn delay_us(&mut self, us: u8) {
        esp_idf_hal::delay::FreeRtos::delay_us(us as u32);
    }
}

fn read_sensor<E>(sensor: &mut impl InputOutputPin<E>) -> (f32, f32) {
    match dht22::Reading::read(&mut DelayU8, sensor) {
        Ok(reading) => (reading.temperature, reading.relative_humidity),
        Err(_) => {
            error!("Error reading temp/hum sensor");
            (0.0, 0.0)
        }
    }
}
