// This is the entry point of your Rust library.
// When adding new code to your project, note that only items used
// here will be transformed to their Dart equivalents.

use anyhow::Result;
use flutter_rust_bridge::StreamSink;

use crate::ble;
use crate::ble::BleDevice;
use crate::logger;
use crate::logger::log;

pub fn ble_discover(sink: StreamSink<Vec<BleDevice>>, timeout: u64) {
    ble::discover(sink, timeout).unwrap();
}

pub fn ble_connect(id: String) {
    ble::connect(id).unwrap()
}

pub fn ble_disconnect(id: String) {
    ble::disconnect().unwrap()
}

pub fn ble_send(data: Vec<u8>) {
    ble::send_data(data).unwrap()
}

pub fn init() {
    log("starting init");
    match ble::init() {
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
