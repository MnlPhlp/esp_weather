mod ble;
mod state;
mod tasks;

use anyhow::Result;
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use std::thread;

fn main() -> Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();
    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();
    // setup timer
    esp_idf_svc::timer::embassy_time::driver::link();

    // setup ble and start advertising
    ble::setup()?;

    // start running tasks on async executor
    let executor = thread::Builder::new()
        .name("task-executor".into())
        .spawn(tasks::setup)
        .expect("Thread should be crated");

    executor.join().unwrap().unwrap();
    panic!("Executor Thread exited");
}
