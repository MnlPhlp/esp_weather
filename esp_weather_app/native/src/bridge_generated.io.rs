use super::*;
// Section: wire functions

#[no_mangle]
pub extern "C" fn wire_ble_discover(port_: i64, timeout: u64) {
    wire_ble_discover_impl(port_, timeout)
}

#[no_mangle]
pub extern "C" fn wire_ble_connect(port_: i64, address: *mut wire_BleAddress) {
    wire_ble_connect_impl(port_, address)
}

#[no_mangle]
pub extern "C" fn wire_ble_disconnect(port_: i64) {
    wire_ble_disconnect_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_read_state(port_: i64) {
    wire_read_state_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_init(port_: i64) {
    wire_init_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_create_log_stream(port_: i64) {
    wire_create_log_stream_impl(port_)
}

// Section: allocate functions

#[no_mangle]
pub extern "C" fn new_box_autoadd_ble_address_0() -> *mut wire_BleAddress {
    support::new_leak_box_ptr(wire_BleAddress::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_uint_8_list_0(len: i32) -> *mut wire_uint_8_list {
    let ans = wire_uint_8_list {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

// Section: related functions

// Section: impl Wire2Api

impl Wire2Api<BleAddress> for wire_BleAddress {
    fn wire2api(self) -> BleAddress {
        BleAddress {
            address: self.address.wire2api(),
        }
    }
}
impl Wire2Api<BleAddress> for *mut wire_BleAddress {
    fn wire2api(self) -> BleAddress {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<BleAddress>::wire2api(*wrap).into()
    }
}

impl Wire2Api<[u8; 6]> for *mut wire_uint_8_list {
    fn wire2api(self) -> [u8; 6] {
        let vec: Vec<u8> = self.wire2api();
        support::from_vec_to_array(vec)
    }
}
impl Wire2Api<Vec<u8>> for *mut wire_uint_8_list {
    fn wire2api(self) -> Vec<u8> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}
// Section: wire structs

#[repr(C)]
#[derive(Clone)]
pub struct wire_BleAddress {
    address: *mut wire_uint_8_list,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_uint_8_list {
    ptr: *mut u8,
    len: i32,
}

// Section: impl NewWithNullPtr

pub trait NewWithNullPtr {
    fn new_with_null_ptr() -> Self;
}

impl<T> NewWithNullPtr for *mut T {
    fn new_with_null_ptr() -> Self {
        std::ptr::null_mut()
    }
}

impl NewWithNullPtr for wire_BleAddress {
    fn new_with_null_ptr() -> Self {
        Self {
            address: core::ptr::null_mut(),
        }
    }
}

impl Default for wire_BleAddress {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

// Section: sync execution mode utility

#[no_mangle]
pub extern "C" fn free_WireSyncReturn(ptr: support::WireSyncReturn) {
    unsafe {
        let _ = support::box_from_leak_ptr(ptr);
    };
}
