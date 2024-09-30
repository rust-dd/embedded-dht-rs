use embedded_hal::{
    delay::DelayNs,
    digital::{ErrorType, InputPin, OutputPin, PinState},
};

use crate::SensorError;

/// Common base struct for DHT11, DHT22 sensors.
pub struct Dht<P: InputPin + OutputPin, D: DelayNs> {
    pub pin: P,
    pub delay: D,
}

impl<P: InputPin + OutputPin, D: DelayNs> Dht<P, D> {
    pub fn new(pin: P, delay: D) -> Self {
        Self { pin, delay }
    }

    /// Reads a byte (8 bits) from the sensor.
    ///
    /// This method reads 8 bits sequentially from the sensor to construct a byte.
    /// It follows the communication protocol of the DHT11/DHT22 sensors:
    ///
    /// For each bit:
    /// - Waits for the pin to go **high** (start of bit transmission).
    /// - Delays for **30 microseconds** to sample the bit value.
    ///   - If the pin is **high** after the delay, the bit is interpreted as **'1'**.
    ///   - If the pin is **low**, the bit is interpreted as **'0'**.
    /// - Waits for the pin to go **low** (end of bit transmission).
    ///
    /// The bits are assembled into a byte, starting from the most significant bit (MSB).
    ///
    /// # Returns
    ///
    /// - `Ok(u8)`: The byte read from the sensor.
    /// - `Err(SensorError<P::Error>)`: If a pin error occurs.
    pub fn read_byte(&mut self) -> Result<u8, SensorError> {
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
    /// It introduces a **1-microsecond delay** between each poll to prevent excessive CPU usage.
    ///
    /// # Arguments
    ///
    /// - `state`: The target `PinState` to wait for (`PinState::High` or `PinState::Low`).
    ///
    /// # Returns
    ///
    /// - `Ok(())`: When the pin reaches the desired state.
    /// - `Err(SensorError<P::Error>)`: If an error occurs while reading the pin state.
    ///
    pub fn wait_until_state(&mut self, state: PinState) -> Result<(), <P as ErrorType>::Error> {
        while !match state {
            PinState::Low => self.pin.is_low(),
            PinState::High => self.pin.is_high(),
        }? {
            self.delay.delay_us(1);
        }
        Ok(())
    }
}
