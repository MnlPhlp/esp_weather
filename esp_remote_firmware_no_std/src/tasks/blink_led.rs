use embassy_time::Instant;
use hal::prelude::*;

use crate::hardware::Led;

use super::{delay_task, DELAYS};

#[embassy_executor::task]
pub async fn run(mut led: Led) {
    let mut last_wake = Instant::now();
    loop {
        delay_task(DELAYS.blink_led, &mut last_wake).await;
        led.toggle().unwrap();
    }
}
