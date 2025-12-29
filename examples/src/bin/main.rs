#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use embedded_dht_rs::dht11::Dht11;
use embedded_dht_rs::dht20::Dht20;
use embedded_dht_rs::dht22::Dht22;
use esp_backtrace as _;
use esp_hal::{
    clock::CpuClock,
    delay::Delay,
    gpio::{DriveMode, Flex, OutputConfig, Pull},
    i2c::master::{Config as I2cConfig, I2c},
    main,
};

// This creates a default app-descriptor required by the ESP-IDF bootloader.
esp_bootloader_esp_idf::esp_app_desc!();

#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]
#[main]
fn main() -> ! {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_println::logger::init_logger_from_env();

    let delay = Delay::new();

    // DHT11 sensor setup (GPIO4)
    let mut dht11_pin = Flex::new(peripherals.GPIO4);
    dht11_pin.apply_output_config(
        &OutputConfig::default()
            .with_drive_mode(DriveMode::OpenDrain)
            .with_pull(Pull::None),
    );
    dht11_pin.set_output_enable(true);
    dht11_pin.set_input_enable(true);
    dht11_pin.set_high();

    let mut dht11 = Dht11::new(dht11_pin, Delay::new());

    // DHT22 sensor setup (GPIO5)
    let mut dht22_pin = Flex::new(peripherals.GPIO5);
    dht22_pin.apply_output_config(
        &OutputConfig::default()
            .with_drive_mode(DriveMode::OpenDrain)
            .with_pull(Pull::None),
    );
    dht22_pin.set_output_enable(true);
    dht22_pin.set_input_enable(true);
    dht22_pin.set_high();

    let mut dht22 = Dht22::new(dht22_pin, Delay::new());

    // DHT20 sensor setup (I2C on GPIO21/GPIO22)
    let i2c = I2c::new(peripherals.I2C0, I2cConfig::default())
        .expect("Failed to initialize I2C")
        .with_sda(peripherals.GPIO21)
        .with_scl(peripherals.GPIO22);

    let mut dht20 = Dht20::new(i2c, Delay::new());

    loop {
        delay.delay_millis(2000);

        match dht11.read() {
            Ok(sensor_reading) => {
                esp_println::println!(
                    "DHT11 - Temperature: {} °C, humidity: {} %",
                    sensor_reading.temperature,
                    sensor_reading.humidity
                );
            }
            Err(error) => {
                esp_println::dbg!("Failed to read DHT11 sensor: {:?}", error);
            }
        }

        match dht22.read() {
            Ok(sensor_reading) => {
                esp_println::println!(
                    "DHT 22 Sensor - Temperature: {} °C, humidity: {} %",
                    sensor_reading.temperature,
                    sensor_reading.humidity
                );
            }
            Err(error) => {
                esp_println::dbg!("An error occurred while trying to read sensor: {:?}", error);
            }
        }

        match dht20.read() {
            Ok(sensor_reading) => {
                esp_println::println!(
                    "DHT 20 Sensor - Temperature: {} °C, humidity: {} %",
                    sensor_reading.temperature,
                    sensor_reading.humidity
                );
            }
            Err(error) => {
                esp_println::dbg!("An error occurred while trying to read sensor: {:?}", error);
            }
        }
    }
}
