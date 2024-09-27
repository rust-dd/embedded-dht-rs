use embedded_hal::{delay::DelayNs, i2c::I2c};

use crate::{SensorError, SensorReading};

pub struct Dht20<I: I2c, D: DelayNs> {
    pub i2c: I,
    pub delay: D,
}

    impl<I: I2c, D: DelayNs> Dht20<I, D> {
        const SENSOR_ADDRESS: u8 = 0x38;

        pub fn new(i2c: I, delay: D) -> Self {
            Self {
                i2c,
                delay,
            }
        }

        pub fn read(&mut self) -> Result<SensorReading<f32>, SensorError> {
            // Check status
            let mut status_response: [u8; 1] = [0; 1];
            let _ = self.i2c.write_read(Self::SENSOR_ADDRESS, &[0x71], &mut status_response);
            
            // Callibration if needed
            if status_response[0] & 0x18 != 0x18 {
                let _ = self.i2c.write(Self::SENSOR_ADDRESS, &[0x1B, 0, 0]);
                let _ = self.i2c.write(Self::SENSOR_ADDRESS, &[0x1C, 0, 0]);
                let _ = self.i2c.write(Self::SENSOR_ADDRESS, &[0x1E, 0, 0]);
            }
            
            // Trigger the measurement
            self.delay.delay_ms(10);
            let _ = self.i2c.write(Self::SENSOR_ADDRESS, &[0xAC, 0x33, 0x00]);

            // Read the measurement status
            self.delay.delay_ms(80);
            loop {
                let mut measurement_status_response: [u8; 1] = [0; 1];
                let _ = self.i2c.read(Self::SENSOR_ADDRESS, &mut measurement_status_response);
                let status_word = measurement_status_response[0];
                if status_word & 0b1000_0000 == 0 {
                    break;
                }
                self.delay.delay_ms(1);
            }

            // Read the measurement (1 status + 5 data + 1 crc)
            let mut measurement_response: [u8; 7] = [0; 7];
            let _ = self.i2c.read(Self::SENSOR_ADDRESS, &mut measurement_response);

            // Humidity 20 bits (8 + 8 + 4)
            let mut raw_humidity = measurement_response[1] as u32;
            raw_humidity = (raw_humidity << 8) + measurement_response[2] as u32;
            raw_humidity = (raw_humidity << 4) + (measurement_response[3] >> 4) as u32;
            let humidity_percentage = (raw_humidity as f32/ ((1 << 20) as f32)) * 100.0;


            // Temperature 20 bits
            let mut raw_temperature = (measurement_response[3] & 0b1111) as u32;
            raw_temperature = (raw_temperature << 8) + measurement_response[4] as u32;
            raw_temperature = (raw_temperature << 8) + measurement_response[5] as u32;
            let temperatue_percentage = (raw_temperature as f32 / ((1 << 20) as f32)) * 200.0 - 50.0;

            // Compare the calculated CRC with the received CRC
            let data = &measurement_response[..6];
            let received_crc = measurement_response[6];
            let calculcated_crc = Self::calculate_crc(data);
            if received_crc != calculcated_crc {
                return Err(SensorError::ChecksumMismatch);
            }

            Ok(SensorReading {
                humidity: humidity_percentage as f32,
                temperature: temperatue_percentage as f32,
            })
        }


        fn calculate_crc(data: &[u8]) -> u8 {
            let polynomial = 0x31u8; // x^8 + x^5 + x^4 + 1
            let mut crc = 0xFFu8;

            for &byte in data {
                crc ^= byte;
                // CRC8 - process every bit
                for _ in 0..8 {
                    if crc & 0x80 != 0 {
                        crc = (crc << 1) ^ polynomial;
                    } else {
                        crc <<= 1;
                    }
                }
            }
            return 10;
        }

    }
