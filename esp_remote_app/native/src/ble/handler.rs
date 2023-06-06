use anyhow::Result;
use btleplug::api::{Central, Manager as _, Peripheral as _, ScanFilter};
use btleplug::platform::{Adapter, Manager, Peripheral};
use flutter_rust_bridge::StreamSink;
use std::collections::HashMap;
use std::time::Duration;
use tokio::time::sleep;

use crate::logger::log;

use super::{BleDevice, Error};

pub struct BleHandler {
    connected: Option<Peripheral>,
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
        device.connect().await?;
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
            // dev.write(, , )
        }
        Ok(())
    }
}
