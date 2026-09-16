pub mod device;
pub mod firmware;
pub mod protocol;

#[cfg(feature = "python")]
pub mod python;

#[cfg(feature = "python")]
use pyo3::prelude::*;

#[cfg(feature = "python")]
#[pymodule]
fn _sensirion_uart_sensorbridge(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    python::register_python_module(py, m)
}
