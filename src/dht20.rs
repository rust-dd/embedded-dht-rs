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
