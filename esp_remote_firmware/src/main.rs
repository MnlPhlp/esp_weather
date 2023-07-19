mod ble;
mod hardware;
mod tasks;
mod temp_display;

use anyhow::Result;
use esp_idf_sys as _;
use esp_remote_common::state::State;
use lazy_static::lazy_static;

lazy_static! {
    static ref STATE: State = State::default();
}

fn main() -> Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();
    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();
    // setup timer
    esp_idf_svc::timer::embassy_time::driver::link();

    // start running tasks on async executor
    tasks::setup().unwrap();

    panic!("Executor exited");
}
