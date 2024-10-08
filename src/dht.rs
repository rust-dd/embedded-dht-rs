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


#[cfg(test)]
mod tests {
    use super::*;
    use embedded_hal_mock::eh1::digital::{Mock, State, Transaction as PinTransaction};
    use embedded_hal_mock::eh1::delay::NoopDelay as MockNoop;

    #[test]
    fn test_read_byte() {
    // Set up the pin transactions to mock the behavior of the sensor during the reading of a byte.
    // Each bit read from the sensor starts with a High state that lasts long enough
    // to signify the bit, followed by reading whether it stays High (bit 1) or goes Low (bit 0).
    let expectations = [
        // Bit 1 - 0
        PinTransaction::get(State::High),
        PinTransaction::get(State::Low), 

        // Bit 2 - 1
        PinTransaction::get(State::High),
        PinTransaction::get(State::High), 
        PinTransaction::get(State::Low),

        // Bit 3 - 0
        PinTransaction::get(State::High),
        PinTransaction::get(State::Low), 

        // Bit 4 - 1
        PinTransaction::get(State::High),
        PinTransaction::get(State::High),
        PinTransaction::get(State::Low),

        // Bit 5 - 0
        PinTransaction::get(State::High),
        PinTransaction::get(State::Low), 

        // Bit 6 - 1
        PinTransaction::get(State::High),
        PinTransaction::get(State::High),
        PinTransaction::get(State::Low), 

        // Bit 7 - 1
        PinTransaction::get(State::High),
        PinTransaction::get(State::High),
        PinTransaction::get(State::Low), 

        // Bit 8 - 1
        PinTransaction::get(State::High),
        PinTransaction::get(State::High),
        PinTransaction::get(State::Low), 
        
    ];

        let mock_pin = Mock::new(&expectations);
        let mock_delay = MockNoop::new();

        let mut dht = Dht::new(mock_pin, mock_delay);

        let result = dht.read_byte().unwrap();
        assert_eq!(result, 0b01010111);
        
        dht.pin.done();
    }

    #[test]
    fn test_wait_until_state() {
        let expectations = [
            PinTransaction::get(State::Low),
            PinTransaction::get(State::Low),
            PinTransaction::get(State::High),
        ];

        let mock_pin = Mock::new(&expectations);
        let mock_delay = MockNoop::new();

        let mut dht = Dht::new(mock_pin, mock_delay);

        let result = dht.wait_until_state(PinState::High);
        assert!(result.is_ok());

        dht.pin.done();
    }
}
