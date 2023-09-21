use dht20::Dht20;
use dht_hal_drv::{dht_read, DhtType};
use embassy_time::{Duration, Instant};
use embedded_hal::digital::v2::{InputPin, OutputPin};
use esp_weather_common::state::SensorState;
use log::error;
use sgp30::Sgp30;

use crate::{
    hardware::{self, I2cDriver, OutsidePin},
    tasks::delay_task,
    STATE,
};

pub async fn run(delay: Duration, mut dht22_pin: OutsidePin, i2c: &'static I2cDriver) {
    let mut start = Instant::now();
    let mut temp_out = 0.0;
    let mut hum_out = 0.0;
    let mut temp_in = 0.0;
    let mut hum_in = 0.0;
    let mut air_quality = sgp30::Measurement {
        co2eq_ppm: 0,
        tvoc_ppb: 0,
    };

    let mut air_sensor = Sgp30::new(i2c, hardware::AIR_SENSOR_ADDRESS, esp_idf_hal::delay::Delay);
    let mut inside_sensor = Dht20::new(
        i2c,
        hardware::INSIDE_SENSOR_ADDRESS,
        esp_idf_hal::delay::Delay,
    );
    if let Err(e) = air_sensor.init() {
        error!("error initializing air sensor: {e:?}");
        return;
    }
    loop {
        delay_task(delay, &mut start).await;

        (temp_in, hum_in) = read_inside(&mut inside_sensor, temp_in, hum_in);
        (temp_out, hum_out) = read_outside(&mut dht22_pin, temp_out, hum_out);
        air_quality = read_air_quality(&mut air_sensor, air_quality);

        let state = SensorState {
            temp_in,
            temp_out,
            hum_in,
            hum_out,
            tvoc_ppb: air_quality.tvoc_ppb,
            co2_ppm: air_quality.co2eq_ppm,
        };
        STATE.set_sensors(state);
    }
}

fn read_inside(
    sensor: &mut Dht20<&I2cDriver, esp_idf_hal::delay::Delay>,
    temp: f32,
    hum: f32,
) -> (f32, f32) {
    match sensor.read() {
        Ok(reading) => (reading.temp, reading.hum),
        Err(e) => {
            error!("Error reading inside sensor: {e:?}");
            (temp, hum)
        }
    }
}

fn delay_us(us: u16) {
    let start = Instant::now();
    while (start.elapsed().as_micros() as u16) < us {}
}

fn read_outside<PIN>(pin: &mut PIN, temp: f32, hum: f32) -> (f32, f32)
where
    PIN: InputPin + OutputPin,
{
    let reading = dht_read(DhtType::DHT21, pin, &mut delay_us);
    match reading {
        Ok(val) => (val.temperature(), val.humidity()),
        Err(e) => {
            error!("could not read outside sensor: {e:?}");
            (temp, hum)
        }
    }
}

fn read_air_quality(
    sensor: &mut Sgp30<&I2cDriver, esp_idf_hal::delay::Delay>,
    old_measurement: sgp30::Measurement,
) -> sgp30::Measurement {
    if let Ok(measurement) = sensor.measure() {
        measurement
    } else {
        old_measurement
    }
}
