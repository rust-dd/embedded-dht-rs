#![no_std]

mod dht;
pub mod dht11;
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
}
