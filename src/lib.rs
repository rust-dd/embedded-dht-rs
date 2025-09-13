#![doc = include_str!("../README.md")]
#![no_std]

mod dht;

#[cfg(feature = "dht11")]
pub mod dht11;

#[cfg(feature = "dht20")]
pub mod dht20;

#[cfg(feature = "dht22")]
pub mod dht22;

/// Represents a reading from the sensor.
pub struct SensorReading<T> {
    pub humidity: T,
    pub temperature: T,
}

/// Possible errors when interacting with the sensor.
#[derive(Debug)]
pub enum SensorError {
    ChecksumMismatch,
    Timeout,
    PinError,
}
