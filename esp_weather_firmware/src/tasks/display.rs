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
        print_air_quality(&mut disp, values.tvoc_ppb, values.co2_ppm);
        delay_task(delay, &mut start).await;
    }
}

fn print_air_quality(disp: &mut Display, tvoc_ppb: u16, co2_ppm: u16) {
    let (text_style, left_aligned) = get_style();
    disp.clear();

    Text::with_text_style(
        &format!("Air:\n\n  TVOC: {tvoc_ppb:3} ppb\n  CO2:  {co2_ppm:3} ppm"),
        Point { x: 8, y: 16 },
        text_style,
        left_aligned,
    )
    .draw(disp)
    .unwrap();

    disp.flush().unwrap();
}

fn print_temp_hum(disp: &mut Display, label: &str, temp: f32, hum: f32) {
    let (text_style, left_aligned) = get_style();
    disp.clear();

    Text::with_text_style(
        &format!("{label}:\n\n  Temp: {temp:3.1} °C\n  Hum:  {hum:3.1} %rel."),
        Point { x: 8, y: 16 },
        text_style,
        left_aligned,
    )
    .draw(disp)
    .unwrap();

    disp.flush().unwrap();
}

fn get_style() -> (
    embedded_graphics::mono_font::MonoTextStyle<'static, BinaryColor>,
    embedded_graphics::text::TextStyle,
) {
    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_7X13)
        .text_color(BinaryColor::On)
        .build();
    let left_aligned = TextStyleBuilder::new()
        .alignment(Alignment::Left)
        .baseline(Baseline::Bottom)
        .build();
    (text_style, left_aligned)
}
