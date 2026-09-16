use crate::firmware::SensorBridgeFirmwareImage;
use crate::python::errors::to_py_sensorbridge_err;
use pyo3::prelude::*;
use rust_shdlc_driver::python::types::PyFirmwareVersion;

#[pyclass(name = "SensorBridgeFirmwareImage")]
#[derive(Debug, Clone)]
pub struct PySensorBridgeFirmwareImage {
    pub inner: SensorBridgeFirmwareImage,
}

#[pymethods]
impl PySensorBridgeFirmwareImage {
    #[new]
    pub fn new(py: Python<'_>, hexfile: Bound<'_, PyAny>) -> PyResult<Self> {
        let hex_content: String = if let Ok(s) = hexfile.extract::<String>() {
            // Check if it's a file path or raw hex content
            if std::path::Path::new(&s).is_file() {
                std::fs::read_to_string(&s).map_err(|e| {
                    to_py_sensorbridge_err(
                        py,
                        crate::protocol::errors::SensorBridgeError::Shdlc(
                            rust_shdlc_driver::protocol::errors::ShdlcError::Io(e.to_string()),
                        ),
                    )
                })?
            } else {
                s
            }
        } else if hexfile.hasattr("read")? {
            let res = hexfile.call_method0("read")?;
            if let Ok(text) = res.extract::<String>() {
                text
            } else if let Ok(bytes) = res.extract::<Vec<u8>>() {
                String::from_utf8(bytes).map_err(|e| {
                    to_py_sensorbridge_err(
                        py,
                        crate::protocol::errors::SensorBridgeError::Shdlc(
                            rust_shdlc_driver::protocol::errors::ShdlcError::Io(e.to_string()),
                        ),
                    )
                })?
            } else {
                return Err(to_py_sensorbridge_err(
                    py,
                    crate::protocol::errors::SensorBridgeError::Shdlc(
                        rust_shdlc_driver::protocol::errors::ShdlcError::Other(
                            "Failed to read hex file object".to_string(),
                        ),
                    ),
                ));
            }
        } else {
            return Err(to_py_sensorbridge_err(
                py,
                crate::protocol::errors::SensorBridgeError::Shdlc(
                    rust_shdlc_driver::protocol::errors::ShdlcError::Other(
                        "Expected file path string or file-like object".to_string(),
                    ),
                ),
            ));
        };

        let inner = SensorBridgeFirmwareImage::from_hex_str(&hex_content).map_err(|e| {
            to_py_sensorbridge_err(py, crate::protocol::errors::SensorBridgeError::Shdlc(e))
        })?;

        Ok(Self { inner })
    }

    #[getter]
    pub fn product_type(&self) -> u32 {
        self.inner.product_type()
    }

    #[getter]
    pub fn bootloader_version(&self) -> PyFirmwareVersion {
        self.inner.bootloader_version().into()
    }

    #[getter]
    pub fn application_version(&self) -> PyFirmwareVersion {
        self.inner.application_version().into()
    }

    #[getter]
    pub fn checksum(&self) -> u8 {
        self.inner.checksum()
    }

    #[getter]
    pub fn size(&self) -> usize {
        self.inner.size()
    }

    fn __repr__(&self) -> String {
        format!(
            "SensorBridgeFirmwareImage(product_type=0x{:08X}, bl_ver={:?}, app_ver={:?}, size={})",
            self.inner.product_type(),
            self.inner.bootloader_version(),
            self.inner.application_version(),
            self.inner.size()
        )
    }
}
