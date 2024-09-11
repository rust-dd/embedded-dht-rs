#![no_std]

use embedded_hal::{
    delay::DelayNs,
    digital::{ErrorType, InputPin, OutputPin, PinState},
};

pub struct Dht11<P: InputPin + OutputPin, D: DelayNs> {
    pin: P,
    delay: D,
}

pub struct SensorReading {
    pub humidity: u8,
    pub temperature: i8,
}

#[derive(Debug)]
pub enum SensorError {
    ChecksumMismatch,
}

impl<P: InputPin + OutputPin, D: DelayNs> Dht11<P, D> {
    pub fn new(pin: P, delay: D) -> Self {
        Self { pin, delay }
    }

    pub fn read(&mut self) -> Result<SensorReading, SensorError> {
        // Start communication: pull pin low for 18ms, then release.
        let _ = self.pin.set_low();
        self.delay.delay_ms(18);
        let _ = self.pin.set_high();

        // Wait for sensor to respond.
        self.delay.delay_us(48);

        // Sync with sensor: wait for high then low signals.
        let _ = self.wait_until_state(PinState::High);
        let _ = self.wait_until_state(PinState::Low);

        // Start reading 40 bits
        let humidity_integer = self.read_byte()?;
        let humidity_decimal = self.read_byte()?;
        let temperature_integer = self.read_byte()?;
        let temperature_decimal = self.read_byte()?;
        let checksum = self.read_byte()?;

        // Checksum
        let sum = humidity_integer + humidity_decimal + temperature_integer + temperature_decimal;
        if sum != checksum {
            return Err(SensorError::ChecksumMismatch);
        }

        Ok(SensorReading {
            humidity: humidity_integer,
            temperature: temperature_integer as i8,
        })
    }

    fn read_byte(&mut self) -> Result<u8, SensorError> {
        let mut byte: u8 = 0;
        for n in 0..8 {
            let _ = self.wait_until_state(PinState::High);
            self.delay.delay_us(30);
            let is_bit_1 = self.pin.is_high();
            if is_bit_1.unwrap() {
                let bit_mask = 1 << (7 - (n % 8));
                byte |= bit_mask;
                let _ = self.wait_until_state(PinState::Low);
            }
        }
        Ok(byte)
    }

    /// Waits until the pin reaches the specified state.
    ///
    /// This helper function continuously polls the pin until it reaches the desired `PinState`.
    ///
    /// # Arguments
    ///
    /// * `state` - The target `PinState` to wait for (either `Low` or `High`).
    fn wait_until_state(&mut self, state: PinState) -> Result<(), <P as ErrorType>::Error> {
        loop {
            match state {
                PinState::Low => {
                    if self.pin.is_low()? {
                        break;
                    }
                }
                PinState::High => {
                    if self.pin.is_high()? {
                        break;
                    }
                }
            };
            self.delay.delay_us(1);
        }
        Ok(())
    }
}
