use crate::protocol::definitions::SensorBridgePort;
use crate::protocol::errors::SensorBridgeError;

/// Handle for a repeated I2C transceive operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RepeatedTransceiveHandle {
    pub raw_handle: u8,
    pub rx_length: usize,
}

impl RepeatedTransceiveHandle {
    pub fn new(raw_handle: u8, rx_length: usize) -> Self {
        Self {
            raw_handle,
            rx_length,
        }
    }

    /// Task slot number (0..7).
    pub fn slot(&self) -> u8 {
        (self.raw_handle >> 4) & 0x0F
    }

    /// Channel number (0..1).
    pub fn channel(&self) -> u8 {
        self.raw_handle & 0x0F
    }

    /// Port corresponding to channel.
    pub fn port(&self) -> SensorBridgePort {
        if self.channel() == 0 {
            SensorBridgePort::One
        } else {
            SensorBridgePort::Two
        }
    }
}

/// Represents a single buffered value from a repeated I2C transceive.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BufferedValue {
    pub raw_status: u8,
    pub raw_data: Vec<u8>,
}

impl BufferedValue {
    pub fn new(raw_packet: &[u8]) -> Result<Self, SensorBridgeError> {
        if raw_packet.is_empty() {
            return Err(SensorBridgeError::InvalidResponse(
                "Empty buffer packet".to_string(),
            ));
        }
        Ok(Self {
            raw_status: raw_packet[0],
            raw_data: raw_packet[1..].to_vec(),
        })
    }

    pub fn is_ok(&self) -> bool {
        self.raw_status == 0
    }

    pub fn error(&self) -> Option<SensorBridgeError> {
        SensorBridgeError::from_i2c_status(self.raw_status)
    }

    pub fn data(&self) -> Result<&[u8], SensorBridgeError> {
        match self.error() {
            None => Ok(&self.raw_data),
            Some(err) => Err(err),
        }
    }
}

/// Response returned from reading the repeated transceive buffer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReadBufferResponse {
    pub lost_bytes: u32,
    pub remaining_bytes: u32,
    pub values: Vec<BufferedValue>,
}

impl ReadBufferResponse {
    pub fn from_raw(
        rx_length: usize,
        lost_bytes: u32,
        remaining_bytes: u32,
        rx_data: &[u8],
    ) -> Result<Self, SensorBridgeError> {
        let packet_len = rx_length + 1; // 1 status byte + rx_length bytes
        if packet_len == 0 || !rx_data.len().is_multiple_of(packet_len) {
            return Err(SensorBridgeError::BufferLengthMismatch {
                expected: packet_len,
                actual: rx_data.len(),
            });
        }

        let num_packets = rx_data.len() / packet_len;
        let mut values = Vec::with_capacity(num_packets);
        for i in 0..num_packets {
            let start = i * packet_len;
            let end = start + packet_len;
            values.push(BufferedValue::new(&rx_data[start..end])?);
        }

        Ok(Self {
            lost_bytes,
            remaining_bytes,
            values,
        })
    }
}
