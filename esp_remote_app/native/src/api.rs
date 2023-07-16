// This is the entry point of your Rust library.
// When adding new code to your project, note that only items used
// here will be transformed to their Dart equivalents.

use anyhow::Result;
use esp_remote_common::state::SensorState;
pub use esp_remote_common::state::State;
use esp_remote_common::SERVICE_UUID;
use esp_remote_common::STATE_UUID;
use flutter_rust_bridge::*;
use futures::executor::block_on;
use tokio::sync::mpsc;

use crate::logger;
use crate::logger::log;
use btleplug_helper::ble;
pub use btleplug_helper::BleDevice;
use log::error;

pub fn ble_discover(sink: StreamSink<Vec<BleDevice>>, timeout: u64) {
    logger::log("discovering");
    let (tx, mut rx) = mpsc::channel(1);
    ble::discover(tx, timeout).unwrap();
    let ret = btleplug_helper::spawn(async move {
        while let Some(devices) = rx.recv().await {
            sink.add(devices);
        }
    });
    if let Err(e) = ret {
        error!("spawning future for discover failed: {e}");
    }
}

pub fn ble_connect(id: String) {
    block_on(ble::connect(id, SERVICE_UUID, vec![STATE_UUID], None)).unwrap()
}

pub fn ble_disconnect() {
    block_on(ble::disconnect()).unwrap()
}

pub struct AppState {
    pub sensors: SensorValues,
}
pub struct SensorValues {
    pub temp_in: f32,
    pub temp_out: f32,
}

impl From<State> for AppState {
    fn from(value: State) -> Self {
        let SensorState { temp_in, temp_out } = value.sensor();
        Self {
            sensors: SensorValues { temp_in, temp_out },
        }
    }
}

fn read_state_inner() -> Result<AppState> {
    logger::log("reading");
    let data = block_on(ble::recv_data(STATE_UUID))?;
    logger::log(format!("received {} bytes", data.len()));
    let state = State::from_bytes(data)?;
    Ok(AppState::from(state))
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

// mirroring some structs

#[frb(mirror(BleDevice))]
struct _BleDevice {
    address: String,
    name: String,
    is_connected: bool,
}
