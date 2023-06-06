// taken from https://github.com/trobanga/flutter_btleplug/blob/b092ef415b36e60f4bb6df0ca261efdedaaa4a7e/packages/btleplug/native/src/ble.rsuse std::collections::HashMap;

use anyhow::Result;
use btleplug::{
    api::Peripheral as _,
    platform::{Peripheral, PeripheralId},
};
use flutter_rust_bridge::StreamSink;
use once_cell::sync::OnceCell;
use tokio::sync::mpsc::{self, UnboundedSender};

mod setup;
pub use setup::*;
mod error;
pub use error::Error;
mod handler;
pub use handler::*;

static TX: OnceCell<UnboundedSender<Command>> = OnceCell::new();

enum Command {
    Connect {
        id: String,
    },
    Disconnect,
    Discover {
        sink: StreamSink<Vec<BleDevice>>,
        timeout: u64,
    },
    SendData(Vec<u8>),
}

impl std::fmt::Debug for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Self::Disconnect => "disconnect".into(),
            Self::Connect { id } => format!("connect({id})"),
            Self::Discover { .. } => "discover".into(),
            Self::SendData(_) => "send_data".into(),
        };
        f.write_str(&msg)
    }
}

/// The init() function must be called before anything else.
/// At the moment the developer has to make sure it is only called once.
pub fn init() -> Result<()> {
    create_runtime()?;
    let rt = RUNTIME.get().ok_or(Error::RuntimeNotInitialized)?;
    let (tx, mut rx) = mpsc::unbounded_channel::<Command>();
    TX.set(tx).map_err(|_| Error::TxAlreadySet)?;
    rt.spawn(async move {
        let mut handler = BleHandler::new().await.unwrap();
        while let Some(msg) = rx.recv().await {
            match msg {
                Command::Connect { id } => handler.connect(id).await.unwrap(),
                Command::Disconnect => handler.disconnect().await.unwrap(),
                Command::Discover { sink, timeout } => {
                    handler.discover(sink, timeout).await.unwrap()
                }
                Command::SendData(data) => handler.send_data(data).await.unwrap(),
            }
        }
    });
    Ok(())
}

/// This is the BleDevice intended to show in Dart/Flutter
#[derive(Debug, Clone, Eq, Ord)]
pub struct BleDevice {
    pub address: String,
    pub name: String,
}

impl BleDevice {
    async fn from_peripheral(peripheral: &Peripheral) -> Result<Self> {
        Ok(Self {
            address: peripheral.id().to_string(),
            name: peripheral
                .properties()
                .await?
                .unwrap_or_default()
                .local_name
                .ok_or(Error::UnknownPeripheral(peripheral.id().to_string()))?,
        })
    }
}

impl PartialEq for BleDevice {
    fn eq(&self, other: &Self) -> bool {
        self.address.eq(&other.address)
    }
}

impl PartialOrd for BleDevice {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return Some(self.address.cmp(&other.address));
    }
}

fn send(cmd: Command) -> Result<()> {
    let tx = TX.get().ok_or(Error::TxNotInitialized)?;
    tx.send(cmd)?;
    Ok(())
}

pub fn connect(id: String) -> Result<()> {
    return send(Command::Connect { id });
}

pub fn disconnect() -> Result<()> {
    return send(Command::Disconnect);
}

pub fn discover(sink: StreamSink<Vec<BleDevice>>, timeout: u64) -> Result<()> {
    return send(Command::Discover { sink, timeout });
}

pub fn send_data(data: Vec<u8>) -> Result<()> {
    return send(Command::SendData(data));
}
