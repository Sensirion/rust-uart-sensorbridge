use crate::protocol::definitions::SensorBridgePort;
use crate::python::errors::to_py_sensorbridge_err;
use pyo3::prelude::*;
use pyo3::types::PyDict;

#[pyclass(name = "SensorBridgePort", eq, eq_int)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PySensorBridgePort {
    ONE = 0x00,
    TWO = 0x01,
    ALL = 0xFF,
}

pub fn py_port_to_sensorbridge_port(
    port_obj: &Bound<'_, PyAny>,
    accept_all: bool,
) -> PyResult<SensorBridgePort> {
    let raw_val: u8 = if let Ok(val) = port_obj.extract::<u8>() {
        val
    } else if let Ok(py_port) = port_obj.extract::<PySensorBridgePort>() {
        py_port as u8
    } else {
        return Err(to_py_sensorbridge_err(
            port_obj.py(),
            crate::protocol::errors::SensorBridgeError::InvalidPort(254),
        ));
    };

    SensorBridgePort::from_byte(raw_val, accept_all)
        .map_err(|e| to_py_sensorbridge_err(port_obj.py(), e))
}

#[pyfunction]
#[pyo3(signature = (port, accept_all=false))]
pub fn port_to_byte(port: Bound<'_, PyAny>, accept_all: bool) -> PyResult<u8> {
    let p = py_port_to_sensorbridge_port(&port, accept_all)?;
    p.to_byte(accept_all)
        .map_err(|e| to_py_sensorbridge_err(port.py(), e))
}

#[pyfunction]
pub fn voltage_to_byte(py: Python<'_>, voltage: f64) -> PyResult<u8> {
    crate::protocol::definitions::voltage_to_byte(voltage)
        .map_err(|e| to_py_sensorbridge_err(py, e))
}

#[pyfunction]
pub fn i2c_frequency_to_byte(py: Python<'_>, frequency: f64) -> PyResult<u8> {
    let freq_u32 = frequency.round() as u32;
    crate::protocol::definitions::i2c_frequency_to_byte(freq_u32)
        .map_err(|e| to_py_sensorbridge_err(py, e))
}

#[pyfunction]
pub fn spi_frequency_to_byte(py: Python<'_>, frequency: f64) -> PyResult<u8> {
    let freq_u32 = frequency.round() as u32;
    crate::protocol::definitions::spi_frequency_to_byte(freq_u32)
        .map_err(|e| to_py_sensorbridge_err(py, e))
}

pub fn register_definitions(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PySensorBridgePort>()?;

    // Create VOLTAGES dictionary
    let voltages_dict = PyDict::new(py);
    voltages_dict.set_item(1.2, 0x00)?;
    voltages_dict.set_item(1.5, 0x04)?;
    voltages_dict.set_item(1.8, 0x01)?;
    voltages_dict.set_item(2.1, 0x05)?;
    voltages_dict.set_item(2.4, 0x06)?;
    voltages_dict.set_item(2.7, 0x07)?;
    voltages_dict.set_item(3.0, 0x08)?;
    voltages_dict.set_item(3.3, 0x02)?;
    voltages_dict.set_item(3.6, 0x09)?;
    voltages_dict.set_item(4.5, 0x0A)?;
    voltages_dict.set_item(5.0, 0x03)?;
    voltages_dict.set_item(5.5, 0x0B)?;
    m.add("VOLTAGES", voltages_dict)?;

    // Create I2C_FREQUENCIES dictionary
    let i2c_freq_dict = PyDict::new(py);
    i2c_freq_dict.set_item(10_000, 0x04)?;
    i2c_freq_dict.set_item(50_000, 0x05)?;
    i2c_freq_dict.set_item(100_000, 0x00)?;
    i2c_freq_dict.set_item(400_000, 0x01)?;
    i2c_freq_dict.set_item(1_000_000, 0x02)?;
    i2c_freq_dict.set_item(2_000_000, 0x03)?;
    m.add("I2C_FREQUENCIES", i2c_freq_dict)?;

    // Create SPI_FREQUENCIES dictionary
    let spi_freq_dict = PyDict::new(py);
    spi_freq_dict.set_item(330_000, 0x00)?;
    spi_freq_dict.set_item(600_000, 0x01)?;
    spi_freq_dict.set_item(1_000_000, 0x02)?;
    spi_freq_dict.set_item(5_000_000, 0x03)?;
    spi_freq_dict.set_item(10_000_000, 0x04)?;
    spi_freq_dict.set_item(21_000_000, 0x05)?;
    m.add("SPI_FREQUENCIES", spi_freq_dict)?;

    m.add_function(wrap_pyfunction!(self::port_to_byte, m)?)?;
    m.add_function(wrap_pyfunction!(self::voltage_to_byte, m)?)?;
    m.add_function(wrap_pyfunction!(self::i2c_frequency_to_byte, m)?)?;
    m.add_function(wrap_pyfunction!(self::spi_frequency_to_byte, m)?)?;

    Ok(())
}
