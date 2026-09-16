use byteorder::{BigEndian, ByteOrder};
use rust_shdlc_driver::protocol::commands::ShdlcCommand;
use rust_shdlc_driver::protocol::errors::ShdlcError;
use std::time::Duration;

/// SHDLC Command 0x02: Set I2C Frequency.
#[derive(Debug, Clone, Copy)]
pub struct SetI2cFrequency {
    pub port: u8,
    pub frequency: u8,
}

impl SetI2cFrequency {
    pub fn new(port: u8, frequency: u8) -> Self {
        Self { port, frequency }
    }
}

impl ShdlcCommand for SetI2cFrequency {
    type Response = ();

    fn id(&self) -> u8 {
        0x02
    }

    fn data(&self) -> Vec<u8> {
        vec![self.port, self.frequency]
    }

    fn max_response_time(&self) -> Duration {
        Duration::from_millis(50)
    }

    fn min_response_length(&self) -> usize {
        0
    }

    fn max_response_length(&self) -> usize {
        0
    }

    fn interpret_response(&self, _data: &[u8]) -> Result<Self::Response, ShdlcError> {
        Ok(())
    }
}

/// SHDLC Command 0x10: I2C Scan.
#[derive(Debug, Clone, Copy)]
pub struct I2cScan {
    pub port: u8,
    pub first_address: u8,
    pub last_address: u8,
}

impl I2cScan {
    pub fn new(port: u8, first_address: u8, last_address: u8) -> Self {
        Self {
            port,
            first_address,
            last_address,
        }
    }
}

impl ShdlcCommand for I2cScan {
    type Response = Vec<u8>;

    fn id(&self) -> u8 {
        0x10
    }

    fn data(&self) -> Vec<u8> {
        vec![self.port, self.first_address, self.last_address]
    }

    fn max_response_time(&self) -> Duration {
        Duration::from_millis(100)
    }

    fn min_response_length(&self) -> usize {
        0
    }

    fn max_response_length(&self) -> usize {
        255
    }

    fn interpret_response(&self, data: &[u8]) -> Result<Self::Response, ShdlcError> {
        Ok(data.to_vec())
    }
}

/// SHDLC Command 0x11: First Transceive.
#[derive(Debug, Clone)]
pub struct FirstTransceive {
    pub port: u8,
    pub address: u8,
    pub tx_length: u32,
    pub rx_length: u32,
    pub timeout_us: u32,
    pub tx_data: Vec<u8>,
}

impl FirstTransceive {
    pub fn new(
        port: u8,
        address: u8,
        tx_length: u32,
        rx_length: u32,
        timeout_us: u32,
        tx_data: &[u8],
    ) -> Self {
        Self {
            port,
            address,
            tx_length,
            rx_length,
            timeout_us,
            tx_data: tx_data.to_vec(),
        }
    }
}

impl ShdlcCommand for FirstTransceive {
    type Response = Vec<u8>;

    fn id(&self) -> u8 {
        0x11
    }

    fn data(&self) -> Vec<u8> {
        let mut d = Vec::with_capacity(15 + self.tx_data.len());
        d.push(0x00); // Subcommand 0x00: first transceive
        d.push(self.port);
        d.push(self.address);
        let mut buf = [0u8; 4];
        BigEndian::write_u32(&mut buf, self.tx_length);
        d.extend_from_slice(&buf);
        BigEndian::write_u32(&mut buf, self.rx_length);
        d.extend_from_slice(&buf);
        BigEndian::write_u32(&mut buf, self.timeout_us);
        d.extend_from_slice(&buf);
        d.extend_from_slice(&self.tx_data);
        d
    }

    fn max_response_time(&self) -> Duration {
        Duration::from_secs(10)
    }

    fn min_response_length(&self) -> usize {
        0
    }

    fn max_response_length(&self) -> usize {
        255
    }

    fn interpret_response(&self, data: &[u8]) -> Result<Self::Response, ShdlcError> {
        Ok(data.to_vec())
    }
}

/// SHDLC Command 0x11: Subsequent Transceive.
#[derive(Debug, Clone)]
pub struct SubsequentTransceive {
    pub port: u8,
    pub tx_data: Vec<u8>,
}

impl SubsequentTransceive {
    pub fn new(port: u8, tx_data: &[u8]) -> Self {
        Self {
            port,
            tx_data: tx_data.to_vec(),
        }
    }
}

impl ShdlcCommand for SubsequentTransceive {
    type Response = Vec<u8>;

    fn id(&self) -> u8 {
        0x11
    }

    fn data(&self) -> Vec<u8> {
        let mut d = Vec::with_capacity(2 + self.tx_data.len());
        d.push(0x01); // Subcommand 0x01: subsequent transceive
        d.push(self.port);
        d.extend_from_slice(&self.tx_data);
        d
    }

    fn max_response_time(&self) -> Duration {
        Duration::from_secs(10)
    }

    fn min_response_length(&self) -> usize {
        0
    }

    fn max_response_length(&self) -> usize {
        255
    }

    fn interpret_response(&self, data: &[u8]) -> Result<Self::Response, ShdlcError> {
        Ok(data.to_vec())
    }
}

/// SHDLC Command 0x12: I2C Repeated Transceive.
#[derive(Debug, Clone)]
pub struct I2cRepeatedTransceive {
    pub interval_us: u32,
    pub port: u8,
    pub address: u8,
    pub tx_length: u32,
    pub rx_length: u32,
    pub timeout_us: u32,
    pub read_delay_us: u32,
    pub tx_data: Vec<u8>,
}

impl I2cRepeatedTransceive {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        interval_us: u32,
        port: u8,
        address: u8,
        tx_length: u32,
        rx_length: u32,
        timeout_us: u32,
        read_delay_us: u32,
        tx_data: &[u8],
    ) -> Self {
        Self {
            interval_us,
            port,
            address,
            tx_length,
            rx_length,
            timeout_us,
            read_delay_us,
            tx_data: tx_data.to_vec(),
        }
    }
}

impl ShdlcCommand for I2cRepeatedTransceive {
    type Response = Vec<u8>;

    fn id(&self) -> u8 {
        0x12
    }

    fn data(&self) -> Vec<u8> {
        let mut d = Vec::with_capacity(18 + self.tx_data.len());
        let mut buf = [0u8; 4];
        BigEndian::write_u32(&mut buf, self.interval_us);
        d.extend_from_slice(&buf);
        d.push(self.port);
        d.push(self.address);
        BigEndian::write_u32(&mut buf, self.tx_length);
        d.extend_from_slice(&buf);
        BigEndian::write_u32(&mut buf, self.rx_length);
        d.extend_from_slice(&buf);
        BigEndian::write_u32(&mut buf, self.timeout_us);
        d.extend_from_slice(&buf);
        BigEndian::write_u32(&mut buf, self.read_delay_us);
        d.extend_from_slice(&buf);
        d.extend_from_slice(&self.tx_data);
        d
    }

    fn max_response_time(&self) -> Duration {
        Duration::from_millis(50)
    }

    fn min_response_length(&self) -> usize {
        0
    }

    fn max_response_length(&self) -> usize {
        255
    }

    fn interpret_response(&self, data: &[u8]) -> Result<Self::Response, ShdlcError> {
        Ok(data.to_vec())
    }
}

/// SHDLC Command 0x50: Read Buffer.
#[derive(Debug, Clone, Copy)]
pub struct ReadBuffer {
    pub handle: u8,
}

impl ReadBuffer {
    pub fn new(handle: u8) -> Self {
        Self { handle }
    }
}

impl ShdlcCommand for ReadBuffer {
    type Response = (u32, u32, Vec<u8>);

    fn id(&self) -> u8 {
        0x50
    }

    fn data(&self) -> Vec<u8> {
        vec![self.handle]
    }

    fn max_response_time(&self) -> Duration {
        Duration::from_millis(50)
    }

    fn min_response_length(&self) -> usize {
        8
    }

    fn max_response_length(&self) -> usize {
        255
    }

    fn interpret_response(&self, data: &[u8]) -> Result<Self::Response, ShdlcError> {
        let lost_bytes = BigEndian::read_u32(&data[0..4]);
        let remaining_bytes = BigEndian::read_u32(&data[4..8]);
        let rx_data = data[8..].to_vec();
        Ok((lost_bytes, remaining_bytes, rx_data))
    }
}

/// SHDLC Command 0x51: Stop Repeated Transceive.
#[derive(Debug, Clone, Copy)]
pub struct StopRepeatedTransceive {
    pub handle: u8,
}

impl StopRepeatedTransceive {
    pub fn new(handle: u8) -> Self {
        Self { handle }
    }
}

impl ShdlcCommand for StopRepeatedTransceive {
    type Response = ();

    fn id(&self) -> u8 {
        0x51
    }

    fn data(&self) -> Vec<u8> {
        vec![self.handle]
    }

    fn max_response_time(&self) -> Duration {
        Duration::from_millis(50)
    }

    fn min_response_length(&self) -> usize {
        0
    }

    fn max_response_length(&self) -> usize {
        0
    }

    fn interpret_response(&self, _data: &[u8]) -> Result<Self::Response, ShdlcError> {
        Ok(())
    }
}
