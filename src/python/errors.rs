use crate::protocol::errors::SensorBridgeError;
use pyo3::create_exception;
use pyo3::exceptions::{PyIOError, PyValueError};
use pyo3::prelude::*;
use rust_shdlc_driver::python::errors::{to_py_err as shdlc_to_py_err, ShdlcDeviceErrorPy};

create_exception!(
    _sensirion_uart_sensorbridge,
    SensorBridgeDeviceErrorPy,
    ShdlcDeviceErrorPy
);
create_exception!(
    _sensirion_uart_sensorbridge,
    SensorBridgeFunctionalityNotImplementedErrorPy,
    SensorBridgeDeviceErrorPy
);
create_exception!(
    _sensirion_uart_sensorbridge,
    SensorBridgeNoMorePeriodicMeasurementsPossibleErrorPy,
    SensorBridgeDeviceErrorPy
);
create_exception!(
    _sensirion_uart_sensorbridge,
    SensorBridgeFrameChecksumErrorPy,
    SensorBridgeDeviceErrorPy
);
create_exception!(
    _sensirion_uart_sensorbridge,
    SensorBridgeFrameSizeInfoMismatchErrorPy,
    SensorBridgeDeviceErrorPy
);
create_exception!(
    _sensirion_uart_sensorbridge,
    SensorBridgeDeviceI2cNackErrorPy,
    SensorBridgeDeviceErrorPy
);
create_exception!(
    _sensirion_uart_sensorbridge,
    SensorBridgeDeviceI2cTimeoutErrorPy,
    SensorBridgeDeviceErrorPy
);
create_exception!(
    _sensirion_uart_sensorbridge,
    SensorBridgeFatalErrorPy,
    SensorBridgeDeviceErrorPy
);

// I2C Errors (subclasses of IOError)
create_exception!(
    _sensirion_uart_sensorbridge,
    SensorBridgeI2cErrorPy,
    PyIOError
);
create_exception!(
    _sensirion_uart_sensorbridge,
    SensorBridgeI2cNackErrorPy,
    SensorBridgeI2cErrorPy
);
create_exception!(
    _sensirion_uart_sensorbridge,
    SensorBridgeI2cTimeoutErrorPy,
    SensorBridgeI2cErrorPy
);
create_exception!(
    _sensirion_uart_sensorbridge,
    SensorBridgeI2cTimingErrorPy,
    SensorBridgeI2cErrorPy
);

pub fn to_py_sensorbridge_err(py: Python<'_>, err: SensorBridgeError) -> PyErr {
    match err {
        SensorBridgeError::Shdlc(rust_shdlc_driver::protocol::errors::ShdlcError::DeviceError {
            code,
            message,
        }) => {
            let py_err = match code {
                0x20 => SensorBridgeFunctionalityNotImplementedErrorPy::new_err(format!(
                    "SHDLC device error 0x20: {}",
                    message
                )),
                0x21 => SensorBridgeNoMorePeriodicMeasurementsPossibleErrorPy::new_err(format!(
                    "SHDLC device error 0x21: {}",
                    message
                )),
                0x22 => SensorBridgeFrameChecksumErrorPy::new_err(format!(
                    "SHDLC device error 0x22: {}",
                    message
                )),
                0x26 => SensorBridgeFrameSizeInfoMismatchErrorPy::new_err(format!(
                    "SHDLC device error 0x26: {}",
                    message
                )),
                0x29 => SensorBridgeDeviceI2cNackErrorPy::new_err(format!(
                    "SHDLC device error 0x29: {}",
                    message
                )),
                0x2B => SensorBridgeDeviceI2cTimeoutErrorPy::new_err(format!(
                    "SHDLC device error 0x2B: {}",
                    message
                )),
                0x7F => SensorBridgeFatalErrorPy::new_err(format!(
                    "SHDLC device error 0x7F: {}",
                    message
                )),
                _ => SensorBridgeDeviceErrorPy::new_err(format!(
                    "SHDLC device error {:#04x}: {}",
                    code, message
                )),
            };
            let _ = py_err.value(py).setattr("error_code", code);
            let _ = py_err.value(py).setattr("error_message", message);
            py_err
        }
        SensorBridgeError::Shdlc(shdlc_err) => shdlc_to_py_err(py, shdlc_err),
        SensorBridgeError::InvalidPort(p) => {
            PyValueError::new_err(format!("Invalid port '{:#04x}' specified.", p))
        }
        SensorBridgeError::InvalidVoltage(v) => {
            PyValueError::new_err(format!("Invalid voltage '{}' specified.", v))
        }
        SensorBridgeError::InvalidI2cFrequency(f) => {
            PyValueError::new_err(format!("Invalid I2C frequency '{}' specified.", f))
        }
        SensorBridgeError::InvalidSpiFrequency(f) => {
            PyValueError::new_err(format!("Invalid SPI frequency '{}' specified.", f))
        }
        SensorBridgeError::InvalidSpiMode(m) => {
            PyValueError::new_err(format!("Invalid SPI mode '{}' specified.", m))
        }
        SensorBridgeError::I2cNack => {
            let py_err = SensorBridgeI2cNackErrorPy::new_err("I2C transceive error: NACK (byte not acknowledged)");
            let _ = py_err.value(py).setattr("error_code", 0x01);
            let _ = py_err.value(py).setattr("error_message", "NACK (byte not acknowledged)");
            py_err
        }
        SensorBridgeError::I2cTimeout => {
            let py_err = SensorBridgeI2cTimeoutErrorPy::new_err("I2C transceive error: Timeout");
            let _ = py_err.value(py).setattr("error_code", 0x02);
            let _ = py_err.value(py).setattr("error_message", "Timeout");
            py_err
        }
        SensorBridgeError::I2cTiming => {
            let py_err = SensorBridgeI2cTimingErrorPy::new_err(
                "I2C transceive error: Invalid timing (frequency, interval, timeout or delay)",
            );
            let _ = py_err.value(py).setattr("error_code", 0x03);
            let _ = py_err
                .value(py)
                .setattr("error_message", "Invalid timing (frequency, interval, timeout or delay)");
            py_err
        }
        SensorBridgeError::I2cGeneric { code, message } => {
            let py_err =
                SensorBridgeI2cErrorPy::new_err(format!("I2C transceive error: {}", message));
            let _ = py_err.value(py).setattr("error_code", code);
            let _ = py_err.value(py).setattr("error_message", message);
            py_err
        }
        SensorBridgeError::InvalidResponse(msg) => {
            shdlc_to_py_err(py, rust_shdlc_driver::protocol::errors::ShdlcError::ResponseError {
                message: msg,
                raw_data: None,
            })
        }
        SensorBridgeError::BufferLengthMismatch { expected, actual } => {
            shdlc_to_py_err(py, rust_shdlc_driver::protocol::errors::ShdlcError::ResponseError {
                message: format!(
                    "Received data length ({}) is not a multiple of the expected packet length ({}).",
                    actual, expected
                ),
                raw_data: None,
            })
        }
    }
}

pub fn py_i2c_error_from_code(py: Python<'_>, code: u8) -> PyResult<Option<PyObject>> {
    if code == 0 {
        return Ok(None);
    }
    let err = match code {
        0x01 => SensorBridgeI2cNackErrorPy::new_err(
            "I2C transceive error: NACK (byte not acknowledged)",
        ),
        0x02 => SensorBridgeI2cTimeoutErrorPy::new_err("I2C transceive error: Timeout"),
        0x03 => SensorBridgeI2cTimingErrorPy::new_err(
            "I2C transceive error: Invalid timing (frequency, interval, timeout or delay)",
        ),
        _ => SensorBridgeI2cErrorPy::new_err(format!(
            "I2C transceive error: Unknown ({:#04x})",
            code
        )),
    };
    let _ = err.value(py).setattr("error_code", code);
    Ok(Some(err.into_pyobject(py)?.unbind().into_any()))
}

pub fn register_exceptions(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add(
        "SensorBridgeDeviceError",
        m.py().get_type::<SensorBridgeDeviceErrorPy>(),
    )?;
    m.add(
        "SensorBridgeFunctionalityNotImplementedError",
        m.py()
            .get_type::<SensorBridgeFunctionalityNotImplementedErrorPy>(),
    )?;
    m.add(
        "SensorBridgeNoMorePeriodicMeasurementsPossibleError",
        m.py()
            .get_type::<SensorBridgeNoMorePeriodicMeasurementsPossibleErrorPy>(),
    )?;
    m.add(
        "SensorBridgeFrameChecksumError",
        m.py().get_type::<SensorBridgeFrameChecksumErrorPy>(),
    )?;
    m.add(
        "SensorBridgeFrameSizeInfoMismatchError",
        m.py()
            .get_type::<SensorBridgeFrameSizeInfoMismatchErrorPy>(),
    )?;
    m.add(
        "SensorBridgeDeviceI2cNackError",
        m.py().get_type::<SensorBridgeDeviceI2cNackErrorPy>(),
    )?;
    m.add(
        "SensorBridgeDeviceI2cTimeoutError",
        m.py().get_type::<SensorBridgeDeviceI2cTimeoutErrorPy>(),
    )?;
    m.add(
        "SensorBridgeFatalError",
        m.py().get_type::<SensorBridgeFatalErrorPy>(),
    )?;

    m.add(
        "SensorBridgeI2cError",
        m.py().get_type::<SensorBridgeI2cErrorPy>(),
    )?;
    m.add(
        "SensorBridgeI2cNackError",
        m.py().get_type::<SensorBridgeI2cNackErrorPy>(),
    )?;
    m.add(
        "SensorBridgeI2cTimeoutError",
        m.py().get_type::<SensorBridgeI2cTimeoutErrorPy>(),
    )?;
    m.add(
        "SensorBridgeI2cTimingError",
        m.py().get_type::<SensorBridgeI2cTimingErrorPy>(),
    )?;

    #[pyfunction]
    #[pyo3(name = "i2c_error_from_code")]
    fn py_fn_i2c_error_from_code(py: Python<'_>, code: u8) -> PyResult<Option<PyObject>> {
        py_i2c_error_from_code(py, code)
    }
    m.add_function(wrap_pyfunction!(py_fn_i2c_error_from_code, m)?)?;

    Ok(())
}
