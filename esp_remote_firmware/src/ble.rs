use std::fmt::Error;

use esp32_nimble::BLEDevice;

pub fn start_advertising(ble: &'static BLEDevice) {
    log::info!("starting advertising");
    let ble_advertising = ble.get_advertising();
    ble_advertising.name("AiO_Rust").start().unwrap();
}

pub(crate) fn setup() -> Result<(), Error> {
    let ble_device = BLEDevice::take();
    start_advertising(ble_device);
    return Ok(());
}
