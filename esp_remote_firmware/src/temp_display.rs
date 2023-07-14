use std::fmt::{Debug, Display};

use embassy_time::{Duration, Instant};
use embedded_graphics::{
    mono_font::{iso_8859_1::FONT_7X14, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Line, PrimitiveStyle},
    text::{Alignment, Baseline, Text, TextStyleBuilder},
};
use log::error;
use thiserror::Error;

use crate::{
    hardware::{self, I2cDriver},
    tasks::delay_task,
};

pub async fn task_temp_display(
    delay: Duration,
    i2c: I2cDriver,
    mut disp: hardware::Display,
    in_addr: i32,
    out_addr: i32,
) {
    let sensor = TempSensor {
        i2c,
        in_addr,
        out_addr,
    };

    loop {
        let start = Instant::now();

        match sensor.read() {
            Ok(Reading { temp_in, temp_out }) => print_temp(&mut disp, temp_in, temp_out),
            Err(e) => error!("could not read temps: {e}"),
        }

        delay_task(delay, start).await;
    }
}

fn print_temp(disp: &mut hardware::Display, temp_in: impl Display, temp_out: impl Display) {
    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_7X14)
        .text_color(BinaryColor::On)
        .build();
    let left_aligned = TextStyleBuilder::new()
        .alignment(Alignment::Left)
        .baseline(Baseline::Bottom)
        .build();
    hardware::Display::clear(disp);

    Text::with_baseline("Temperature", Point::zero(), text_style, Baseline::Top)
        .draw(disp)
        .unwrap();

    Line::new(Point { x: 0, y: 13 }, Point { x: 76, y: 13 })
        .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
        .draw(disp)
        .unwrap();

    Text::with_text_style(
        &format!(" In: {temp_in}°C \nOut: {temp_out}°C"),
        Point { x: 24, y: 36 },
        text_style,
        left_aligned,
    )
    .draw(disp)
    .unwrap();

    disp.flush().unwrap();
}

#[derive(Error, Debug)]
enum Error {}

struct Reading {
    temp_in: f32,
    temp_out: f32,
}

struct TempSensor {
    i2c: I2cDriver,
    in_addr: i32,
    out_addr: i32,
}

impl TempSensor {
    fn read(&self) -> Result<Reading, Error> {
        Ok(Reading {
            temp_in: 25.0,
            temp_out: 30.0,
        })
    }
}
