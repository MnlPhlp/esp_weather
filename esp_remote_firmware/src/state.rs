// some global state variables

use std::sync::RwLock;

pub static BLUETOOTH_CONNECTED: RwLock<bool> = RwLock::new(false);
