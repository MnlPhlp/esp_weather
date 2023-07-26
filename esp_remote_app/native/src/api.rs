// This is the entry point of your Rust library.
// When adding new code to your project, note that only items used
// here will be transformed to their Dart equivalents.

use std::time::Instant;

use anyhow::anyhow;
use anyhow::Result;
pub use esp_remote_common::state::{SensorState, State};
use esp_remote_common::SERVICE_UUID;
use esp_remote_common::STATE_UUID;
use flutter_rust_bridge::*;
use futures::executor::block_on;
use tokio::sync::mpsc;

use crate::logger;
use crate::logger::log;
pub use blec::BleDevice;
use log::error;

pub fn ble_discover(sink: StreamSink<Vec<BleDevice>>, timeout: u64) {
    logger::log("discovering");
    let (tx, mut rx) = mpsc::channel(1);
    blec::discover(tx, timeout).unwrap();
    let ret = blec::spawn(async move {
        while let Some(devices) = rx.recv().await {
            sink.add(devices);
        }
    });
    if let Err(e) = ret {
        error!("spawning future for discover failed: {e}");
    }
}

pub fn ble_connect(id: String) {
    block_on(blec::connect(
        id,
        SERVICE_UUID,
        vec![STATE_UUID],
        Some(|| {}),
    ))
    .unwrap()
}

pub fn ble_disconnect() {
    block_on(blec::disconnect()).unwrap()
}

pub struct AppState {
    pub sensors: SensorState,
}

fn read_state_inner() -> Result<AppState> {
    logger::log("reading");
    let start = Instant::now();
    let data = block_on(blec::recv_data(STATE_UUID))?;
    logger::log(format!(
        "received {} bytes in {:?}",
        data.len(),
        start.elapsed()
    ));
    let state = State::from_bytes(&data).map_err(|e| anyhow!("error reading state: {e}"))?;
    Ok(AppState {
        sensors: state.sensor(),
    })
}

pub fn read_state() -> Option<AppState> {
    match read_state_inner() {
        Ok(val) => Some(val),
        Err(e) => {
            logger::log(format!("Error reading state: {e}"));
            None
        }
    }
}

pub fn init() {
    log("starting init");
    match blec::init() {
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

// mirroring some structs

#[frb(mirror(BleDevice))]
struct _BleDevice {
    address: String,
    name: String,
    is_connected: bool,
}

#[frb(mirror(SensorState))]
struct _SensorState {
    temp_in: f32,
    temp_out: f32,
    hum_in: f32,
}
