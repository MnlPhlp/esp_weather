use embassy_time::{Duration, Instant};
use embedded_graphics::{
    mono_font::{iso_8859_1::FONT_7X13, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::Point,
    text::{Alignment, Baseline, Text, TextStyleBuilder},
    Drawable,
};

use crate::{hardware::Display, STATE};

use super::delay_task;

pub async fn run(delay: Duration, mut disp: Display) {
    let mut start = Instant::now();
    loop {
        let values = STATE.sensors();
        print_temp_hum(&mut disp, "Inside", values.temp_in, values.hum_in);
        delay_task(delay, &mut start).await;
        print_temp_hum(&mut disp, "Outside", values.temp_out, values.hum_out);
        delay_task(delay, &mut start).await;
    }
}

fn print_temp_hum(disp: &mut Display, label: &str, temp: f32, hum: f32) {
    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_7X13)
        .text_color(BinaryColor::On)
        .build();
    let left_aligned = TextStyleBuilder::new()
        .alignment(Alignment::Left)
        .baseline(Baseline::Bottom)
        .build();
    disp.clear();

    Text::with_text_style(
        &format!("{label}:\n\n  {temp:3.1} °C\n  {hum:3.1} %rel."),
        Point { x: 16, y: 16 },
        text_style,
        left_aligned,
    )
    .draw(disp)
    .unwrap();

    disp.flush().unwrap();
}
