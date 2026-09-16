use crate::protocol::types::{
    BufferedValue as NativeBufferedValue, ReadBufferResponse as NativeReadBufferResponse,
    RepeatedTransceiveHandle as NativeRepeatedTransceiveHandle,
};
use crate::python::errors::{py_i2c_error_from_code, to_py_sensorbridge_err};
use pyo3::prelude::*;
use pyo3::types::PyBytes;

#[pyclass(name = "RepeatedTransceiveHandle")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PyRepeatedTransceiveHandle {
    #[pyo3(get)]
    pub raw_handle: u8,
    #[pyo3(get)]
    pub rx_length: usize,
}

#[pymethods]
impl PyRepeatedTransceiveHandle {
    #[new]
    pub fn new(raw_handle: u8, rx_length: usize) -> Self {
        Self {
            raw_handle,
            rx_length,
        }
    }

    #[getter]
    pub fn slot(&self) -> u8 {
        (self.raw_handle >> 4) & 0x0F
    }

    #[getter]
    pub fn channel(&self) -> u8 {
        self.raw_handle & 0x0F
    }

    fn __repr__(&self) -> String {
        format!(
            "RepeatedTransceiveHandle(raw_handle=0x{:02X}, rx_length={}, slot={}, channel={})",
            self.raw_handle,
            self.rx_length,
            self.slot(),
            self.channel()
        )
    }
}

impl From<NativeRepeatedTransceiveHandle> for PyRepeatedTransceiveHandle {
    fn from(h: NativeRepeatedTransceiveHandle) -> Self {
        Self {
            raw_handle: h.raw_handle,
            rx_length: h.rx_length,
        }
    }
}

impl From<PyRepeatedTransceiveHandle> for NativeRepeatedTransceiveHandle {
    fn from(h: PyRepeatedTransceiveHandle) -> Self {
        Self {
            raw_handle: h.raw_handle,
            rx_length: h.rx_length,
        }
    }
}

#[pyclass(name = "BufferedValue")]
#[derive(Debug, Clone)]
pub struct PyBufferedValue {
    #[pyo3(get)]
    pub raw_status: u8,
    #[pyo3(get)]
    pub raw_data: Vec<u8>,
}

#[pymethods]
impl PyBufferedValue {
    #[new]
    pub fn new(py: Python<'_>, rx_data: &[u8]) -> PyResult<Self> {
        let native =
            NativeBufferedValue::new(rx_data).map_err(|e| to_py_sensorbridge_err(py, e))?;
        Ok(Self {
            raw_status: native.raw_status,
            raw_data: native.raw_data,
        })
    }

    #[getter]
    pub fn error(&self, py: Python<'_>) -> PyResult<Option<PyObject>> {
        py_i2c_error_from_code(py, self.raw_status)
    }

    #[getter]
    pub fn data<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyBytes>> {
        if let Some(err_obj) = self.error(py)? {
            Err(PyErr::from_value(err_obj.bind(py).clone()))
        } else {
            Ok(PyBytes::new(py, &self.raw_data))
        }
    }

    fn __repr__(&self) -> String {
        format!(
            "BufferedValue(raw_status=0x{:02X}, raw_data_len={})",
            self.raw_status,
            self.raw_data.len()
        )
    }
}

impl From<NativeBufferedValue> for PyBufferedValue {
    fn from(b: NativeBufferedValue) -> Self {
        Self {
            raw_status: b.raw_status,
            raw_data: b.raw_data,
        }
    }
}

#[pyclass(name = "ReadBufferResponse")]
#[derive(Debug, Clone)]
pub struct PyReadBufferResponse {
    #[pyo3(get)]
    pub lost_bytes: u32,
    #[pyo3(get)]
    pub remaining_bytes: u32,
    #[pyo3(get)]
    pub values: Vec<PyBufferedValue>,
}

#[pymethods]
impl PyReadBufferResponse {
    #[new]
    pub fn new(
        py: Python<'_>,
        rx_length: usize,
        lost_bytes: u32,
        remaining_bytes: u32,
        rx_data: &[u8],
    ) -> PyResult<Self> {
        let native =
            NativeReadBufferResponse::from_raw(rx_length, lost_bytes, remaining_bytes, rx_data)
                .map_err(|e| to_py_sensorbridge_err(py, e))?;
        Ok(Self {
            lost_bytes: native.lost_bytes,
            remaining_bytes: native.remaining_bytes,
            values: native.values.into_iter().map(Into::into).collect(),
        })
    }

    fn __repr__(&self) -> String {
        format!(
            "ReadBufferResponse(lost_bytes={}, remaining_bytes={}, value_count={})",
            self.lost_bytes,
            self.remaining_bytes,
            self.values.len()
        )
    }
}

impl From<NativeReadBufferResponse> for PyReadBufferResponse {
    fn from(r: NativeReadBufferResponse) -> Self {
        Self {
            lost_bytes: r.lost_bytes,
            remaining_bytes: r.remaining_bytes,
            values: r.values.into_iter().map(Into::into).collect(),
        }
    }
}
