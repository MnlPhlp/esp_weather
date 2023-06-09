use anyhow::{Error, Result};
use embassy_time::{Duration, Instant};
use esp_idf_hal::{
    gpio::{OutputPin, PinDriver},
    prelude::Peripherals,
};

use crate::state::BLUETOOTH_CONNECTED;

use super::delay_task;

pub async fn task_blink_led(delay: Duration, led_pin: impl OutputPin) -> Result<()> {
    let mut led = PinDriver::output(led_pin)?;
    loop {
        let start = Instant::now();
        let connected = BLUETOOTH_CONNECTED.read().unwrap().clone();
        if !connected {
            led.toggle()?;
        } else {
            led.set_high()?;
        }
        delay_task(delay, start).await;
    }
}
