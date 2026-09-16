use crate::protocol::errors::SensorBridgeError;
use std::fmt;

/// SensorBridge Port identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum SensorBridgePort {
    /// Port 1 (0x00)
    One = 0x00,
    /// Port 2 (0x01)
    Two = 0x01,
    /// All Ports (0xFF)
    All = 0xFF,
}

impl SensorBridgePort {
    /// Converts port to raw byte representation.
    pub fn to_byte(self, accept_all: bool) -> Result<u8, SensorBridgeError> {
        match self {
            SensorBridgePort::One => Ok(0x00),
            SensorBridgePort::Two => Ok(0x01),
            SensorBridgePort::All => {
                if accept_all {
                    Ok(0xFF)
                } else {
                    Err(SensorBridgeError::InvalidPort(0xFF))
                }
            }
        }
    }

    /// Converts byte representation to SensorBridgePort enum.
    pub fn from_byte(byte: u8, accept_all: bool) -> Result<Self, SensorBridgeError> {
        match byte {
            0x00 => Ok(SensorBridgePort::One),
            0x01 => Ok(SensorBridgePort::Two),
            0xFF if accept_all => Ok(SensorBridgePort::All),
            _ => Err(SensorBridgeError::InvalidPort(byte)),
        }
    }
}

impl fmt::Display for SensorBridgePort {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SensorBridgePort::One => write!(f, "Port 1"),
            SensorBridgePort::Two => write!(f, "Port 2"),
            SensorBridgePort::All => write!(f, "All Ports"),
        }
    }
}

/// SPI modes supported by SensorBridge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum SensorBridgeSpiMode {
    Mode0 = 0,
    Mode1 = 1,
    Mode2 = 2,
    Mode3 = 3,
}

impl SensorBridgeSpiMode {
    pub fn from_u8(val: u8) -> Result<Self, SensorBridgeError> {
        match val {
            0 => Ok(SensorBridgeSpiMode::Mode0),
            1 => Ok(SensorBridgeSpiMode::Mode1),
            2 => Ok(SensorBridgeSpiMode::Mode2),
            3 => Ok(SensorBridgeSpiMode::Mode3),
            _ => Err(SensorBridgeError::InvalidSpiMode(val)),
        }
    }

    pub fn to_u8(self) -> u8 {
        self as u8
    }
}

/// Map of voltage in Volts (multiplied by 10 for integer lookup) to byte code.
/// 1.2V -> 0x00, 1.8V -> 0x01, 3.3V -> 0x02, 5.0V -> 0x03,
/// 1.5V -> 0x04, 2.1V -> 0x05, 2.4V -> 0x06, 2.7V -> 0x07,
/// 3.0V -> 0x08, 3.6V -> 0x09, 4.5V -> 0x0A, 5.5V -> 0x0B
pub const SUPPORTED_VOLTAGES: [f64; 12] =
    [1.2, 1.5, 1.8, 2.1, 2.4, 2.7, 3.0, 3.3, 3.6, 4.5, 5.0, 5.5];

pub fn voltage_to_byte(voltage: f64) -> Result<u8, SensorBridgeError> {
    // Round to 1 decimal place to handle floating point comparisons
    let rounded = (voltage * 10.0).round() as i32;
    match rounded {
        12 => Ok(0x00),
        15 => Ok(0x04),
        18 => Ok(0x01),
        21 => Ok(0x05),
        24 => Ok(0x06),
        27 => Ok(0x07),
        30 => Ok(0x08),
        33 => Ok(0x02),
        36 => Ok(0x09),
        45 => Ok(0x0A),
        50 => Ok(0x03),
        55 => Ok(0x0B),
        _ => Err(SensorBridgeError::InvalidVoltage(voltage)),
    }
}

pub fn byte_to_voltage(byte: u8) -> Result<f64, SensorBridgeError> {
    match byte {
        0x00 => Ok(1.2),
        0x01 => Ok(1.8),
        0x02 => Ok(3.3),
        0x03 => Ok(5.0),
        0x04 => Ok(1.5),
        0x05 => Ok(2.1),
        0x06 => Ok(2.4),
        0x07 => Ok(2.7),
        0x08 => Ok(3.0),
        0x09 => Ok(3.6),
        0x0A => Ok(4.5),
        0x0B => Ok(5.5),
        _ => Err(SensorBridgeError::InvalidVoltage(byte as f64)),
    }
}

/// Supported I2C frequencies in Hz.
pub const SUPPORTED_I2C_FREQUENCIES: [u32; 6] =
    [10_000, 50_000, 100_000, 400_000, 1_000_000, 2_000_000];

pub fn i2c_frequency_to_byte(frequency_hz: u32) -> Result<u8, SensorBridgeError> {
    match frequency_hz {
        100_000 => Ok(0x00),
        400_000 => Ok(0x01),
        1_000_000 => Ok(0x02),
        2_000_000 => Ok(0x03),
        10_000 => Ok(0x04),
        50_000 => Ok(0x05),
        _ => Err(SensorBridgeError::InvalidI2cFrequency(frequency_hz)),
    }
}

pub fn byte_to_i2c_frequency(byte: u8) -> Result<u32, SensorBridgeError> {
    match byte {
        0x00 => Ok(100_000),
        0x01 => Ok(400_000),
        0x02 => Ok(1_000_000),
        0x03 => Ok(2_000_000),
        0x04 => Ok(10_000),
        0x05 => Ok(50_000),
        _ => Err(SensorBridgeError::InvalidI2cFrequency(byte as u32)),
    }
}

/// Supported SPI frequencies in Hz.
pub const SUPPORTED_SPI_FREQUENCIES: [u32; 6] = [
    330_000, 600_000, 1_000_000, 5_000_000, 10_000_000, 21_000_000,
];

pub fn spi_frequency_to_byte(frequency_hz: u32) -> Result<u8, SensorBridgeError> {
    match frequency_hz {
        330_000 => Ok(0x00),
        600_000 => Ok(0x01),
        1_000_000 => Ok(0x02),
        5_000_000 => Ok(0x03),
        10_000_000 => Ok(0x04),
        21_000_000 => Ok(0x05),
        _ => Err(SensorBridgeError::InvalidSpiFrequency(frequency_hz)),
    }
}

pub fn byte_to_spi_frequency(byte: u8) -> Result<u32, SensorBridgeError> {
    match byte {
        0x00 => Ok(330_000),
        0x01 => Ok(600_000),
        0x02 => Ok(1_000_000),
        0x03 => Ok(5_000_000),
        0x04 => Ok(10_000_000),
        0x05 => Ok(21_000_000),
        _ => Err(SensorBridgeError::InvalidSpiFrequency(byte as u32)),
    }
}
