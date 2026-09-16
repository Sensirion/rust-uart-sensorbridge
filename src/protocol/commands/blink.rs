use rust_shdlc_driver::protocol::commands::ShdlcCommand;
use rust_shdlc_driver::protocol::errors::ShdlcError;
use std::time::Duration;

/// SHDLC Command 0x70: Blink.
#[derive(Debug, Clone, Copy)]
pub struct Blink {
    pub port: u8,
}

impl Blink {
    pub fn new(port: u8) -> Self {
        Self { port }
    }
}

impl ShdlcCommand for Blink {
    type Response = ();

    fn id(&self) -> u8 {
        0x70
    }

    fn data(&self) -> Vec<u8> {
        vec![self.port]
    }

    fn max_response_time(&self) -> Duration {
        Duration::from_secs(1)
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
