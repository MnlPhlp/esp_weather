#![no_std]

use uuid::{uuid, Uuid};
pub mod state;

pub const SERVICE_UUID: Uuid = uuid!("033214d2-0ff0-4cba-814e-c5074c1ad00c");
pub const STATE_UUID: Uuid = uuid!("ac6744a7-77f3-43e9-b3c8-9955ac6bb0d4");
