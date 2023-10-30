#![allow(
    non_camel_case_types,
    unused,
    clippy::redundant_closure,
    clippy::useless_conversion,
    clippy::unit_arg,
    clippy::double_parens,
    non_snake_case,
    clippy::too_many_arguments
)]
// AUTO GENERATED FILE, DO NOT EDIT.
// Generated by `flutter_rust_bridge`@ 1.82.3.

use crate::api::*;
use core::panic::UnwindSafe;
use flutter_rust_bridge::rust2dart::IntoIntoDart;
use flutter_rust_bridge::*;
use std::ffi::c_void;
use std::sync::Arc;

// Section: imports

// Section: wire functions

fn wire_ble_discover_impl(port_: MessagePort, timeout: impl Wire2Api<u64> + UnwindSafe) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, (), _>(
        WrapInfo {
            debug_name: "ble_discover",
            port: Some(port_),
            mode: FfiCallMode::Stream,
        },
        move || {
            let api_timeout = timeout.wire2api();
            move |task_callback| {
                Result::<_, ()>::Ok(ble_discover(
                    task_callback.stream_sink::<_, Vec<mirror_BleDevice>>(),
                    api_timeout,
                ))
            }
        },
    )
}
fn wire_ble_connect_impl(port_: MessagePort, address: impl Wire2Api<BleAddress> + UnwindSafe) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, (), _>(
        WrapInfo {
            debug_name: "ble_connect",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_address = address.wire2api();
            move |task_callback| Result::<_, ()>::Ok(ble_connect(api_address))
        },
    )
}
fn wire_ble_disconnect_impl(port_: MessagePort) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, (), _>(
        WrapInfo {
            debug_name: "ble_disconnect",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || move |task_callback| Result::<_, ()>::Ok(ble_disconnect()),
    )
}
fn wire_read_state_impl(port_: MessagePort) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, Option<AppState>, _>(
        WrapInfo {
            debug_name: "read_state",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || move |task_callback| Result::<_, ()>::Ok(read_state()),
    )
}
fn wire_init_impl(port_: MessagePort) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, (), _>(
        WrapInfo {
            debug_name: "init",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || move |task_callback| Result::<_, ()>::Ok(init()),
    )
}
fn wire_create_log_stream_impl(port_: MessagePort) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, (), _>(
        WrapInfo {
            debug_name: "create_log_stream",
            port: Some(port_),
            mode: FfiCallMode::Stream,
        },
        move || {
            move |task_callback| {
                Result::<_, ()>::Ok(create_log_stream(
                    task_callback.stream_sink::<_, mirror_LogEntry>(),
                ))
            }
        },
    )
}
// Section: wrapper structs

#[derive(Clone)]
pub struct mirror_BleAddress(BleAddress);

#[derive(Clone)]
pub struct mirror_BleDevice(BleDevice);

#[derive(Clone)]
pub struct mirror_Level(Level);

#[derive(Clone)]
pub struct mirror_LogEntry(LogEntry);

#[derive(Clone)]
pub struct mirror_SensorState(SensorState);

// Section: static checks

const _: fn() = || {
    {
        let BleAddress = None::<BleAddress>.unwrap();
        let _: [u8; 6] = BleAddress.address;
    }
    {
        let BleDevice = None::<BleDevice>.unwrap();
        let _: BleAddress = BleDevice.address;
        let _: String = BleDevice.name;
        let _: bool = BleDevice.is_connected;
    }
    match None::<Level>.unwrap() {
        Level::Error => {}
        Level::Warn => {}
        Level::Info => {}
        Level::Debug => {}
        Level::Trace => {}
    }
    {
        let LogEntry = None::<LogEntry>.unwrap();
        let _: i64 = LogEntry.time_millis;
        let _: String = LogEntry.msg;
        let _: Level = LogEntry.log_level;
        let _: String = LogEntry.lbl;
    }
    {
        let SensorState = None::<SensorState>.unwrap();
        let _: f32 = SensorState.temp_in;
        let _: f32 = SensorState.temp_out;
        let _: f32 = SensorState.hum_in;
        let _: f32 = SensorState.hum_out;
        let _: u16 = SensorState.tvoc_ppb;
        let _: u16 = SensorState.co2_ppm;
    }
};
// Section: allocate functions

// Section: related functions

// Section: impl Wire2Api

pub trait Wire2Api<T> {
    fn wire2api(self) -> T;
}

impl<T, S> Wire2Api<Option<T>> for *mut S
where
    *mut S: Wire2Api<T>,
{
    fn wire2api(self) -> Option<T> {
        (!self.is_null()).then(|| self.wire2api())
    }
}

impl Wire2Api<u64> for u64 {
    fn wire2api(self) -> u64 {
        self
    }
}
impl Wire2Api<u8> for u8 {
    fn wire2api(self) -> u8 {
        self
    }
}

// Section: impl IntoDart

impl support::IntoDart for AppState {
    fn into_dart(self) -> support::DartAbi {
        vec![self.sensors.into_into_dart().into_dart()].into_dart()
    }
}
impl support::IntoDartExceptPrimitive for AppState {}
impl rust2dart::IntoIntoDart<AppState> for AppState {
    fn into_into_dart(self) -> Self {
        self
    }
}

impl support::IntoDart for mirror_BleAddress {
    fn into_dart(self) -> support::DartAbi {
        vec![self.0.address.into_into_dart().into_dart()].into_dart()
    }
}
impl support::IntoDartExceptPrimitive for mirror_BleAddress {}
impl rust2dart::IntoIntoDart<mirror_BleAddress> for BleAddress {
    fn into_into_dart(self) -> mirror_BleAddress {
        mirror_BleAddress(self)
    }
}

impl support::IntoDart for mirror_BleDevice {
    fn into_dart(self) -> support::DartAbi {
        vec![
            self.0.address.into_into_dart().into_dart(),
            self.0.name.into_into_dart().into_dart(),
            self.0.is_connected.into_into_dart().into_dart(),
        ]
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for mirror_BleDevice {}
impl rust2dart::IntoIntoDart<mirror_BleDevice> for BleDevice {
    fn into_into_dart(self) -> mirror_BleDevice {
        mirror_BleDevice(self)
    }
}

impl support::IntoDart for mirror_Level {
    fn into_dart(self) -> support::DartAbi {
        match self.0 {
            Level::Error => 0,
            Level::Warn => 1,
            Level::Info => 2,
            Level::Debug => 3,
            Level::Trace => 4,
        }
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for mirror_Level {}
impl rust2dart::IntoIntoDart<mirror_Level> for Level {
    fn into_into_dart(self) -> mirror_Level {
        mirror_Level(self)
    }
}

impl support::IntoDart for mirror_LogEntry {
    fn into_dart(self) -> support::DartAbi {
        vec![
            self.0.time_millis.into_into_dart().into_dart(),
            self.0.msg.into_into_dart().into_dart(),
            self.0.log_level.into_into_dart().into_dart(),
            self.0.lbl.into_into_dart().into_dart(),
        ]
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for mirror_LogEntry {}
impl rust2dart::IntoIntoDart<mirror_LogEntry> for LogEntry {
    fn into_into_dart(self) -> mirror_LogEntry {
        mirror_LogEntry(self)
    }
}

impl support::IntoDart for mirror_SensorState {
    fn into_dart(self) -> support::DartAbi {
        vec![
            self.0.temp_in.into_into_dart().into_dart(),
            self.0.temp_out.into_into_dart().into_dart(),
            self.0.hum_in.into_into_dart().into_dart(),
            self.0.hum_out.into_into_dart().into_dart(),
            self.0.tvoc_ppb.into_into_dart().into_dart(),
            self.0.co2_ppm.into_into_dart().into_dart(),
        ]
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for mirror_SensorState {}
impl rust2dart::IntoIntoDart<mirror_SensorState> for SensorState {
    fn into_into_dart(self) -> mirror_SensorState {
        mirror_SensorState(self)
    }
}

// Section: executor

support::lazy_static! {
    pub static ref FLUTTER_RUST_BRIDGE_HANDLER: support::DefaultHandler = Default::default();
}

#[cfg(not(target_family = "wasm"))]
#[path = "bridge_generated.io.rs"]
mod io;
#[cfg(not(target_family = "wasm"))]
pub use io::*;
