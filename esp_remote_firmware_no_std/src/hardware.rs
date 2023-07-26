use core::cell::RefCell;

use alloc::boxed::Box;
use bleps::asynch::Ble;
use esp_wifi::{ble::controller::asynch::BleConnector, initialize, EspWifiInitFor};
use hal::{
    clock::{ClockControl, Clocks, CpuClock},
    gpio::{GpioPin, OpenDrain, Output, PushPull},
    i2c::I2C,
    peripherals::{Peripherals, I2C0, TIMG0},
    prelude::*,
    timer::{Timer0, TimerGroup},
    Rng, Rtc, Timer, IO,
};
use log::error;
use sh1106::prelude::*;

pub(crate) fn get_hardware() -> (Hardware, Timer<Timer0<TIMG0>>) {
    let peripherals = Peripherals::take();
    let mut system = peripherals.DPORT.split();
    let clocks = ClockControl::configure(system.clock_control, CpuClock::Clock240MHz).freeze();

    // Disable the RTC and TIMG watchdog timers
    let mut rtc = Rtc::new(peripherals.RTC_CNTL);
    let timer_group0 = TimerGroup::new(
        peripherals.TIMG0,
        &clocks,
        &mut system.peripheral_clock_control,
    );
    let mut wdt0 = timer_group0.wdt;
    let timer_group1 = TimerGroup::new(
        peripherals.TIMG1,
        &clocks,
        &mut system.peripheral_clock_control,
    );
    let mut wdt1 = timer_group1.wdt;
    rtc.rwdt.disable();
    wdt0.disable();
    wdt1.disable();

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let led = io.pins.gpio2.into_push_pull_output();
    let dht11 = dht11::Dht11::new(io.pins.gpio4.into_open_drain_output());

    // Create a new peripheral object with the standard I2C clock speed
    let i2c = I2C::new(
        peripherals.I2C0,
        io.pins.gpio21,
        io.pins.gpio22,
        100u32.kHz(),
        &mut system.peripheral_clock_control,
        &clocks,
    );
    let i2c = ShareableI2c::new(i2c);
    let lm75 = lm75::Lm75::new(i2c, 0x48);

    // create and setup display
    let mut disp: GraphicsMode<_> = sh1106::Builder::new()
        .with_size(DisplaySize::Display128x64)
        .connect_i2c(i2c)
        .into();
    let disp = match disp.init() {
        Ok(_) => {
            disp.clear();
            disp.flush().unwrap();
            Some(disp)
        }
        Err(sh1106::Error::Comm(e)) => {
            error!("could not connect to display: {e:?}");
            None
        }
        Err(sh1106::Error::Pin(_)) => {
            error!("could not set pins");
            None
        }
    };

    // setup ble
    let (_, ble) = peripherals.RADIO.split();
    let timer = timer_group1.timer0;
    let init = initialize(
        EspWifiInitFor::Ble,
        timer,
        Rng::new(peripherals.RNG),
        system.radio_clock_control,
        &clocks,
    )
    .unwrap();
    let connector = BleConnector::new(&init, ble);
    let ble = Ble::new(connector, esp_wifi::current_millis);

    (
        Hardware {
            clocks,
            led,
            dht11,
            lm75,
            disp,
            ble,
        },
        timer_group0.timer0,
    )
}

pub struct ShareableI2c(RefCell<I2C<'static, I2C0>>);
impl ShareableI2c {
    fn new(i2c: I2C<'static, I2C0>) -> &'static Self {
        Box::leak(Box::new(Self(RefCell::new(i2c))))
    }
}
impl _embedded_hal_blocking_i2c_Write for &ShareableI2c {
    type Error = hal::i2c::Error;

    fn write(&mut self, address: u8, bytes: &[u8]) -> Result<(), Self::Error> {
        self.0.borrow_mut().write(address, bytes)
    }
}
impl _embedded_hal_blocking_i2c_WriteRead for &ShareableI2c {
    type Error = hal::i2c::Error;

    fn write_read(
        &mut self,
        address: u8,
        bytes: &[u8],
        buffer: &mut [u8],
    ) -> Result<(), Self::Error> {
        self.0.borrow_mut().write_read(address, bytes, buffer)
    }
}

pub type Led = GpioPin<Output<PushPull>, 2>;
pub type Lm75 = lm75::Lm75<&'static ShareableI2c, lm75::ic::Lm75>;
pub type Dht11 = dht11::Dht11<GpioPin<Output<OpenDrain>, 4>>;
pub type Display = GraphicsMode<I2cInterface<&'static ShareableI2c>>;
pub type BleCon = Ble<BleConnector<'static>>;
pub struct Hardware {
    pub clocks: Clocks<'static>,
    /// onboad led
    pub led: Led,
    /// combined temp and humidity sensor
    pub dht11: Dht11,
    /// i2c temp sensor
    pub lm75: Lm75,
    /// i2c connected oled display
    pub disp: Option<Display>,
    /// bleps Ble interface
    pub ble: BleCon,
}
