use rust_shdlc_driver::protocol::commands::ShdlcCommand;
use rust_shdlc_driver::protocol::errors::ShdlcError;
use std::time::Duration;

/// SHDLC Command 0x00: Set Port Voltage.
#[derive(Debug, Clone, Copy)]
pub struct SetPortVoltage {
    pub port: u8,
    pub voltage: u8,
}

impl SetPortVoltage {
    pub fn new(port: u8, voltage: u8) -> Self {
        Self { port, voltage }
    }
}

impl ShdlcCommand for SetPortVoltage {
    type Response = ();

    fn id(&self) -> u8 {
        0x00
    }

    fn data(&self) -> Vec<u8> {
        vec![self.port, self.voltage]
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

/// SHDLC Command 0x01: Port Voltage On/Off.
#[derive(Debug, Clone, Copy)]
pub struct PortVoltageOnOff {
    pub port: u8,
    pub state: u8,
}

impl PortVoltageOnOff {
    pub fn new(port: u8, state: u8) -> Self {
        Self { port, state }
    }
}

impl ShdlcCommand for PortVoltageOnOff {
    type Response = ();

    fn id(&self) -> u8 {
        0x01
    }

    fn data(&self) -> Vec<u8> {
        vec![self.port, self.state]
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
