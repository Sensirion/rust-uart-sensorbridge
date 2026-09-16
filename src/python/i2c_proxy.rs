use crate::device::i2c_proxy::{
    I2C_STATUS_CHANNEL_DISABLED, I2C_STATUS_NACK, I2C_STATUS_OK, I2C_STATUS_TIMEOUT,
    I2C_STATUS_UNSPECIFIED_ERROR,
};
use crate::python::definitions::py_port_to_sensorbridge_port;
use crate::python::device::{PyAsyncSensorBridgeShdlcDevice, PySensorBridgeShdlcDevice};
use crate::python::errors::{SensorBridgeI2cNackErrorPy, SensorBridgeI2cTimeoutErrorPy};
use pyo3::prelude::*;
use pyo3::types::PyBytes;

#[pyclass(name = "SensorBridgeI2cProxy")]
pub struct PySensorBridgeI2cProxy {
    device: Py<PySensorBridgeShdlcDevice>,
    port_byte: u8,
}

#[pymethods]
impl PySensorBridgeI2cProxy {
    #[classattr]
    pub const API_VERSION: u32 = 1;
    #[classattr]
    pub const STATUS_OK: u8 = I2C_STATUS_OK;
    #[classattr]
    pub const STATUS_CHANNEL_DISABLED: u8 = I2C_STATUS_CHANNEL_DISABLED;
    #[classattr]
    pub const STATUS_NACK: u8 = I2C_STATUS_NACK;
    #[classattr]
    pub const STATUS_TIMEOUT: u8 = I2C_STATUS_TIMEOUT;
    #[classattr]
    pub const STATUS_UNSPECIFIED_ERROR: u8 = I2C_STATUS_UNSPECIFIED_ERROR;

    #[new]
    pub fn new(
        py: Python<'_>,
        device: Py<PySensorBridgeShdlcDevice>,
        port: Bound<'_, PyAny>,
    ) -> PyResult<Self> {
        let p = py_port_to_sensorbridge_port(&port, false)?;
        let port_byte = p
            .to_byte(false)
            .map_err(|e| crate::python::errors::to_py_sensorbridge_err(py, e))?;
        Ok(Self { device, port_byte })
    }

    #[getter]
    pub fn description(&self) -> &'static str {
        "SensorBridge"
    }

    #[getter]
    pub fn channel_count(&self) -> Option<usize> {
        None
    }

    #[pyo3(signature = (slave_address, tx_data=None, rx_length=None, read_delay=0.0, timeout=0.0))]
    pub fn transceive<'py>(
        &self,
        py: Python<'py>,
        slave_address: u8,
        tx_data: Option<&[u8]>,
        rx_length: Option<usize>,
        read_delay: f64,
        timeout: f64,
    ) -> PyResult<(u8, Option<PyObject>, Bound<'py, PyBytes>)> {
        let max_sec = read_delay.max(timeout);
        let total_timeout_us = max_sec * 1_000_000.0;
        let tx = tx_data.unwrap_or(&[]);
        let rx_len = rx_length.unwrap_or(0);

        let dev_bound = self.device.bind(py);
        let port_obj = self.port_byte.into_pyobject(py)?;

        let result = dev_bound.borrow_mut().transceive_i2c(
            py,
            port_obj.as_any().clone(),
            slave_address,
            tx,
            rx_len,
            total_timeout_us,
        );

        match result {
            Ok(bytes) => Ok((Self::STATUS_OK, None, bytes)),
            Err(e) => {
                let error_code: Option<u8> = e
                    .value(py)
                    .getattr("error_code")
                    .ok()
                    .and_then(|v| v.extract().ok());
                if error_code == Some(0x29)
                    || error_code == Some(0x01)
                    || e.is_instance_of::<SensorBridgeI2cNackErrorPy>(py)
                {
                    Ok((
                        Self::STATUS_NACK,
                        Some(e.into_pyobject(py)?.unbind().into_any()),
                        PyBytes::new(py, &[]),
                    ))
                } else if error_code == Some(0x2B)
                    || error_code == Some(0x02)
                    || e.is_instance_of::<SensorBridgeI2cTimeoutErrorPy>(py)
                {
                    Ok((
                        Self::STATUS_TIMEOUT,
                        Some(e.into_pyobject(py)?.unbind().into_any()),
                        PyBytes::new(py, &[]),
                    ))
                } else {
                    Ok((
                        Self::STATUS_UNSPECIFIED_ERROR,
                        Some(e.into_pyobject(py)?.unbind().into_any()),
                        PyBytes::new(py, &[]),
                    ))
                }
            }
        }
    }
}

#[pyclass(name = "AsyncSensorBridgeI2cProxy")]
pub struct PyAsyncSensorBridgeI2cProxy {
    device: Py<PyAsyncSensorBridgeShdlcDevice>,
    port_byte: u8,
}

#[pymethods]
impl PyAsyncSensorBridgeI2cProxy {
    #[classattr]
    pub const API_VERSION: u32 = 1;
    #[classattr]
    pub const STATUS_OK: u8 = I2C_STATUS_OK;
    #[classattr]
    pub const STATUS_CHANNEL_DISABLED: u8 = I2C_STATUS_CHANNEL_DISABLED;
    #[classattr]
    pub const STATUS_NACK: u8 = I2C_STATUS_NACK;
    #[classattr]
    pub const STATUS_TIMEOUT: u8 = I2C_STATUS_TIMEOUT;
    #[classattr]
    pub const STATUS_UNSPECIFIED_ERROR: u8 = I2C_STATUS_UNSPECIFIED_ERROR;

    #[new]
    pub fn new(
        py: Python<'_>,
        device: Py<PyAsyncSensorBridgeShdlcDevice>,
        port: Bound<'_, PyAny>,
    ) -> PyResult<Self> {
        let p = py_port_to_sensorbridge_port(&port, false)?;
        let port_byte = p
            .to_byte(false)
            .map_err(|e| crate::python::errors::to_py_sensorbridge_err(py, e))?;
        Ok(Self { device, port_byte })
    }

    #[getter]
    pub fn description(&self) -> &'static str {
        "SensorBridge"
    }

    #[getter]
    pub fn channel_count(&self) -> Option<usize> {
        None
    }

    #[pyo3(signature = (slave_address, tx_data=None, rx_length=None, read_delay=0.0, timeout=0.0))]
    pub fn transceive<'py>(
        &self,
        py: Python<'py>,
        slave_address: u8,
        tx_data: Option<Vec<u8>>,
        rx_length: Option<usize>,
        read_delay: f64,
        timeout: f64,
    ) -> PyResult<Bound<'py, PyAny>> {
        let max_sec = read_delay.max(timeout);
        let total_timeout_us = max_sec * 1_000_000.0;
        let tx = tx_data.unwrap_or_default();
        let rx_len = rx_length.unwrap_or(0);
        let dev_py = self.device.clone_ref(py);
        let port_b = self.port_byte;

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let coro = Python::with_gil(|py| -> PyResult<PyObject> {
                let dev_bound = dev_py.bind(py);
                let port_obj = port_b.into_pyobject(py)?;
                let fut = dev_bound.call_method1(
                    "transceive_i2c",
                    (
                        port_obj.as_any(),
                        slave_address,
                        PyBytes::new(py, &tx),
                        rx_len,
                        total_timeout_us,
                    ),
                )?;
                Ok(fut.unbind())
            })?;

            let fut_res = Python::with_gil(|py| {
                pyo3_async_runtimes::tokio::into_future(coro.bind(py).clone())
            })?
            .await;

            Python::with_gil(|py| -> PyResult<(u8, Option<PyObject>, PyObject)> {
                match fut_res {
                    Ok(rx) => Ok((I2C_STATUS_OK, None, rx)),
                    Err(e) => {
                        let error_code: Option<u8> = e
                            .value(py)
                            .getattr("error_code")
                            .ok()
                            .and_then(|v| v.extract().ok());
                        if error_code == Some(0x29)
                            || error_code == Some(0x01)
                            || e.is_instance_of::<SensorBridgeI2cNackErrorPy>(py)
                        {
                            Ok((
                                I2C_STATUS_NACK,
                                Some(e.into_pyobject(py)?.unbind().into_any()),
                                PyBytes::new(py, &[]).unbind().into(),
                            ))
                        } else if error_code == Some(0x2B)
                            || error_code == Some(0x02)
                            || e.is_instance_of::<SensorBridgeI2cTimeoutErrorPy>(py)
                        {
                            Ok((
                                I2C_STATUS_TIMEOUT,
                                Some(e.into_pyobject(py)?.unbind().into_any()),
                                PyBytes::new(py, &[]).unbind().into(),
                            ))
                        } else {
                            Ok((
                                I2C_STATUS_UNSPECIFIED_ERROR,
                                Some(e.into_pyobject(py)?.unbind().into_any()),
                                PyBytes::new(py, &[]).unbind().into(),
                            ))
                        }
                    }
                }
            })
        })
    }
}
