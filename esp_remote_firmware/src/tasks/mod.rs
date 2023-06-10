// this module contains functions for long-running async tasks
mod blink_led;

use embassy_time::{Duration, Instant, Timer};

use anyhow::{Error, Result};
use esp_idf_hal::{
    prelude::Peripherals,
    task::executor::{EspExecutor, Local},
};

/// Delay in ms each task shold wait for after running once
struct Delays {
    blink_led: Duration,
}

static DELAYS: Delays = Delays {
    blink_led: Duration::from_millis(500),
};

/// delay the task for given duration.
/// start parameter should be start of the task to subtract runtime from delay.
pub async fn delay_task(task_delay: Duration, start: Instant) {
    let elapsed = start.elapsed();
    if elapsed < task_delay {
        Timer::after(task_delay - elapsed).await;
    } else {
        log::warn!(
            "Took longer than specified taskdelay: {} ms",
            elapsed.as_micros() as f32 / 1000.0
        );
    }
}

pub(crate) fn setup() -> Result<()> {
    log::info!(
        "starting task-runner on core {:?}",
        esp_idf_hal::cpu::core()
    );

    // create executor for async tasks
    let executor = EspExecutor::<16, Local>::new();

    let p = Peripherals::take().ok_or(Error::msg("could not take Peripherals"))?;

    executor.spawn_detached(blink_led::task_blink_led(DELAYS.blink_led, p.pins.gpio2))?;

    // start task Execution
    executor.run(|| true);
    Ok(())
}
