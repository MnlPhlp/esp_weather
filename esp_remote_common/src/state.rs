use paste::paste;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

/// Creates getter and setter methods for a state field
macro_rules! get_set {
    ($name:ident, $type:ty) => {
        pub fn $name(&self) -> $type {
            self.0.lock().unwrap().$name.clone()
        }

        paste! {
            pub fn [<set_ $name>](&self, val: $type) {
                self.0.lock().unwrap().$name = val;
            }
        }
    };
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct InnerState {
    bt_connected: bool,
    sensor: SensorState,
}
#[derive(Default, Clone, Serialize, Deserialize)]
pub struct SensorState {
    pub temp_in: f32,
    pub temp_out: f32,
}

#[derive(Default)]
pub struct State(Mutex<InnerState>);
impl State {
    get_set!(bt_connected, bool);
    get_set!(sensor, SensorState);

    pub fn to_bytes(&self) -> Vec<u8> {
        let inner = self.0.lock().unwrap().clone();
        bincode::serialize(&inner).unwrap()
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<Self, bincode::Error> {
        bincode::deserialize(&data).and_then(|inner| Ok(Self(Mutex::new(inner))))
    }
}
