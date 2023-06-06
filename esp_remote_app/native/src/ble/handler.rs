use anyhow::Result;
use btleplug::api::{Central, Manager as _, Peripheral as _, ScanFilter};
use btleplug::platform::{Adapter, Manager};
use std::collections::HashMap;
use std::time::Duration;
use tokio::time::sleep;

use crate::logger::log;

use super::{BleDevice, Error, Peripheral};

pub struct BleHandler {
    connected: Option<BleDevice>,
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
            adapter: central,
        })
    }

    pub async fn connect(&mut self, address: String) -> Result<()> {
        log(format!("Trying to connect to: {address}"));
        let device = self
            .devices
            .get(&address)
            .ok_or(Error::UnknownPeripheral(address))?;
        device.connect().await;
        self.connected = Some(BleDevice::from_peripheral(device).await);
        Ok(())
    }

    pub async fn disconnect(&mut self) -> Result<()> {
        log("disconnecting");
        if let Some(dev) = self.connected.as_mut() {
            let device = self
                .devices
                .get(&dev.address)
                .ok_or(Error::UnknownPeripheral(dev.address.clone()))?;
            device.disconnect().await?;
        }
        Ok(())
    }

    /// Scans for [timeout] milliseconds and returns vector with all discovered devices
    pub async fn discover(&self, timeout: u64) -> Result<Vec<BleDevice>> {
        log("Starting discovery");
        self.adapter.start_scan(ScanFilter::default()).await?;
        log("scan started");
        sleep(Duration::from_millis(timeout)).await;
        log("sleep done");
        let discovered = self.adapter.peripherals().await?;
        log(format!("discovered {} devices", discovered.len()));
        self.adapter.stop_scan().await?;
        log("scan stopped");
        let mut devices = vec![];
        for dev in discovered {
            let props = dev.properties().await?.unwrap();
            devices.push(BleDevice {
                address: props.address.to_string(),
                name: props.local_name.unwrap_or("unknwon".into()),
            });
        }
        return Ok(devices);
    }
}
