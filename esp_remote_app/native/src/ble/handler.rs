use anyhow::Result;
use btleplug::api::{
    Central, Characteristic, Manager as _, Peripheral as _, ScanFilter, WriteType,
};
use btleplug::platform::{Adapter, Manager, Peripheral};
use esp_remote_common::{CMD_UUID, SERVICE_UUID};
use flutter_rust_bridge::StreamSink;
use std::collections::HashMap;
use std::time::Duration;
use tokio::time::sleep;
use uuid::{uuid, Uuid};

use super::{BleDevice, Error};
use crate::logger::log;

pub struct BleHandler {
    connected: Option<Peripheral>,
    cmd_charac: Option<Characteristic>,
    devices: HashMap<String, Peripheral>,
    adapter: Adapter,
}

impl BleHandler {
    pub async fn new() -> Result<Self> {
        let manager = Manager::new().await?;
        let adapters = manager.adapters().await?;
        let central = adapters.into_iter().next().expect("cannot fail");
        Ok(Self {
            devices: HashMap::new(),
            connected: None,
            cmd_charac: None,
            adapter: central,
        })
    }

    pub async fn connect(&mut self, address: String) -> Result<()> {
        log(format!("Trying to connect to: {address}"));
        let device = self
            .devices
            .get(&address)
            .ok_or(Error::UnknownPeripheral(address))?;
        // discover service and characteristics needed
        // return with error if something is not found
        device.connect().await?;
        device.discover_services().await?;
        let services = device.services();
        log(format!("Services: {:?}", services));
        let service = services
            .iter()
            .find(|s| s.uuid == SERVICE_UUID)
            .ok_or(Error::ServiceNotFound)?;
        log(format!("Characteristics: {:?}", service.characteristics));
        self.cmd_charac = find_charac(service, CMD_UUID)?;
        self.connected = Some(device.clone());
        Ok(())
    }

    pub async fn disconnect(&mut self) -> Result<()> {
        log("disconnecting");
        if let Some(dev) = self.connected.as_mut() {
            dev.disconnect().await?;
        }
        Ok(())
    }

    /// Scans for [timeout] milliseconds and returns vector with all discovered devices
    pub async fn discover(&mut self, sink: StreamSink<Vec<BleDevice>>, timeout: u64) -> Result<()> {
        log("Starting discovery");
        self.adapter.start_scan(ScanFilter::default()).await?;
        self.devices.clear();
        let loops = timeout / 500;
        for _ in 0..loops {
            sleep(Duration::from_millis(500)).await;
            let discovered = self.adapter.peripherals().await?;
            log(format!("discovered.len() => {}", discovered.len()));
            let devices = self.add_devices(discovered).await;
            if devices.len() > 0 {
                sink.add(devices);
            }
        }
        self.adapter.stop_scan().await?;
        log(format!("discovered {} devices", self.devices.len()));
        log("scan stopped");
        return Ok(());
    }

    async fn add_devices(&mut self, discovered: Vec<Peripheral>) -> Vec<BleDevice> {
        let mut devices = vec![];
        for p in discovered {
            if let Ok(dev) = BleDevice::from_peripheral(&p).await {
                self.devices.insert(dev.address.clone(), p);
                devices.push(dev);
            }
        }
        devices.sort();
        return devices;
    }

    pub async fn send_data(&mut self, data: Vec<u8>) -> Result<()> {
        if let Some(dev) = self.connected.as_mut() {
            if let Some(cmd_charac) = &self.cmd_charac {
                dev.write(cmd_charac, &data, WriteType::WithoutResponse)
                    .await?;
            }
        }
        Ok(())
    }
}

fn find_charac(service: &btleplug::api::Service, uuid: Uuid) -> Result<Option<Characteristic>> {
    match service.characteristics.iter().find(|c| c.uuid == uuid) {
        Some(c) => Ok(Some(c.clone())),
        None => Err(Error::CMDNotFound.into()),
    }
}
