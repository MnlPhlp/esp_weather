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
// Generated by `flutter_rust_bridge`@ 1.78.0.

use crate::api::*;
use core::panic::UnwindSafe;
use flutter_rust_bridge::rust2dart::IntoIntoDart;
use flutter_rust_bridge::*;
use std::ffi::c_void;
use std::sync::Arc;

// Section: imports

use crate::logger::LogEntry;

// Section: wire functions

fn wire_ble_discover_impl(port_: MessagePort, timeout: impl Wire2Api<u64> + UnwindSafe) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, ()>(
        WrapInfo {
            debug_name: "ble_discover",
            port: Some(port_),
            mode: FfiCallMode::Stream,
        },
        move || {
            let api_timeout = timeout.wire2api();
            move |task_callback| {
                Ok(ble_discover(
                    task_callback.stream_sink::<_, Vec<mirror_BleDevice>>(),
                    api_timeout,
                ))
            }
        },
    )
}
fn wire_ble_connect_impl(port_: MessagePort, id: impl Wire2Api<String> + UnwindSafe) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, ()>(
        WrapInfo {
            debug_name: "ble_connect",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_id = id.wire2api();
            move |task_callback| Ok(ble_connect(api_id))
        },
    )
}
fn wire_ble_disconnect_impl(port_: MessagePort) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, ()>(
        WrapInfo {
            debug_name: "ble_disconnect",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || move |task_callback| Ok(ble_disconnect()),
    )
}
fn wire_read_state_impl(port_: MessagePort) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, Option<AppState>>(
        WrapInfo {
            debug_name: "read_state",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || move |task_callback| Ok(read_state()),
    )
}
fn wire_init_impl(port_: MessagePort) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, ()>(
        WrapInfo {
            debug_name: "init",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || move |task_callback| Ok(init()),
    )
}
fn wire_log_test_impl(port_: MessagePort) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, ()>(
        WrapInfo {
            debug_name: "log_test",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || move |task_callback| Ok(log_test()),
    )
}
fn wire_create_log_stream_impl(port_: MessagePort) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, ()>(
        WrapInfo {
            debug_name: "create_log_stream",
            port: Some(port_),
            mode: FfiCallMode::Stream,
        },
        move || {
            move |task_callback| {
                Ok(create_log_stream(
                    task_callback.stream_sink::<_, LogEntry>(),
                ))
            }
        },
    )
}
// Section: wrapper structs

#[derive(Clone)]
pub struct mirror_BleDevice(BleDevice);

// Section: static checks

const _: fn() = || {
    let BleDevice = None::<BleDevice>.unwrap();
    let _: String = BleDevice.address;
    let _: String = BleDevice.name;
    let _: bool = BleDevice.is_connected;
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

impl support::IntoDart for LogEntry {
    fn into_dart(self) -> support::DartAbi {
        vec![
            self.time_millis.into_into_dart().into_dart(),
            self.msg.into_into_dart().into_dart(),
        ]
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for LogEntry {}
impl rust2dart::IntoIntoDart<LogEntry> for LogEntry {
    fn into_into_dart(self) -> Self {
        self
    }
}

impl support::IntoDart for SensorValues {
    fn into_dart(self) -> support::DartAbi {
        vec![
            self.temp_in.into_into_dart().into_dart(),
            self.temp_out.into_into_dart().into_dart(),
        ]
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for SensorValues {}
impl rust2dart::IntoIntoDart<SensorValues> for SensorValues {
    fn into_into_dart(self) -> Self {
        self
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
