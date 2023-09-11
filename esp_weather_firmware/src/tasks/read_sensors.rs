use dht_hal_drv::{dht_read, DhtType};
use embassy_time::{Duration, Instant};
use embedded_hal::digital::v2::{InputPin, OutputPin};
use esp_weather_common::state::SensorState;
use log::{debug, error};
use sgp30::Sgp30;

use crate::{
    hardware::{Dht11Pin, Dht22Pin, I2cDriver},
    tasks::delay_task,
    STATE,
};

pub async fn run(
    delay: Duration,
    mut dht11_pin: Dht11Pin,
    mut dht22_pin: Dht22Pin,
    i2c: I2cDriver,
) {
    let mut start = Instant::now();
    let mut temp_out = 0.0;
    let mut hum_out = 0.0;
    let mut temp_in = 0.0;
    let mut hum_in = 0.0;
    let mut air_quality = sgp30::Measurement {
        co2eq_ppm: 0,
        tvoc_ppb: 0,
    };

    let mut air_sensor = Sgp30::new(i2c, 0x58, esp_idf_hal::delay::Delay);
    if let Err(e) = air_sensor.init() {
        error!("error initializing air sensor: {e:?}");
        return;
    }
    loop {
        delay_task(delay, &mut start).await;

        (temp_in, hum_in) = read_dht(DhtType::DHT11, &mut dht11_pin, temp_in, hum_in);
        (temp_out, hum_out) = read_dht(DhtType::DHT22, &mut dht22_pin, temp_out, hum_out);
        air_quality = read_air_quality(&mut air_sensor, air_quality);

        let state = SensorState {
            temp_in,
            temp_out,
            hum_in,
            hum_out,
        };
        debug!("Sensor state: {state:?}");
        STATE.set_sensors(state);
    }
}

fn delay_us(us: u16) {
    let start = Instant::now();
    while (start.elapsed().as_micros() as u16) < us {}
}

fn read_dht<PIN>(dht_type: DhtType, pin: &mut PIN, temp: f32, hum: f32) -> (f32, f32)
where
    PIN: InputPin + OutputPin,
{
    let reading = dht_read(dht_type.clone(), pin, &mut delay_us);
    match reading {
        Ok(val) => (val.temperature(), val.humidity()),
        Err(e) => {
            error!("could not read {dht_type:?}: {e:?}");
            (temp, hum)
        }
    }
}

fn read_air_quality(
    sensor: &mut Sgp30<I2cDriver, esp_idf_hal::delay::Delay>,
    old_measurement: sgp30::Measurement,
) -> sgp30::Measurement {
    if let Ok(measurement) = sensor.measure() {
        measurement
    } else {
        old_measurement
    }
}
