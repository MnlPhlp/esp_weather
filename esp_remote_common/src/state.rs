use const_default::ConstDefault;
use paste::paste;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

/// Creates getter and setter methods for a state field
macro_rules! get_set {
    ($name:ident, $type:ty) => {
        pub fn $name(&self) -> $type {
            self.$name.load(Ordering::Relaxed)
        }

        paste! {
            pub fn [<set_ $name>](&self, val: $type) {
                self.$name.store(val,Ordering::Relaxed);
            }
        }
    };
}

#[derive(ConstDefault)]
pub struct State {
    bt_connected: AtomicBool,
}
impl State {
    get_set!(bt_connected, bool);

    /// Create the default state
    pub const fn default() -> Self {
        Self::DEFAULT
    }
}
