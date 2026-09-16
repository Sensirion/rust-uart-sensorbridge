use byteorder::{ByteOrder, LittleEndian};
use rust_shdlc_driver::protocol::commands::ShdlcCommand;
use rust_shdlc_driver::protocol::errors::ShdlcError;
use std::time::Duration;

/// SHDLC Command 0x80: Analog Measurement.
#[derive(Debug, Clone, Copy)]
pub struct AnalogMeasurement {
    pub port: u8,
}

impl AnalogMeasurement {
    pub fn new(port: u8) -> Self {
        Self { port }
    }
}

impl ShdlcCommand for AnalogMeasurement {
    type Response = f32;

    fn id(&self) -> u8 {
        0x80
    }

    fn data(&self) -> Vec<u8> {
        vec![self.port]
    }

    fn max_response_time(&self) -> Duration {
        Duration::from_millis(100)
    }

    fn min_response_length(&self) -> usize {
        4
    }

    fn max_response_length(&self) -> usize {
        4
    }

    fn interpret_response(&self, data: &[u8]) -> Result<Self::Response, ShdlcError> {
        if data.len() < 4 {
            return Err(ShdlcError::response_error(
                "Invalid length for analog measurement response",
                Some(data.to_vec()),
            ));
        }
        // SensorBridge returns IEEE-754 float in Little-Endian byte order
        let val = LittleEndian::read_f32(&data[0..4]);
        Ok(val)
    }
}
