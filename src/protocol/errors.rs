use rust_shdlc_driver::protocol::errors::ShdlcError;
use std::collections::HashMap;
use thiserror::Error;

/// Error types for SEK-SensorBridge operations.
#[derive(Debug, Error)]
pub enum SensorBridgeError {
    #[error("SHDLC error: {0}")]
    Shdlc(#[from] ShdlcError),

    #[error("Invalid port: {0:#04x}")]
    InvalidPort(u8),

    #[error("Invalid voltage: {0}V")]
    InvalidVoltage(f64),

    #[error("Invalid I2C frequency: {0} Hz")]
    InvalidI2cFrequency(u32),

    #[error("Invalid SPI frequency: {0} Hz")]
    InvalidSpiFrequency(u32),

    #[error("Invalid SPI mode: {0}")]
    InvalidSpiMode(u8),

    #[error("I2C NACK error: slave did not acknowledge")]
    I2cNack,

    #[error("I2C timeout error: operation timed out")]
    I2cTimeout,

    #[error("I2C timing error: invalid frequency, interval, timeout or delay")]
    I2cTiming,

    #[error("I2C transceive error ({code:#04x}): {message}")]
    I2cGeneric { code: u8, message: String },

    #[error("Invalid response: {0}")]
    InvalidResponse(String),

    #[error("Buffer length mismatch: expected multiple of {expected}, got {actual}")]
    BufferLengthMismatch { expected: usize, actual: usize },
}

impl SensorBridgeError {
    /// Maps an I2C status code (0x01, 0x02, 0x03, ...) to a SensorBridgeError.
    pub fn from_i2c_status(code: u8) -> Option<Self> {
        match code {
            0x00 => None,
            0x01 => Some(SensorBridgeError::I2cNack),
            0x02 => Some(SensorBridgeError::I2cTimeout),
            0x03 => Some(SensorBridgeError::I2cTiming),
            other => Some(SensorBridgeError::I2cGeneric {
                code: other,
                message: format!("I2C error code {:#04x}", other),
            }),
        }
    }
}

/// Returns the map of SensorBridge device-specific error codes and messages for SHDLC registration.
pub fn sensorbridge_device_error_map() -> HashMap<u8, String> {
    let mut m = HashMap::new();
    m.insert(
        0x20,
        "The command is not supported by the device.".to_string(),
    );
    m.insert(
        0x21,
        "Too many repeated transceive operations are already running.".to_string(),
    );
    m.insert(
        0x22,
        "The received command contained a wrong checksum.".to_string(),
    );
    m.insert(
        0x26,
        "The received command had a different size than specified in the size field.".to_string(),
    );
    m.insert(
        0x29,
        "An I2C slave did not acknowledge the sent data.".to_string(),
    );
    m.insert(0x2B, "An I2C operation timed out.".to_string());
    m.insert(0x7F, "An unknown error occurred.".to_string());
    m
}
