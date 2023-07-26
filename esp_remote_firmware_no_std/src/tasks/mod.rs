use alloc::boxed::Box;
use embassy_time::{Duration, Instant, Timer};

use crate::{
    ble::{self, BleHandler},
    hardware::Hardware,
};

mod blink_led;
mod temp_display;

/// Delay in ms each task shold wait for after running once
pub struct Delays {
    blink_led: Duration,
    temp_display: Duration,
}

pub static DELAYS: Delays = Delays {
    blink_led: Duration::from_millis(500),
    temp_display: Duration::from_secs(5),
};

/// delay the task for given duration.
/// start parameter should be start of the task to subtract runtime from delay.
pub async fn delay_task(task_delay: Duration, start: &mut Instant) {
    let elapsed = start.elapsed();
    if elapsed < task_delay {
        Timer::after(task_delay - elapsed).await;
    } else {
        log::warn!(
            "Took longer than specified taskdelay: {} ms",
            elapsed.as_micros() as f32 / 1000.0
        );
    }
    *start = Instant::now();
}

pub(crate) fn setup(spawner: embassy_executor::Spawner, hw: Hardware) {
    let ble_handler = Box::leak(Box::new(BleHandler::new(hw.ble)));
    spawner.spawn(ble::setup(ble_handler)).unwrap();
    spawner.spawn(blink_led::run(hw.led)).unwrap();
    spawner
        .spawn(temp_display::run(hw.disp, hw.lm75, hw.dht11))
        .unwrap();
}
