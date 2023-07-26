#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

extern crate alloc;

use embassy_executor::Executor;
use esp_backtrace as _;
use hal::{embassy, prelude::*};
use log::info;
use static_cell::StaticCell;

mod ble;
mod hardware;
mod tasks;

#[global_allocator]
static ALLOCATOR: esp_alloc::EspHeap = esp_alloc::EspHeap::empty();

fn init_heap() {
    const HEAP_SIZE: usize = 32 * 1024;

    extern "C" {
        static mut _heap_start: u32;
        static mut _heap_end: u32;
    }

    unsafe {
        let heap_start = &_heap_start as *const _ as usize;
        let heap_end = &_heap_end as *const _ as usize;
        assert!(
            heap_end - heap_start > HEAP_SIZE,
            "Not enough available heap memory."
        );
        ALLOCATOR.init(heap_start as *mut u8, HEAP_SIZE);
    }
}

static EXECUTOR: StaticCell<Executor> = StaticCell::new();

#[entry]
fn main() -> ! {
    init_heap();

    esp_println::logger::init_logger_from_env();
    info!("Logger is setup");

    let (hw, timer0) = hardware::get_hardware();
    embassy::init(&hw.clocks, timer0);

    let executor = EXECUTOR.init(Executor::new());
    executor.run(|spawner| {
        tasks::setup(spawner, hw);
    });
}
