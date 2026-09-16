use crate::device::{AsyncSensorBridgeDevice, SensorBridgeDevice};
use crate::protocol::definitions::SensorBridgePort;
use crate::protocol::errors::SensorBridgeError;

/// Status code constants for I2C proxy transceive operations.
pub const I2C_STATUS_OK: u8 = 0;
pub const I2C_STATUS_CHANNEL_DISABLED: u8 = 1;
pub const I2C_STATUS_NACK: u8 = 2;
pub const I2C_STATUS_TIMEOUT: u8 = 3;
pub const I2C_STATUS_UNSPECIFIED_ERROR: u8 = 4;

/// Result of an I2C transceive operation via I2cProxy.
#[derive(Debug, Clone)]
pub struct I2cProxyResult {
    pub status: u8,
    pub error: Option<String>,
    pub rx_data: Vec<u8>,
}

/// Synchronous I2C Proxy for communicating through a specific port on SEK-SensorBridge.
#[derive(Clone)]
pub struct SensorBridgeI2cProxy {
    device: SensorBridgeDevice,
    port: SensorBridgePort,
}

impl SensorBridgeI2cProxy {
    pub const API_VERSION: u32 = 1;

    pub fn new(
        device: SensorBridgeDevice,
        port: SensorBridgePort,
    ) -> Result<Self, SensorBridgeError> {
        if port == SensorBridgePort::All {
            return Err(SensorBridgeError::InvalidPort(0xFF));
        }
        Ok(Self { device, port })
    }

    pub fn port(&self) -> SensorBridgePort {
        self.port
    }

    pub fn description(&self) -> &'static str {
        "SensorBridge"
    }

    pub fn channel_count(&self) -> Option<usize> {
        None
    }

    pub fn transceive(
        &mut self,
        slave_address: u8,
        tx_data: &[u8],
        rx_length: usize,
        read_delay_s: f64,
        timeout_s: f64,
    ) -> I2cProxyResult {
        let max_sec = read_delay_s.max(timeout_s);
        let total_timeout_us = (max_sec * 1_000_000.0) as u32;

        match self.device.transceive_i2c(
            self.port,
            slave_address,
            tx_data,
            rx_length,
            total_timeout_us,
        ) {
            Ok(rx_data) => I2cProxyResult {
                status: I2C_STATUS_OK,
                error: None,
                rx_data,
            },
            Err(SensorBridgeError::I2cNack)
            | Err(SensorBridgeError::Shdlc(
                rust_shdlc_driver::protocol::errors::ShdlcError::DeviceError { code: 0x29, .. },
            )) => I2cProxyResult {
                status: I2C_STATUS_NACK,
                error: Some("I2C slave NACK".to_string()),
                rx_data: Vec::new(),
            },
            Err(SensorBridgeError::I2cTimeout)
            | Err(SensorBridgeError::Shdlc(
                rust_shdlc_driver::protocol::errors::ShdlcError::DeviceError { code: 0x2B, .. },
            )) => I2cProxyResult {
                status: I2C_STATUS_TIMEOUT,
                error: Some("I2C operation timeout".to_string()),
                rx_data: Vec::new(),
            },
            Err(e) => I2cProxyResult {
                status: I2C_STATUS_UNSPECIFIED_ERROR,
                error: Some(e.to_string()),
                rx_data: Vec::new(),
            },
        }
    }
}

/// Asynchronous I2C Proxy for communicating through a specific port on SEK-SensorBridge.
#[derive(Clone)]
pub struct AsyncSensorBridgeI2cProxy {
    device: AsyncSensorBridgeDevice,
    port: SensorBridgePort,
}

impl AsyncSensorBridgeI2cProxy {
    pub const API_VERSION: u32 = 1;

    pub fn new(
        device: AsyncSensorBridgeDevice,
        port: SensorBridgePort,
    ) -> Result<Self, SensorBridgeError> {
        if port == SensorBridgePort::All {
            return Err(SensorBridgeError::InvalidPort(0xFF));
        }
        Ok(Self { device, port })
    }

    pub fn port(&self) -> SensorBridgePort {
        self.port
    }

    pub fn description(&self) -> &'static str {
        "SensorBridge"
    }

    pub fn channel_count(&self) -> Option<usize> {
        None
    }

    pub async fn transceive(
        &mut self,
        slave_address: u8,
        tx_data: &[u8],
        rx_length: usize,
        read_delay_s: f64,
        timeout_s: f64,
    ) -> I2cProxyResult {
        let max_sec = read_delay_s.max(timeout_s);
        let total_timeout_us = (max_sec * 1_000_000.0) as u32;

        match self
            .device
            .transceive_i2c(
                self.port,
                slave_address,
                tx_data,
                rx_length,
                total_timeout_us,
            )
            .await
        {
            Ok(rx_data) => I2cProxyResult {
                status: I2C_STATUS_OK,
                error: None,
                rx_data,
            },
            Err(SensorBridgeError::I2cNack)
            | Err(SensorBridgeError::Shdlc(
                rust_shdlc_driver::protocol::errors::ShdlcError::DeviceError { code: 0x29, .. },
            )) => I2cProxyResult {
                status: I2C_STATUS_NACK,
                error: Some("I2C slave NACK".to_string()),
                rx_data: Vec::new(),
            },
            Err(SensorBridgeError::I2cTimeout)
            | Err(SensorBridgeError::Shdlc(
                rust_shdlc_driver::protocol::errors::ShdlcError::DeviceError { code: 0x2B, .. },
            )) => I2cProxyResult {
                status: I2C_STATUS_TIMEOUT,
                error: Some("I2C operation timeout".to_string()),
                rx_data: Vec::new(),
            },
            Err(e) => I2cProxyResult {
                status: I2C_STATUS_UNSPECIFIED_ERROR,
                error: Some(e.to_string()),
                rx_data: Vec::new(),
            },
        }
    }
}
