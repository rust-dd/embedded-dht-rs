use embedded_hal::{
    delay::DelayNs,
    digital::{InputPin, OutputPin, PinState},
};

use crate::{dht::Dht, SensorError, SensorReading};

pub struct Dht11<P: InputPin + OutputPin, D: DelayNs> {
    dht: Dht<P, D>,
}

impl<P: InputPin + OutputPin, D: DelayNs> Dht11<P, D> {
    pub fn new(pin: P, delay: D) -> Self {
        Self {
            dht: Dht::new(pin, delay),
        }
    }

    pub fn read(&mut self) -> Result<SensorReading, SensorError> {
        // Start communication: pull pin low for 18ms, then release.
        let _ = self.dht.pin.set_low();
        self.dht.delay.delay_ms(18);
        let _ = self.dht.pin.set_high();

        // Wait for sensor to respond.
        self.dht.delay.delay_us(48);

        // Sync with sensor: wait for high then low signals.
        let _ = self.dht.wait_until_state(PinState::High);
        let _ = self.dht.wait_until_state(PinState::Low);

        // Start reading 40 bits
        let humidity_integer = self.dht.read_byte()?;
        let humidity_decimal = self.dht.read_byte()?;
        let temperature_integer = self.dht.read_byte()?;
        let temperature_decimal = self.dht.read_byte()?;
        let checksum = self.dht.read_byte()?;

        // Checksum
        let sum = humidity_integer
            .wrapping_add(humidity_decimal)
            .wrapping_add(temperature_integer)
            .wrapping_add(temperature_decimal);
        if sum != checksum {
            return Err(SensorError::ChecksumMismatch);
        }

        Ok(SensorReading {
            humidity: humidity_integer as f32,
            temperature: temperature_integer as f32,
        })
    }
}
