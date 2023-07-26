use alloc::format;
use dht11::Measurement;
use embassy_time::Instant;
use embedded_graphics::{
    mono_font::{iso_8859_1::FONT_7X14, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Alignment, Baseline, Text, TextStyleBuilder},
};
use log::{error, info};

use super::{delay_task, DELAYS};
use crate::hardware::{Dht11, Display, Lm75};

#[embassy_executor::task]
pub async fn run(mut disp: Option<Display>, mut temp_sensor: Lm75, mut temp_hum_sensor: Dht11) {
    if let Err(e) = temp_sensor.enable() {
        error!("Error enabling i2c temp sensor: {e:?}")
    }
    let mut last_wake = Instant::now();
    let mut temp_out = 0.0;
    let mut reading = Measurement::default();
    loop {
        delay_task(DELAYS.temp_display, &mut last_wake).await;

        temp_out = read_temp(&mut temp_sensor, temp_out);
        reading = read_temp_hum(&mut temp_hum_sensor, reading);
        info!("temp: {temp_out}, reading: {reading:?}");
        let temp_in = reading.temperature as f32 / 10.0;
        let hum_in = reading.humidity as f32 / 10.0;

        if let Some(disp) = &mut disp {
            print_temp(disp, temp_out, temp_in, hum_in);
        }
    }
}

fn read_temp_hum(temp_hum_sensor: &mut Dht11, temp_hum: Measurement) -> Measurement {
    match temp_hum_sensor.perform_measurement(&mut embassy_time::Delay) {
        Ok(val) => val,
        Err(e) => {
            error!("could not read temp_hum: {e:?}");
            temp_hum
        }
    }
}

fn read_temp(temp_sensor: &mut Lm75, old_temp: f32) -> f32 {
    match temp_sensor.read_temperature() {
        Ok(val) => val,
        Err(e) => {
            error!("could not read temperature: {e:?}");
            old_temp
        }
    }
}

fn print_temp(disp: &mut Display, temp_out: f32, temp_in: f32, hum_in: f32) {
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
        &format!("In:  {temp_in:.1} °C\n     {hum_in:3.1} %rel.\n\nOut: {temp_out:.1} °C"),
        Point { x: 16, y: 16 },
        text_style,
        left_aligned,
    )
    .draw(disp)
    .unwrap();

    disp.flush().unwrap();
}
