use std::fmt::Error;

use esp32_nimble::{utilities::mutex::RawMutex, BLEDevice, BLEService, NimbleProperties};
use esp_remote_common::{CMD_UUID, SERVICE_UUID};

pub fn start_advertising(ble: &'static BLEDevice) {
    log::info!("starting advertising");
    let ble_advertising = ble.get_advertising();
    ble_advertising.name("esp_remote").start().unwrap();
}

pub(crate) fn setup() -> Result<(), Error> {
    let ble_device = BLEDevice::take();
    let service = ble_device.get_server().create_service(SERVICE_UUID.into());
    println!("created service");
    setup_service(service);
    start_advertising(ble_device);
    println!("Started advertising");
    return Ok(());
}

fn setup_service(service: std::sync::Arc<embedded_svc::utils::mutex::Mutex<RawMutex, BLEService>>) {
    let mut service = service.lock();
    let charac = service.create_characteristic(
        CMD_UUID.into(),
        NimbleProperties::READ | NimbleProperties::WRITE,
    );
    println!("Created charac");
    charac.lock().on_write(move |val, _| {
        println!("Wrote {:?}", val);
    });
}
