// logger taken from https://github.com/trobanga/flutter_btleplug/blob/main/packages/btleplug/native/src/logger.rs
use flutter_rust_bridge::*;
use once_cell::sync::OnceCell;
use std::{sync::RwLock, time};

pub struct LogEntry {
    pub time_millis: i64,
    pub msg: String,
}

static LOGGER: OnceCell<RwLock<StreamSink<LogEntry>>> = OnceCell::new();
static START: OnceCell<time::Instant> = OnceCell::new();

pub fn create_log_stream(s: StreamSink<LogEntry>) {
    if LOGGER.get().is_none() && START.get().is_none() {
        let _ = START.set(time::Instant::now());
        let _ = LOGGER.set(RwLock::new(s));
        log("Logger ready!");
    } else {
        panic!("time to panic");
    }
}

pub fn log<S: AsRef<str>>(msg: S) {
    if let Some(logger) = LOGGER.get() {
        let logger = logger.read().unwrap();
        let start = START.get().unwrap();
        logger.add(LogEntry {
            time_millis: start.elapsed().as_millis() as i64,
            msg: msg.as_ref().to_string(),
        });
    }
}
