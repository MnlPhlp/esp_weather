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
use log::info;
use log::LevelFilter;
use tokio::sync::mpsc;

pub use blec::BleDevice;
pub use flutter_logger::LogEntry;
use log::error;
pub use log::Level;

pub fn ble_discover(sink: StreamSink<Vec<BleDevice>>, timeout: u64) {
    info!("discovering");
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
    info!("reading");
    let start = Instant::now();
    let data = block_on(blec::recv_data(STATE_UUID))?;
    info!("received {} bytes in {:?}", data.len(), start.elapsed());
    let state = State::from_bytes(&data).map_err(|e| anyhow!("error reading state: {e}"))?;
    Ok(AppState {
        sensors: state.sensors(),
    })
}

pub fn read_state() -> Option<AppState> {
    match read_state_inner() {
        Ok(val) => Some(val),
        Err(e) => {
            error!("Error reading state: {e}");
            None
        }
    }
}

pub fn init() {
    info!("starting init");
    match blec::init() {
        Ok(_) => info!("init done"),
        Err(e) => error!("Error while running init: {e}"),
    }
}

pub fn create_log_stream(s: StreamSink<LogEntry>) {
    flutter_logger::init(s, LevelFilter::Info).unwrap();
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
    hum_out: f32,
}

#[frb(mirror(LogEntry))]
struct _LogEntry {
    time_millis: i64,
    msg: String,
    log_level: Level,
    lbl: String,
}

#[frb(mirror(Level))]
enum _Level {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}
