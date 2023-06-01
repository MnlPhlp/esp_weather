// This is the entry point of your Rust library.
// When adding new code to your project, note that only items used
// here will be transformed to their Dart equivalents.

use flutter_rust_bridge::StreamSink;
use futures::executor::block_on;

use crate::ble;
use crate::logger;
use crate::logger::log;

pub fn ble_discover() -> Vec<ble::BleDevice> {
    log("ble_discover");
    return block_on(ble::discover()).unwrap();
}

pub fn ble_stream_discover(s: StreamSink<Vec<ble::BleDevice>>, filter: Vec<String>) {
    log("ble_stream_discover");
    ble::stream_discover(s, filter);
}

pub fn ble_connect(id: String) {
    ble::connect(id).unwrap();
}

pub fn ble_disconnect(id: String) {
    ble::disconnect(id).unwrap();
}

pub fn init() {
    log("starting init");
    ble::init().unwrap();
    log("init done");
}

pub fn log_test() {
    log("Hello World!");
}

pub fn create_log_stream(s: StreamSink<logger::LogEntry>) {
    logger::create_log_stream(s);
}
