use embedded_hal::{
    delay::DelayNs,
    digital::{InputPin, OutputPin, PinState},
};

use crate::{dht::Dht, SensorError, SensorReading};

pub struct Dht22<P: InputPin + OutputPin, D: DelayNs> {
    dht: Dht<P, D>,
}

impl<P: InputPin + OutputPin, D: DelayNs> Dht22<P, D> {
    pub fn new(pin: P, delay: D) -> Self {
        Self {
            dht: Dht::new(pin, delay),
        }
    }

    pub fn read(&mut self) -> Result<SensorReading<f32>, SensorError> {
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
        let humidity_high = self.dht.read_byte()?;
        let humidity_low = self.dht.read_byte()?;
        let temperature_high = self.dht.read_byte()?;
        let temperature_low = self.dht.read_byte()?;
        let checksum = self.dht.read_byte()?;

        // Checksum
        let sum = humidity_high
            .wrapping_add(humidity_low)
            .wrapping_add(temperature_high)
            .wrapping_add(temperature_low);
        if sum != checksum {
            return Err(SensorError::ChecksumMismatch);
        }

        let humidity_value = ((humidity_high as u16) << 8) | (humidity_low as u16);
        let humidity_percentage = humidity_value as f32 / 10.0;

        let temperature_high_clean = temperature_high & 0x7F; // 0x7F = 0111 1111
        let temperature_value = ((temperature_high_clean as u16) << 8) | (temperature_low as u16);
        let mut temperature_percentage = temperature_value as f32 / 10.0;

        if temperature_high & 0x80 != 0 {
            temperature_percentage = -temperature_percentage;
        }


        Ok(SensorReading {
            humidity: humidity_percentage,
            temperature: temperature_percentage,
        })
    }
}
