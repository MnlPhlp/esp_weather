extern crate alloc;
use alloc::vec::Vec;
use bincode::{
    config::{self, Configuration},
    Decode, Encode,
};
use paste::paste;
use spin::Mutex;

/// Creates getter and setter methods for a state field
macro_rules! get_set {
    ($name:ident, $type:ty) => {
        pub fn $name(&self) -> $type {
            self.0.lock().$name.clone()
        }

        paste! {
            pub fn [<set_ $name>](&self, val: $type) {
                self.0.lock().$name = val;
            }
        }
    };
}

#[derive(Default, Clone, Encode, Decode)]
pub struct InnerState {
    bt_connected: bool,
    sensor: SensorState,
}
#[derive(Default, Clone, Encode, Decode, Debug)]
pub struct SensorState {
    pub temp_in: f32,
    pub temp_out: f32,
    pub hum_in: f32,
}

const CONFIG: Configuration = config::standard();

#[derive(Default)]
pub struct State(Mutex<InnerState>);
impl State {
    get_set!(bt_connected, bool);
    get_set!(sensor, SensorState);

    pub fn to_bytes(&self) -> Vec<u8> {
        let inner = self.0.lock();
        bincode::encode_to_vec(&*inner, CONFIG).unwrap()
    }

    pub fn from_bytes(data: &[u8]) -> Result<Self, bincode::error::DecodeError> {
        let (state, _) = bincode::decode_from_slice(data, CONFIG)?;
        Ok(Self(Mutex::new(state)))
    }
}
