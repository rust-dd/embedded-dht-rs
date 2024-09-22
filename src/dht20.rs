use embedded_hal::{delay::DelayNs, i2c::I2c};

use crate::{SensorError, SensorReading};

pub struct Dht20<P: I2c, D: DelayNs> {
    pub pin: P,
    pub delay: D,
}

impl<P: I2c, D: DelayNs> Dht20<P, D> {
    pub fn new(pin: P, delay: D) -> Self {
        Self {
            pin: pin,
            delay: delay,
        }
    }

    pub fn read(&mut self) -> Result<SensorReading, SensorError> {
        // Check status
        let status_request: &[u8] = &[0x71];
        let mut status_response: [u8; 1] = [0; 1];

        let _ = self.pin.write_read(0x38, status_request, &mut status_response);
        if status_response[0] & 0x18 != 0x18 {
            // Callibration
            let _ = self.pin.write(0x38, &[0x1B, 0, 0]);
            let _ = self.pin.write(0x38, &[0x1C, 0, 0]);
            let _ = self.pin.write(0x38, &[0x1E, 0, 0]);
        }
        
        




        // TODO
        let humidity_percentage = 10.0;
        // TODO
        let temperatue_percentage = 10.0;

        Ok(SensorReading {
            humidity: humidity_percentage as f32,
            temperature: temperatue_percentage as f32,
        })
    }
}
