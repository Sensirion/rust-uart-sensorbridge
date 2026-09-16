use byteorder::{BigEndian, ByteOrder};
use rust_shdlc_driver::protocol::commands::ShdlcCommand;
use rust_shdlc_driver::protocol::errors::ShdlcError;
use std::time::Duration;

/// SHDLC Command 0x60: SPI Config.
#[derive(Debug, Clone, Copy)]
pub struct SpiConfig {
    pub port: u8,
    pub mode: u8,
    pub frequency: u8,
}

impl SpiConfig {
    pub fn new(port: u8, mode: u8, frequency: u8) -> Self {
        Self {
            port,
            mode,
            frequency,
        }
    }
}

impl ShdlcCommand for SpiConfig {
    type Response = ();

    fn id(&self) -> u8 {
        0x60
    }

    fn data(&self) -> Vec<u8> {
        vec![self.port, self.mode, self.frequency]
    }

    fn max_response_time(&self) -> Duration {
        Duration::from_millis(100)
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

/// SHDLC Command 0x61: SPI Transceive.
#[derive(Debug, Clone)]
pub struct SpiTransceive {
    pub port: u8,
    pub length: u32,
    pub tx_data: Vec<u8>,
}

impl SpiTransceive {
    pub fn new(port: u8, tx_data: &[u8]) -> Self {
        Self {
            port,
            length: tx_data.len() as u32,
            tx_data: tx_data.to_vec(),
        }
    }
}

impl ShdlcCommand for SpiTransceive {
    type Response = Vec<u8>;

    fn id(&self) -> u8 {
        0x61
    }

    fn data(&self) -> Vec<u8> {
        let mut d = Vec::with_capacity(5 + self.tx_data.len());
        d.push(self.port);
        let mut buf = [0u8; 4];
        BigEndian::write_u32(&mut buf, self.length);
        d.extend_from_slice(&buf);
        d.extend_from_slice(&self.tx_data);
        d
    }

    fn max_response_time(&self) -> Duration {
        Duration::from_secs(5)
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
