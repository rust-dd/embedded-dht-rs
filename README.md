![build workflow](https://github.com/rust-dd/embedded-dht-rs/actions/workflows/rust.yml/badge.svg)
[![Crates.io](https://img.shields.io/crates/v/embedded-dht-rs?style=flat-square)](https://crates.io/crates/embedded-dht-rs)
![Crates.io](https://img.shields.io/crates/l/embedded-dht-rs?style=flat-square)

# embedded-dht-rs

Welcome to `embedded-dht-rs`, a Rust library designed to make working with DHT sensors a breeze!

This library only depends on `embedded_hal`, making it versatile and compatible with virtually any microcontroller.

### Features:

- **DHT11 and DHT22 sensor support**: Both sensors are fully implemented and ready to use.

We’ve tested it with the ESP32-WROOM, and you can find a detailed example below to help you get started.

## Getting Started

### Example 

```rust
#![no_std]
#![no_main]

use embedded_dht_rs::{dht11::Dht11, dht22::Dht22};
use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl,
    delay::Delay,
    gpio::{Io, Level, OutputOpenDrain, Pull},
    peripherals::Peripherals,
    prelude::*,
    system::SystemControl,
};
#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = SystemControl::new(peripherals.SYSTEM);
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();
    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);

    esp_println::logger::init_logger_from_env();

    let delay = Delay::new(&clocks);

    let gpio4 = OutputOpenDrain::new(io.pins.gpio4, Level::High, Pull::None);
    let gpio5 = OutputOpenDrain::new(io.pins.gpio5, Level::High, Pull::None);

    let mut dht11 = Dht11::new(gpio4, delay);
    let mut dht22 = Dht22::new(gpio5, delay);

    loop {
        delay.delay(5000.millis());

        match dht11.read() {
            Ok(sensor_reading) => log::info!(
                "DHT 11 Sensor - Temperature: {} °C, humidity: {} %",
                sensor_reading.temperature,
                sensor_reading.humidity
            ),
            Err(error) => log::error!("An error occurred while trying to read sensor: {:?}", error),
        }

        match dht22.read() {
            Ok(sensor_reading) => log::info!(
                "DHT 22 Sensor - Temperature: {} °C, humidity: {} %",
                sensor_reading.temperature,
                sensor_reading.humidity
            ),
            Err(error) => log::error!("An error occurred while trying to read sensor: {:?}", error),
        }

        log::info!("-----");
    }
}
```

![running](/docs/example_esp32_dht_running.png)


## Implementation Specification

We have gathered all the information you need to understand in order to implement a library like this. Additionally, we’ve included a few comments in the code for those curious about the details, based on the following specification.


![steps](/docs/steps.png)

### Step 1

After powering on the DHT11/DHT22 (once powered, allow 1 second to pass during which the sensor stabilizes; during this time, no commands should be sent), it measures the temperature and humidity of the surrounding environment and stores the data. Meanwhile, the DATA line of the DHT11/DHT22 is kept high by a pull-up resistor. The DATA pin of the DHT11/DHT22 is in input mode, ready to detect any external signals.

### Step 2

The microprocessor's I/O pin is set to output mode and pulled low, holding this state for at least 18 milliseconds. Then, the microprocessor's I/O is switched to input mode. Due to the pull-up resistor, the microprocessor’s I/O line and the DHT11/DHT22 DATA line will remain high, waiting for the DHT11/DHT22 to respond with a signal, as illustrated below:

![step2](/docs/step2.png)


### Step 3

The DHT11/DHT22’s DATA pin detects an external signal and goes low, indicating that it is waiting for the external signal to complete. Once the signal ends, the DHT11/DHT22’s DATA pin switches to output mode, producing a low signal for 80 microseconds as a response. This is followed by an 80-microsecond high signal, notifying the microprocessor that the sensor is ready to transmit data. At this point, the microprocessor's I/O pin, still in input mode, detects the low signal from the DHT11/DHT22 (indicating the response) and then waits for the 80-microsecond high signal to start receiving data. The sequence of signal transmission is illustrated below:

![step3](/docs/step3.png)

### Step 4

The DHT11/DHT22 outputs 40 bits of data through the DATA pin, and the microprocessor receives these 40 data bits. The format for a data bit "0" consists of a low level lasting 50 microseconds, followed by a high level lasting 26-28 microseconds, depending on changes in the I/O level. For a data bit "1," the format includes a low level of 50 microseconds followed by a high level lasting up to 70 microseconds. The signal formats for data bits "0" and "1" are shown below.

![step4](/docs/step4.png)

### End signal

After outputting a low signal for 50 microseconds, the DHT11/DHT22 completes sending the 40 bits of data and switches the DATA pin back to input mode, which, along with the pull-up resistor, returns to a high state. Meanwhile, the DHT11/DHT22 internally re-measures the environmental temperature and humidity, records the new data, and waits for the next external signal.



## Comparison of DHT11 and DHT22 40-Bit Data Formats

| Feature               | DHT11                                                                                               | DHT22                                                                                           |
|-----------------------|-----------------------------------------------------------------------------------------------------|-------------------------------------------------------------------------------------------------|
| **Data Structure**    | - **Byte 1:** Humidity Integer Part<br>- **Byte 2:** Humidity Decimal Part (always 0)<br>- **Byte 3:** Temperature Integer Part<br>- **Byte 4:** Temperature Decimal Part (always 0)<br>- **Byte 5:** Checksum | - **Byte 1:** Humidity High Byte<br>- **Byte 2:** Humidity Low Byte<br>- **Byte 3:** Temperature High Byte<br>- **Byte 4:** Temperature Low Byte<br>- **Byte 5:** Checksum |
| **Precision**         | Integer values only                                                                                | Includes decimal values for higher precision                                                   |
| **Example Temperature** | 25°C                                                                                              | 25.6°C                                                                                          |
| **Example Humidity**  | 60%                                                                                                 | 60.5%                                                                                           |
| **Example Data Bytes**        | `60, 0, 25, 0, 85`                                                                                  | `2, 93, 1, 0, 96`                                                                               |
| **Measurement Range** | - Temperature: 0–50°C<br>- Humidity: 20–90%                                                         | - Temperature: -40–80°C<br>- Humidity: 0–100%                                                   |


## Example Schematic 

![step3](/docs/example_esp32_dht11.png)
