![build workflow](https://github.com/rust-dd/embedded-dht-rs/actions/workflows/rust.yml/badge.svg)
[![Crates.io](https://img.shields.io/crates/v/embedded-dht-rs?style=flat-square)](https://crates.io/crates/embedded-dht-rs)
![Crates.io](https://img.shields.io/crates/l/embedded-dht-rs?style=flat-square)

# embedded-dht-rs

Welcome to `embedded-dht-rs`, a Rust library designed to make working with DHT sensors a breeze!

This library only depends on `embedded_hal`, making it versatile and compatible with virtually any microcontroller.

**Support for DHT11, DHT20, and DHT22 Sensors**: All three sensors are fully implemented and ready for use.

We’ve tested it with the ESP32-WROOM, and you can find a detailed example below to help you get started.

## Getting Started

### Tutorials

Here are some general tutorials that provide brief introductions to embedded programming:

- **Part 1 (Introduction)** - [Introduction to Embedded Systems with Rust: A Beginner's Guide Using ESP32](https://rust-dd.com/post/introduction-to-embedded-systems-with-rust-a-beginner-s-guide-using-esp32)
- **Part 2 (LED + Button)** - [Building a Simple LED and Button Interface with Rust on ESP32](https://rust-dd.com/post/building-a-simple-led-and-button-interface-with-rust-on-esp32)


### Example - ESP32

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

The DHT20 differs from the DHT11 and DHT22 because it uses the I2C communication protocol, while both the DHT11 and DHT22 rely on a single-wire signal for data transmission.


### DHT11/DHT22

![steps](/docs/steps.png)

#### Step 1

After powering on the DHT11/DHT22 (once powered, allow 1 second to pass during which the sensor stabilizes; during this time, no commands should be sent), it measures the temperature and humidity of the surrounding environment and stores the data. Meanwhile, the DATA line of the DHT11/DHT22 is kept high by a pull-up resistor. The DATA pin of the DHT11/DHT22 is in input mode, ready to detect any external signals.

#### Step 2

The microprocessor's I/O pin is set to output mode and pulled low, holding this state for at least 18 milliseconds. Then, the microprocessor's I/O is switched to input mode. Due to the pull-up resistor, the microprocessor’s I/O line and the DHT11/DHT22 DATA line will remain high, waiting for the DHT11/DHT22 to respond with a signal, as illustrated below:

![step2](/docs/step2.png)


#### Step 3

The DHT11/DHT22’s DATA pin detects an external signal and goes low, indicating that it is waiting for the external signal to complete. Once the signal ends, the DHT11/DHT22’s DATA pin switches to output mode, producing a low signal for 80 microseconds as a response. This is followed by an 80-microsecond high signal, notifying the microprocessor that the sensor is ready to transmit data. At this point, the microprocessor's I/O pin, still in input mode, detects the low signal from the DHT11/DHT22 (indicating the response) and then waits for the 80-microsecond high signal to start receiving data. The sequence of signal transmission is illustrated below:

![step3](/docs/step3.png)

#### Step 4

The DHT11/DHT22 outputs 40 bits of data through the DATA pin, and the microprocessor receives these 40 data bits. The format for a data bit "0" consists of a low level lasting 50 microseconds, followed by a high level lasting 26-28 microseconds, depending on changes in the I/O level. For a data bit "1," the format includes a low level of 50 microseconds followed by a high level lasting up to 70 microseconds. The signal formats for data bits "0" and "1" are shown below.

![step4](/docs/step4.png)

#### End signal

After outputting a low signal for 50 microseconds, the DHT11/DHT22 completes sending the 40 bits of data and switches the DATA pin back to input mode, which, along with the pull-up resistor, returns to a high state. Meanwhile, the DHT11/DHT22 internally re-measures the environmental temperature and humidity, records the new data, and waits for the next external signal.


### DHT20

![steps](/docs/dht20_steps.png)

- SDA = Serial Data Line
- SCL = Serial Clock Line

####  Start the sensor

The initial step is to supply power to the sensor using the chosen VDD voltage, which can range from 2.2V to 5.5V. Once powered on, the sensor requires less than 100ms to stabilize (with SCL held high during this period) before entering the idle state, after which it is ready to accept commands from the host (MCU).


#### Step 1

After powering on, wait at least 100ms. Before reading the temperature and humidity values, retrieve a status byte by sending 0x71. If the result of the status byte and 0x18 is not equal to 0x18, initialize the 0x1B, 0x1C, and 0x1E registers.


#### Step 2
Wait for 10ms before sending the 0xAC command to trigger the measurement. The command consists of two bytes: the first byte is 0x33 and the second byte is 0x00.

![step4](/docs/dht20_step2.png)

#### Step 3
Wait for 80ms for the measurement to complete. If Bit [7] of the status word is 0, the measurement is done, and you can proceed to read six bytes continuously; if not, continue waiting.

#### Step 4
After receiving the six bytes, the following byte is the CRC check data, which can be read if needed. If the receiver requires CRC validation, an ACK is sent after the sixth byte is received; otherwise, send a NACK to terminate. The initial CRC value is 0xFF, and the CRC8 check uses the polynomial: CRC [7:0] = 1 + X^4 + X^5 + X^8.

![step4](/docs/dht20_step4.png)

#### Step 5
Compute the temperature and humidity values.


## Comparison of DHT11, DHT20, and DHT22 40-Bit Data Formats

| Feature               | DHT11                                              | DHT20                                                  | DHT22                                                   |
|-----------------------|----------------------------------------------------|--------------------------------------------------------|---------------------------------------------------------|
| **Data Structure**     | - Byte 1: Humidity Int<br>- Byte 2: Humidity Dec (0)<br>- Byte 3: Temp Int<br>- Byte 4: Temp Dec (0)<br>- Byte 5: Checksum | - Byte 1: Humidity High<br>- Byte 2: Humidity Low<br>- Byte 3: Temp High<br>- Byte 4: Temp Low<br>- Byte 5: CRC | - Byte 1: Humidity High<br>- Byte 2: Humidity Low<br>- Byte 3: Temp High<br>- Byte 4: Temp Low<br>- Byte 5: Checksum |
| **Precision**          | Integer only                                      | Higher precision with decimals                         | Higher precision with decimals                           |
| **Example Temp**       | 25°C                                              | 25.6°C                                                 | 25.6°C                                                   |
| **Example Humidity**   | 60%                                               | 60.5%                                                  | 60.5%                                                    |
| **Example Data Bytes** | `60, 0, 25, 0, 85`                                | `2, 93, 1, 0, CRC`                                     | `2, 93, 1, 0, 96`                                        |
| **Range**              | Temp: 0–50°C<br>Hum: 20–90%                       | Temp: -40–80°C<br>Hum: 10–90%                          | Temp: -40–80°C<br>Hum: 0–100%                            |

## Example Schematic

![step3](/docs/example_esp32_dht11.png)
