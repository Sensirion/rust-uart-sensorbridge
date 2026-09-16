pub mod commands;
pub mod definitions;
pub mod device;
pub mod errors;
pub mod firmware;
pub mod i2c_proxy;
pub mod types;

use pyo3::prelude::*;

pub fn register_python_module(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Register exceptions
    errors::register_exceptions(py, m)?;

    // Register definitions
    definitions::register_definitions(py, m)?;

    // Register types
    m.add_class::<types::PyRepeatedTransceiveHandle>()?;
    m.add_class::<types::PyBufferedValue>()?;
    m.add_class::<types::PyReadBufferResponse>()?;

    // Register firmware
    m.add_class::<firmware::PySensorBridgeFirmwareImage>()?;

    // Register commands
    m.add_class::<commands::PySensorBridgeCmdSetPortVoltage>()?;
    m.add_class::<commands::PySensorBridgeCmdPortVoltageOnOff>()?;
    m.add_class::<commands::PySensorBridgeCmdSetI2cFrequency>()?;
    m.add_class::<commands::PySensorBridgeCmdI2cScan>()?;
    m.add_class::<commands::PySensorBridgeCmdFirstTransceive>()?;
    m.add_class::<commands::PySensorBridgeCmdSubsequentTransceive>()?;
    m.add_class::<commands::PySensorBridgeCmdI2cRepeatedTransceive>()?;
    m.add_class::<commands::PySensorBridgeCmdReadBuffer>()?;
    m.add_class::<commands::PySensorBridgeCmdStopRepeatedTransceive>()?;
    m.add_class::<commands::PySensorBridgeCmdSpiConfig>()?;
    m.add_class::<commands::PySensorBridgeCmdSpiTransceive>()?;
    m.add_class::<commands::PySensorBridgeCmdBlink>()?;
    m.add_class::<commands::PySensorBridgeCmdAnalogMeasurement>()?;

    // Register devices
    m.add_class::<device::PySensorBridgeShdlcDevice>()?;
    m.add_class::<device::PyAsyncSensorBridgeShdlcDevice>()?;

    // Register I2C proxy
    m.add_class::<i2c_proxy::PySensorBridgeI2cProxy>()?;
    m.add_class::<i2c_proxy::PyAsyncSensorBridgeI2cProxy>()?;

    // Module info
    m.add("__version__", "0.1.0")?;
    m.add("__copyright__", "(c) Copyright Sensirion AG")?;

    Ok(())
}
