// taken from https://github.com/trobanga/flutter_btleplug/blob/b092ef415b36e60f4bb6df0ca261efdedaaa4a7e/packages/btleplug/native/src/ble.rsuse std::collections::HashMap;

use anyhow::Result;
use btleplug::{api::Peripheral as _, platform::PeripheralId};
use futures::Future;
use once_cell::sync::OnceCell;
use tokio::sync::{Mutex, MutexGuard};

mod setup;
pub use setup::*;
mod error;
pub use error::Error;
use tokio::time;
mod handler;
pub use handler::*;

/// Wrapper struct around btleplug::platform::Peripheral that adds the last_seen variable.
///
#[derive(Debug, Clone)]
struct Peripheral {
    peripheral: btleplug::platform::Peripheral,
    last_seen: time::Instant,
    is_connected: bool,
}

impl Peripheral {
    fn new(peripheral: btleplug::platform::Peripheral) -> Self {
        Self {
            peripheral,
            last_seen: time::Instant::now(),
            is_connected: false,
        }
    }

    fn id(&self) -> PeripheralId {
        self.peripheral.id()
    }

    async fn name(&self) -> Option<String> {
        if let Ok(Some(properties)) = self.peripheral.properties().await {
            properties.local_name
        } else {
            None
        }
    }

    async fn connect(&self) -> Result<()> {
        self.peripheral.connect().await?;
        Ok(())
    }

    async fn disconnect(&self) -> Result<()> {
        self.peripheral.disconnect().await?;
        Ok(())
    }

    fn is_connected(&self) -> bool {
        self.is_connected
    }
}

static HANDLER: OnceCell<Mutex<BleHandler>> = OnceCell::new();

/// The init() function must be called before anything else.
/// At the moment the developer has to make sure it is only called once.
pub async fn init() -> Result<()> {
    create_runtime()?;
    let rt = RUNTIME.get().ok_or(Error::RuntimeNotInitialized)?;
    let handler = BleHandler::new().await?;
    HANDLER
        .set(Mutex::new(handler))
        .map_err(|_| Error::HandlerNotInitialized)?;
    Ok(())
}

/// This is the BleDevice intended to show in Dart/Flutter
#[derive(Debug, Clone)]
pub struct BleDevice {
    pub address: String,
    pub name: String,
}

impl BleDevice {
    async fn from_peripheral(peripheral: &Peripheral) -> Self {
        Self {
            address: peripheral.id().to_string(),
            name: peripheral.name().await.unwrap_or_default(),
        }
    }
}

// /// Helper function to send all [BleDevice]s to Dart/Flutter.
// ///
// /// # Arguments
// ///
// /// sink: StreamSink<Vec<BleDevice>>
// ///     The StreamSink to which the Vec<BleDevice> should be sent
// ///
// /// # Return
// ///
// /// Returns false if the stream is closed.
// async fn send_devices(sink: &StreamSink<Vec<BleDevice>>) -> bool {
//     let devices = DEVICES.lock().await;
//     let mut d = vec![];
//     for device in devices.values() {
//         let dev = BleDevice::from_peripheral(&device).await;
//         d.push(dev.clone())
//     }
//     sink.add(d)
// }

// /// This function is used to scan for BLE devices and returns the results via the given stream sink.
// ///
// /// Parameters
// ///
// /// sink: StreamSink<Vec<BleDevice>> - A stream sink to which the results are send
// ///
// /// filter: Vec<String> - A vector of strings to filter the results with
// pub fn stream_discover(sink: StreamSink<Vec<BleDevice>>, filter: Vec<String>) -> Result<()> {
//     let rt = get_runtime()?;
//     rt.block_on(inner_scan(sink, filter))
// }

// async fn inner_scan(sink: StreamSink<Vec<BleDevice>>, _filter: Vec<String>) -> Result<()> {
//     let manager = Manager::new().await?;
//     let adapters = manager.adapters().await?;
//     let central = adapters.into_iter().next().expect("cannot fail");
//     let mut events = central.events().await?;

//     // start scanning for devices
//     log(format!(
//         "start scanning on {}",
//         central.adapter_info().await?
//     ));
//     central.start_scan(ScanFilter::default()).await?;

//     let mut device_send_interval = time::interval(time::Duration::from_secs(1));
//     loop {
//         tokio::select! {
//             _ = device_send_interval.tick() => {
//                 remove_stale_devices(3).await;
//                 if send_devices(&sink).await == false {
//                     break;
//                 }
//             }
//             Some(event) = events.next() => {
//                 // log(format!("{:?}", event));
//                 match event {
//                     CentralEvent::DeviceDiscovered(id) => {
//                         log(format!("DeviceDiscovered: {:?}", &id));
//                         let peripheral = central.peripheral(&id).await?;
//                         let peripheral = Peripheral::new(peripheral);
//                         let mut devices = DEVICES.lock().await;
//                         devices.insert(id.to_string(), peripheral);
//                     }
//                     CentralEvent::DeviceUpdated(id) => {
//                         let mut devices = DEVICES.lock().await;
//                         if let Some(device) = devices.get_mut(&id.to_string()) {
//                             device.last_seen = time::Instant::now();
//                         }
//                     }
//                     CentralEvent::DeviceConnected(id) => {
//                         log(format!("DeviceConnected: {:?}", id));
//                         let mut devices = DEVICES.lock().await;
//                         if let Some(device) = devices.get_mut(&id.to_string()) {
//                             device.is_connected = true;
//                         }
//                     }
//                     CentralEvent::DeviceDisconnected(id) => {
//                         log(format!("DeviceDisconnected: {:?}", id));
//                         let mut devices = DEVICES.lock().await;
//                         if let Some(device) = devices.get_mut(&id.to_string()) {
//                             device.is_connected = false;
//                         }
//                     }
//                     CentralEvent::ManufacturerDataAdvertisement {
//                         id,
//                         manufacturer_data,
//                     } => {
//                         // log(format!(
//                         //     "ManufacturerDataAdvertisement: {:?}, {:?}",
//                         //     id, manufacturer_data
//                         // ));
//                     }
//                     CentralEvent::ServiceDataAdvertisement { id, service_data } => {
//                         // log(format!(
//                         //     "ServiceDataAdvertisement: {:?}, {:?}",
//                         //     id, service_data
//                         // ));
//                     }
//                     CentralEvent::ServicesAdvertisement { id, services } => {
//                         let services: Vec<String> =
//                             services.into_iter().map(|s| s.to_short_string()).collect();
//                         // log(format!("ServicesAdvertisement: {:?}, {:?}", id, services));
//                     }
//                 }
//             }
//         }
//     }
//     log("Scan finished");
//     Ok(())
// }

pub async fn get_handler<'a>() -> Result<MutexGuard<'a, BleHandler>> {
    let h = HANDLER.get().ok_or(Error::HandlerNotInitialized)?;
    return Ok(h.lock().await);
}

pub async fn connect(id: String) -> Result<()> {
    let mut handler = get_handler().await?;
    return handler.connect(id).await;
}

pub async fn disconnect() -> Result<()> {
    let mut handler = get_handler().await?;
    return handler.disconnect().await;
}

pub async fn discover(timeout: u64) -> Result<Vec<BleDevice>> {
    let handler = get_handler().await?;
    return handler.discover(timeout).await;
}

pub fn block_on<F: Future>(f: F) -> F::Output {
    let rt = RUNTIME.get().ok_or(Error::RuntimeNotInitialized).unwrap();
    return rt.block_on(f);
}
