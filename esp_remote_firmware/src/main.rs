mod ble;
mod hardware;
mod tasks;

use std::{
    sync::{Arc, Mutex},
    thread,
};

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

    let setup_mutex = Arc::new(Mutex::new(()));
    let task_mutex = setup_mutex.clone();
    thread::Builder::new()
        .name(String::from("Initial Setup"))
        .spawn(move || {
            let lock = setup_mutex.lock();
            log::info!(
                "\n###########################\n# starting setup on {:10?} #\n###########################",
                esp_idf_hal::cpu::core()
            );
            drop(lock);
            // setup ble and start advertising
            ble::setup().unwrap();
        })
        .unwrap();

    thread::Builder::new()
        .name(String::from("Task Runner"))
        .stack_size(14000)
        .spawn(move || {
            let lock = task_mutex.lock();
            log::info!(
                "\n#################################\n# starting task-runner on {:10?} #\n#################################",
                esp_idf_hal::cpu::core()
            );
            drop(lock);
            // start running tasks on async executor
            tasks::setup().unwrap();
        })
        .unwrap()
        .join()
        .unwrap();

    panic!("Executor exited");
}
