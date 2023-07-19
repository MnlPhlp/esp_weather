use anyhow::Result;
use embassy_time::{Duration, Instant};
use esp_idf_hal::gpio::{OutputPin, PinDriver};

use crate::STATE;

use super::delay_task;

pub async fn task_blink_led(delay: Duration, led_pin: impl OutputPin) -> Result<()> {
    let mut led = PinDriver::output(led_pin)?;
    let mut start = Instant::now();
    loop {
        let connected = STATE.bt_connected();
        if !connected {
            led.toggle()?;
        } else {
            led.set_high()?;
        }
        delay_task(delay, &mut start).await;
    }
}
