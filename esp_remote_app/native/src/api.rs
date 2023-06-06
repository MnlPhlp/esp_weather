// This is the entry point of your Rust library.
// When adding new code to your project, note that only items used
// here will be transformed to their Dart equivalents.

use anyhow::Result;
use flutter_rust_bridge::StreamSink;

use crate::ble;
use crate::logger;
use crate::logger::log;

/// Scans for [timeout] milliseconds and returns vector with all discovered devices
pub fn ble_discover(timeout: u64) -> Vec<ble::BleDevice> {
    log("ble_discover");
    match ble::block_on(ble::discover(timeout)) {
        Ok(devices) => return devices,
        Err(e) => {
            log(format!("Error during discovering: {e}"));
            return vec![];
        }
    }
}

pub fn ble_connect(id: String) {
    ble::block_on(ble::connect(id)).unwrap()
}

pub fn ble_disconnect(id: String) {
    ble::block_on(ble::disconnect()).unwrap()
}

#[tokio::main]
pub async fn init() {
    log("starting init");
    match ble::init().await {
        Ok(_) => log("init done"),
        Err(e) => log(format!("Error while running init: {e}")),
    }
}

pub fn log_test() {
    log("Hello World!");
}

pub fn create_log_stream(s: StreamSink<logger::LogEntry>) {
    logger::create_log_stream(s);
}
