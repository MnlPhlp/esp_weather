// this module contains functions for long-running async tasks
mod blink_led;
mod display;
mod read_sensors;

use embassy_time::{Duration, Instant, Timer};

use anyhow::Result;
use esp_idf_hal::task::executor::{EspExecutor, Local};

use crate::hardware;

/// Delay in ms each task shold wait for after running once
struct Delays {
    blink_led: Duration,
    sample_sensors: Duration,
    display: Duration,
}

static DELAYS: Delays = Delays {
    blink_led: Duration::from_millis(500),
    sample_sensors: Duration::from_secs(2),
    display: Duration::from_secs(4),
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

pub(crate) fn setup() -> Result<()> {
    // create executor for async tasks
    let executor = EspExecutor::<16, Local>::new();

    let hw = hardware::get_hardware();

    executor.spawn_detached(blink_led::task_blink_led(DELAYS.blink_led, hw.led))?;

    executor.spawn_detached(read_sensors::run(
        DELAYS.sample_sensors,
        hw.dht11_pin,
        hw.dht22_pin,
    ))?;

    if let Some(disp) = hw.disp {
        executor.spawn_detached(display::run(DELAYS.display, disp))?;
    }

    // start task Execution
    executor.run(|| true);
    Ok(())
}
