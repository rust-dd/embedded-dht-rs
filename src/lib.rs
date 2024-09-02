#![no_std]

use embedded_hal::digital::{ErrorType, InputPin, OutputPin};

pub struct Dht11<P: InputPin + OutputPin> {
    pin: P
}

impl<P: InputPin + OutputPin> Dht11<P> {

    pub fn new(pin: P) -> Self {
       Self { pin } 
    }

    pub fn read(&mut self) -> Result<bool, <P as ErrorType>::Error> {
        return self.pin.is_high();
    }
}