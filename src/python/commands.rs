use crate::protocol::commands::*;
use crate::python::errors::to_py_sensorbridge_err;
use pyo3::prelude::*;
use rust_shdlc_driver::protocol::commands::ShdlcCommand;
use rust_shdlc_driver::python::commands::PyShdlcCommand;

#[pyclass(name = "SensorBridgeCmdSetPortVoltage", extends = PyShdlcCommand)]
pub struct PySensorBridgeCmdSetPortVoltage;

#[pymethods]
impl PySensorBridgeCmdSetPortVoltage {
    #[new]
    pub fn new(port: u8, voltage: u8) -> (Self, PyShdlcCommand) {
        let cmd = SetPortVoltage::new(port, voltage);
        let base = PyShdlcCommand::new(
            cmd.id(),
            cmd.data(),
            cmd.max_response_time().as_secs_f64(),
            cmd.min_response_length(),
            cmd.max_response_length(),
            cmd.post_processing_time().as_secs_f64(),
        );
        (PySensorBridgeCmdSetPortVoltage, base)
    }
}

#[pyclass(name = "SensorBridgeCmdPortVoltageOnOff", extends = PyShdlcCommand)]
pub struct PySensorBridgeCmdPortVoltageOnOff;

#[pymethods]
impl PySensorBridgeCmdPortVoltageOnOff {
    #[new]
    pub fn new(port: u8, state: u8) -> (Self, PyShdlcCommand) {
        let cmd = PortVoltageOnOff::new(port, state);
        let base = PyShdlcCommand::new(
            cmd.id(),
            cmd.data(),
            cmd.max_response_time().as_secs_f64(),
            cmd.min_response_length(),
            cmd.max_response_length(),
            cmd.post_processing_time().as_secs_f64(),
        );
        (PySensorBridgeCmdPortVoltageOnOff, base)
    }
}

#[pyclass(name = "SensorBridgeCmdSetI2cFrequency", extends = PyShdlcCommand)]
pub struct PySensorBridgeCmdSetI2cFrequency;

#[pymethods]
impl PySensorBridgeCmdSetI2cFrequency {
    #[new]
    pub fn new(port: u8, frequency: u8) -> (Self, PyShdlcCommand) {
        let cmd = SetI2cFrequency::new(port, frequency);
        let base = PyShdlcCommand::new(
            cmd.id(),
            cmd.data(),
            cmd.max_response_time().as_secs_f64(),
            cmd.min_response_length(),
            cmd.max_response_length(),
            cmd.post_processing_time().as_secs_f64(),
        );
        (PySensorBridgeCmdSetI2cFrequency, base)
    }
}

#[pyclass(name = "SensorBridgeCmdI2cScan", extends = PyShdlcCommand)]
pub struct PySensorBridgeCmdI2cScan;

#[pymethods]
impl PySensorBridgeCmdI2cScan {
    #[new]
    pub fn new(port: u8, first_address: u8, last_address: u8) -> (Self, PyShdlcCommand) {
        let cmd = I2cScan::new(port, first_address, last_address);
        let base = PyShdlcCommand::new(
            cmd.id(),
            cmd.data(),
            cmd.max_response_time().as_secs_f64(),
            cmd.min_response_length(),
            cmd.max_response_length(),
            cmd.post_processing_time().as_secs_f64(),
        );
        (PySensorBridgeCmdI2cScan, base)
    }

    #[staticmethod]
    pub fn interpret_response(data: &[u8]) -> Vec<u8> {
        data.to_vec()
    }
}

#[pyclass(name = "SensorBridgeCmdFirstTransceive", extends = PyShdlcCommand)]
pub struct PySensorBridgeCmdFirstTransceive;

#[pymethods]
impl PySensorBridgeCmdFirstTransceive {
    #[new]
    pub fn new(
        port: u8,
        address: u8,
        tx_length: u32,
        rx_length: u32,
        timeout_us: u32,
        tx_data: &[u8],
    ) -> (Self, PyShdlcCommand) {
        let cmd = FirstTransceive::new(port, address, tx_length, rx_length, timeout_us, tx_data);
        let base = PyShdlcCommand::new(
            cmd.id(),
            cmd.data(),
            cmd.max_response_time().as_secs_f64(),
            cmd.min_response_length(),
            cmd.max_response_length(),
            cmd.post_processing_time().as_secs_f64(),
        );
        (PySensorBridgeCmdFirstTransceive, base)
    }

    #[staticmethod]
    pub fn interpret_response(data: &[u8]) -> Vec<u8> {
        data.to_vec()
    }
}

#[pyclass(name = "SensorBridgeCmdSubsequentTransceive", extends = PyShdlcCommand)]
pub struct PySensorBridgeCmdSubsequentTransceive;

#[pymethods]
impl PySensorBridgeCmdSubsequentTransceive {
    #[new]
    pub fn new(port: u8, tx_data: &[u8]) -> (Self, PyShdlcCommand) {
        let cmd = SubsequentTransceive::new(port, tx_data);
        let base = PyShdlcCommand::new(
            cmd.id(),
            cmd.data(),
            cmd.max_response_time().as_secs_f64(),
            cmd.min_response_length(),
            cmd.max_response_length(),
            cmd.post_processing_time().as_secs_f64(),
        );
        (PySensorBridgeCmdSubsequentTransceive, base)
    }

    #[staticmethod]
    pub fn interpret_response(data: &[u8]) -> Vec<u8> {
        data.to_vec()
    }
}

#[pyclass(name = "SensorBridgeCmdI2cRepeatedTransceive", extends = PyShdlcCommand)]
pub struct PySensorBridgeCmdI2cRepeatedTransceive;

#[pymethods]
impl PySensorBridgeCmdI2cRepeatedTransceive {
    #[allow(clippy::too_many_arguments)]
    #[new]
    pub fn new(
        interval_us: u32,
        port: u8,
        address: u8,
        tx_length: u32,
        rx_length: u32,
        timeout_us: u32,
        read_delay_us: u32,
        tx_data: &[u8],
    ) -> (Self, PyShdlcCommand) {
        let cmd = I2cRepeatedTransceive::new(
            interval_us,
            port,
            address,
            tx_length,
            rx_length,
            timeout_us,
            read_delay_us,
            tx_data,
        );
        let base = PyShdlcCommand::new(
            cmd.id(),
            cmd.data(),
            cmd.max_response_time().as_secs_f64(),
            cmd.min_response_length(),
            cmd.max_response_length(),
            cmd.post_processing_time().as_secs_f64(),
        );
        (PySensorBridgeCmdI2cRepeatedTransceive, base)
    }

    #[staticmethod]
    pub fn interpret_response(data: &[u8]) -> Vec<u8> {
        data.to_vec()
    }
}

#[pyclass(name = "SensorBridgeCmdReadBuffer", extends = PyShdlcCommand)]
pub struct PySensorBridgeCmdReadBuffer;

#[pymethods]
impl PySensorBridgeCmdReadBuffer {
    #[new]
    pub fn new(handle: u8) -> (Self, PyShdlcCommand) {
        let cmd = ReadBuffer::new(handle);
        let base = PyShdlcCommand::new(
            cmd.id(),
            cmd.data(),
            cmd.max_response_time().as_secs_f64(),
            cmd.min_response_length(),
            cmd.max_response_length(),
            cmd.post_processing_time().as_secs_f64(),
        );
        (PySensorBridgeCmdReadBuffer, base)
    }

    #[staticmethod]
    pub fn interpret_response(py: Python<'_>, data: &[u8]) -> PyResult<(u32, u32, Vec<u8>)> {
        let cmd = ReadBuffer::new(0);
        cmd.interpret_response(data).map_err(|e| {
            to_py_sensorbridge_err(py, crate::protocol::errors::SensorBridgeError::Shdlc(e))
        })
    }
}

#[pyclass(name = "SensorBridgeCmdStopRepeatedTransceive", extends = PyShdlcCommand)]
pub struct PySensorBridgeCmdStopRepeatedTransceive;

#[pymethods]
impl PySensorBridgeCmdStopRepeatedTransceive {
    #[new]
    pub fn new(handle: u8) -> (Self, PyShdlcCommand) {
        let cmd = StopRepeatedTransceive::new(handle);
        let base = PyShdlcCommand::new(
            cmd.id(),
            cmd.data(),
            cmd.max_response_time().as_secs_f64(),
            cmd.min_response_length(),
            cmd.max_response_length(),
            cmd.post_processing_time().as_secs_f64(),
        );
        (PySensorBridgeCmdStopRepeatedTransceive, base)
    }
}

#[pyclass(name = "SensorBridgeCmdSpiConfig", extends = PyShdlcCommand)]
pub struct PySensorBridgeCmdSpiConfig;

#[pymethods]
impl PySensorBridgeCmdSpiConfig {
    #[new]
    pub fn new(port: u8, mode: u8, frequency: u8) -> (Self, PyShdlcCommand) {
        let cmd = SpiConfig::new(port, mode, frequency);
        let base = PyShdlcCommand::new(
            cmd.id(),
            cmd.data(),
            cmd.max_response_time().as_secs_f64(),
            cmd.min_response_length(),
            cmd.max_response_length(),
            cmd.post_processing_time().as_secs_f64(),
        );
        (PySensorBridgeCmdSpiConfig, base)
    }
}

#[pyclass(name = "SensorBridgeCmdSpiTransceive", extends = PyShdlcCommand)]
pub struct PySensorBridgeCmdSpiTransceive;

#[pymethods]
impl PySensorBridgeCmdSpiTransceive {
    #[new]
    pub fn new(port: u8, tx_data: &[u8]) -> (Self, PyShdlcCommand) {
        let cmd = SpiTransceive::new(port, tx_data);
        let base = PyShdlcCommand::new(
            cmd.id(),
            cmd.data(),
            cmd.max_response_time().as_secs_f64(),
            cmd.min_response_length(),
            cmd.max_response_length(),
            cmd.post_processing_time().as_secs_f64(),
        );
        (PySensorBridgeCmdSpiTransceive, base)
    }

    #[staticmethod]
    pub fn interpret_response(data: &[u8]) -> Vec<u8> {
        data.to_vec()
    }
}

#[pyclass(name = "SensorBridgeCmdBlink", extends = PyShdlcCommand)]
pub struct PySensorBridgeCmdBlink;

#[pymethods]
impl PySensorBridgeCmdBlink {
    #[new]
    pub fn new(port: u8) -> (Self, PyShdlcCommand) {
        let cmd = Blink::new(port);
        let base = PyShdlcCommand::new(
            cmd.id(),
            cmd.data(),
            cmd.max_response_time().as_secs_f64(),
            cmd.min_response_length(),
            cmd.max_response_length(),
            cmd.post_processing_time().as_secs_f64(),
        );
        (PySensorBridgeCmdBlink, base)
    }
}

#[pyclass(name = "SensorBridgeCmdAnalogMeasurement", extends = PyShdlcCommand)]
pub struct PySensorBridgeCmdAnalogMeasurement;

#[pymethods]
impl PySensorBridgeCmdAnalogMeasurement {
    #[new]
    pub fn new(port: u8) -> (Self, PyShdlcCommand) {
        let cmd = AnalogMeasurement::new(port);
        let base = PyShdlcCommand::new(
            cmd.id(),
            cmd.data(),
            cmd.max_response_time().as_secs_f64(),
            cmd.min_response_length(),
            cmd.max_response_length(),
            cmd.post_processing_time().as_secs_f64(),
        );
        (PySensorBridgeCmdAnalogMeasurement, base)
    }

    #[staticmethod]
    pub fn interpret_response(py: Python<'_>, data: &[u8]) -> PyResult<f32> {
        let cmd = AnalogMeasurement::new(0);
        cmd.interpret_response(data).map_err(|e| {
            to_py_sensorbridge_err(py, crate::protocol::errors::SensorBridgeError::Shdlc(e))
        })
    }
}
