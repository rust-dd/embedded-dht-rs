#![no_std]

use embedded_hal::{
    delay::DelayNs,
    digital::{ErrorType, InputPin, OutputPin, PinState},
};

pub struct Dht11<P: InputPin + OutputPin, D: DelayNs> {
    pin: P,
    delay: D,
}

impl<P: InputPin + OutputPin, D: DelayNs> Dht11<P, D> {
    pub fn new(pin: P, delay: D) -> Self {
        Self { pin, delay }
    }

    pub fn read(&mut self) -> Result<bool, <P as ErrorType>::Error> {
        // Start communication: pull pin low for 18ms, then release.
        let _ = self.pin.set_low();
        self.delay.delay_ms(18); 
        let _ = self.pin.set_high();

        // Wait for sensor to respond.
        self.delay.delay_us(48);

        // Sync with sensor: wait for high then low signals.
        let _ = self.wait_until_state(PinState::High);
        let _ = self.wait_until_state(PinState::Low);

        // Start reading 

        // TODO

        return self.pin.is_high();
    }

    /// Waits until the pin reaches the specified state.
    ///
    /// This helper function continuously polls the pin until it reaches the desired `PinState`.
    ///
    /// # Arguments
    ///
    /// * `state` - The target `PinState` to wait for (either `Low` or `High`).
    fn wait_until_state(&mut self, state: PinState) -> Result<(), <P as ErrorType>::Error>{
        loop {
            match state {
                PinState::Low => if self.pin.is_low()? { break; },
                PinState::High => if self.pin.is_high()? { break; }
            };
            self.delay.delay_us(1);
        }
        Ok(())
    }
}

