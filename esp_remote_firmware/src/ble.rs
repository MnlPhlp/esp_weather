use std::fmt::Error;

use esp32_nimble::{utilities::BleUuid, uuid128, BLEDevice, NimbleProperties};
use esp_remote_common::{CMD_UUID, SERVICE_UUID};

pub fn start_advertising(ble: &'static BLEDevice) {
    log::info!("starting advertising");
    let ble_advertising = ble.get_advertising();
    ble_advertising.name("AiO_Rust").start().unwrap();
}

pub(crate) fn setup() -> Result<(), Error> {
    let ble_device = BLEDevice::take();
    let service = ble_device
        .get_server()
        .create_service(BleUuid::from_uuid128_string(SERVICE_UUID).unwrap());
    println!("created service");
    let mut service = service.lock();
    let charac = service.create_characteristic(
        BleUuid::from_uuid128_string(CMD_UUID).unwrap(),
        NimbleProperties::READ | NimbleProperties::WRITE | NimbleProperties::WRITE_NO_RSP,
    );
    println!("Created charac");
    charac.lock().on_write(move |val, _| {
        println!("Wrote {:?}", val);
    });
    start_advertising(ble_device);
    println!("Started advertising");
    return Ok(());
}
