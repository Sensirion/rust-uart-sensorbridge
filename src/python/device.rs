use crate::protocol::definitions::{SensorBridgePort, SensorBridgeSpiMode};
use crate::python::definitions::{
    i2c_frequency_to_byte, port_to_byte, py_port_to_sensorbridge_port, spi_frequency_to_byte,
    voltage_to_byte,
};
use crate::python::errors::{to_py_sensorbridge_err, SensorBridgeDeviceErrorPy};
use crate::python::firmware::PySensorBridgeFirmwareImage;
use crate::python::types::{PyReadBufferResponse, PyRepeatedTransceiveHandle};
use pyo3::prelude::*;
use pyo3::types::PyBytes;
use std::collections::HashMap;

// ===========================================================================
// Synchronous SensorBridgeShdlcDevice
// ===========================================================================
#[pyclass(name = "SensorBridgeShdlcDevice")]
pub struct PySensorBridgeShdlcDevice {
    pub conn_obj: PyObject,
    pub slave_address: u8,
    pub last_error_flag: bool,
    pub device_errors: HashMap<u8, String>,
}

#[pymethods]
impl PySensorBridgeShdlcDevice {
    #[new]
    #[pyo3(signature = (connection, slave_address=0))]
    pub fn new(connection: PyObject, slave_address: u8) -> Self {
        let mut device_errors = HashMap::new();
        for (code, msg) in crate::protocol::errors::sensorbridge_device_error_map() {
            device_errors.insert(code, msg);
        }
        Self {
            conn_obj: connection,
            slave_address,
            last_error_flag: false,
            device_errors,
        }
    }

    #[getter(connection)]
    pub fn connection_prop(&self, py: Python<'_>) -> PyObject {
        self.conn_obj.clone_ref(py)
    }

    #[getter(slave_address)]
    pub fn slave_address_prop(&self) -> u8 {
        self.slave_address
    }

    #[getter(last_error_flag)]
    pub fn last_error_flag_prop(&self) -> bool {
        self.last_error_flag
    }

    pub fn execute(&mut self, py: Python<'_>, command: Bound<'_, PyAny>) -> PyResult<PyObject> {
        let res = self
            .conn_obj
            .bind(py)
            .call_method1("execute", (self.slave_address, command, true))?;
        let (data, err_flag): (PyObject, bool) = res.extract()?;
        self.last_error_flag = err_flag;
        Ok(data)
    }

    // --- Standard SHDLC Device Commands ---

    #[pyo3(signature = (as_int=false))]
    pub fn get_product_type(&mut self, py: Python<'_>, as_int: bool) -> PyResult<PyObject> {
        let py_cmd = Py::new(
            py,
            rust_shdlc_driver::python::commands::PyShdlcCmdGetProductType::new(),
        )?;
        let data = self.execute(py, py_cmd.bind(py).clone().into_any())?;
        if as_int {
            let s: String = data.extract(py)?;
            let val = u32::from_str_radix(&s, 16).unwrap_or(0);
            Ok(val.into_pyobject(py)?.unbind().into())
        } else {
            Ok(data)
        }
    }

    pub fn get_product_subtype(&mut self, py: Python<'_>) -> PyResult<u8> {
        let py_cmd = Py::new(
            py,
            rust_shdlc_driver::python::commands::PyShdlcCmdGetProductSubType::new(),
        )?;
        let data = self.execute(py, py_cmd.bind(py).clone().into_any())?;
        data.extract(py)
    }

    pub fn get_product_name(&mut self, py: Python<'_>) -> PyResult<String> {
        let py_cmd = Py::new(
            py,
            rust_shdlc_driver::python::commands::PyShdlcCmdGetProductName::new(),
        )?;
        let data = self.execute(py, py_cmd.bind(py).clone().into_any())?;
        data.extract(py)
    }

    pub fn get_article_code(&mut self, py: Python<'_>) -> PyResult<String> {
        let py_cmd = Py::new(
            py,
            rust_shdlc_driver::python::commands::PyShdlcCmdGetArticleCode::new(),
        )?;
        let data = self.execute(py, py_cmd.bind(py).clone().into_any())?;
        data.extract(py)
    }

    pub fn get_serial_number(&mut self, py: Python<'_>) -> PyResult<String> {
        let py_cmd = Py::new(
            py,
            rust_shdlc_driver::python::commands::PyShdlcCmdGetSerialNumber::new(),
        )?;
        let data = self.execute(py, py_cmd.bind(py).clone().into_any())?;
        data.extract(py)
    }

    pub fn get_version(&mut self, py: Python<'_>) -> PyResult<PyObject> {
        let py_cmd = Py::new(
            py,
            rust_shdlc_driver::python::commands::PyShdlcCmdGetVersion::new(),
        )?;
        self.execute(py, py_cmd.bind(py).clone().into_any())
    }

    #[pyo3(signature = (clear=true, as_exception=false))]
    pub fn get_error_state(
        &mut self,
        py: Python<'_>,
        clear: bool,
        as_exception: bool,
    ) -> PyResult<PyObject> {
        let py_cmd = Py::new(
            py,
            rust_shdlc_driver::python::commands::PyShdlcCmdGetErrorState::new(clear),
        )?;
        let data = self.execute(py, py_cmd.bind(py).clone().into_any())?;
        if as_exception {
            let (state, last_err): (u32, u8) = data.extract(py)?;
            if last_err != 0 {
                let err_msg = self
                    .device_errors
                    .get(&last_err)
                    .cloned()
                    .unwrap_or_else(|| format!("Unknown device error {}", last_err));
                let py_err = SensorBridgeDeviceErrorPy::new_err(format!(
                    "SHDLC device returned error code {}: {}",
                    last_err, err_msg
                ));
                let _ = py_err.value(py).setattr("error_code", last_err);
                let _ = py_err.value(py).setattr("error_message", err_msg);
                let tuple = (state, py_err.value(py).clone().unbind());
                Ok(tuple.into_pyobject(py)?.unbind().into())
            } else {
                let tuple = (state, py.None());
                Ok(tuple.into_pyobject(py)?.unbind().into())
            }
        } else {
            Ok(data)
        }
    }

    pub fn get_slave_address(&mut self, py: Python<'_>) -> PyResult<u8> {
        let py_cmd = Py::new(
            py,
            rust_shdlc_driver::python::commands::PyShdlcCmdGetSlaveAddress::new(),
        )?;
        let data = self.execute(py, py_cmd.bind(py).clone().into_any())?;
        data.extract(py)
    }

    #[pyo3(signature = (slave_address, update_driver=true))]
    pub fn set_slave_address(
        &mut self,
        py: Python<'_>,
        slave_address: u8,
        update_driver: bool,
    ) -> PyResult<()> {
        let py_cmd = Py::new(
            py,
            rust_shdlc_driver::python::commands::PyShdlcCmdSetSlaveAddress::new(slave_address),
        )?;
        let _ = self.execute(py, py_cmd.bind(py).clone().into_any())?;
        if update_driver {
            self.slave_address = slave_address;
        }
        Ok(())
    }

    pub fn get_baudrate(&mut self, py: Python<'_>) -> PyResult<u32> {
        let py_cmd = Py::new(
            py,
            rust_shdlc_driver::python::commands::PyShdlcCmdGetBaudrate::new(),
        )?;
        let data = self.execute(py, py_cmd.bind(py).clone().into_any())?;
        data.extract(py)
    }

    #[pyo3(signature = (baudrate, update_driver=true))]
    pub fn set_baudrate(
        &mut self,
        py: Python<'_>,
        baudrate: u32,
        update_driver: bool,
    ) -> PyResult<()> {
        let py_cmd = Py::new(
            py,
            rust_shdlc_driver::python::commands::PyShdlcCmdSetBaudrate::new(baudrate),
        )?;
        let _ = self.execute(py, py_cmd.bind(py).clone().into_any())?;
        if update_driver && self.conn_obj.bind(py).hasattr("port")? {
            let port = self.conn_obj.bind(py).getattr("port")?;
            if port.hasattr("bitrate")? {
                port.setattr("bitrate", baudrate)?;
            }
        }
        Ok(())
    }

    pub fn get_reply_delay(&mut self, py: Python<'_>) -> PyResult<u16> {
        let py_cmd = Py::new(
            py,
            rust_shdlc_driver::python::commands::PyShdlcCmdGetReplyDelay::new(),
        )?;
        let data = self.execute(py, py_cmd.bind(py).clone().into_any())?;
        data.extract(py)
    }

    pub fn set_reply_delay(&mut self, py: Python<'_>, reply_delay: u16) -> PyResult<()> {
        let py_cmd = Py::new(
            py,
            rust_shdlc_driver::python::commands::PyShdlcCmdSetReplyDelay::new(reply_delay),
        )?;
        let _ = self.execute(py, py_cmd.bind(py).clone().into_any())?;
        Ok(())
    }

    pub fn get_system_up_time(&mut self, py: Python<'_>) -> PyResult<u32> {
        let py_cmd = Py::new(
            py,
            rust_shdlc_driver::python::commands::PyShdlcCmdGetSystemUpTime::new(),
        )?;
        let data = self.execute(py, py_cmd.bind(py).clone().into_any())?;
        data.extract(py)
    }

    pub fn device_reset(&mut self, py: Python<'_>) -> PyResult<()> {
        let py_cmd = Py::new(
            py,
            rust_shdlc_driver::python::commands::PyShdlcCmdDeviceReset::new(),
        )?;
        let _ = self.execute(py, py_cmd.bind(py).clone().into_any())?;
        Ok(())
    }

    pub fn factory_reset(&mut self, py: Python<'_>) -> PyResult<()> {
        let py_cmd = Py::new(
            py,
            rust_shdlc_driver::python::commands::PyShdlcCmdFactoryReset::new(),
        )?;
        let _ = self.execute(py, py_cmd.bind(py).clone().into_any())?;
        Ok(())
    }

    // --- SensorBridge Commands ---

    pub fn blink_led(&mut self, py: Python<'_>, port: Bound<'_, PyAny>) -> PyResult<()> {
        let raw_port = port_to_byte(port, true)?;
        let py_cmd = Py::new(
            py,
            crate::python::commands::PySensorBridgeCmdBlink::new(raw_port),
        )?;
        let _ = self.execute(py, py_cmd.bind(py).clone().into_any())?;
        Ok(())
    }

    pub fn measure_voltage(&mut self, py: Python<'_>, port: Bound<'_, PyAny>) -> PyResult<f32> {
        let raw_port = port_to_byte(port, false)?;
        let py_cmd = Py::new(
            py,
            crate::python::commands::PySensorBridgeCmdAnalogMeasurement::new(raw_port),
        )?;
        let val = self.execute(py, py_cmd.bind(py).clone().into_any())?;
        val.extract(py)
    }

    pub fn set_supply_voltage(
        &mut self,
        py: Python<'_>,
        port: Bound<'_, PyAny>,
        voltage: f64,
    ) -> PyResult<()> {
        let raw_port = port_to_byte(port, true)?;
        let raw_voltage = voltage_to_byte(py, voltage)?;
        let py_cmd = Py::new(
            py,
            crate::python::commands::PySensorBridgeCmdSetPortVoltage::new(raw_port, raw_voltage),
        )?;
        let _ = self.execute(py, py_cmd.bind(py).clone().into_any())?;
        Ok(())
    }

    pub fn switch_supply_on(&mut self, py: Python<'_>, port: Bound<'_, PyAny>) -> PyResult<()> {
        let raw_port = port_to_byte(port, true)?;
        let py_cmd = Py::new(
            py,
            crate::python::commands::PySensorBridgeCmdPortVoltageOnOff::new(raw_port, 1),
        )?;
        let _ = self.execute(py, py_cmd.bind(py).clone().into_any())?;
        Ok(())
    }

    pub fn switch_supply_off(&mut self, py: Python<'_>, port: Bound<'_, PyAny>) -> PyResult<()> {
        let raw_port = port_to_byte(port, true)?;
        let py_cmd = Py::new(
            py,
            crate::python::commands::PySensorBridgeCmdPortVoltageOnOff::new(raw_port, 0),
        )?;
        let _ = self.execute(py, py_cmd.bind(py).clone().into_any())?;
        Ok(())
    }

    pub fn set_i2c_frequency(
        &mut self,
        py: Python<'_>,
        port: Bound<'_, PyAny>,
        frequency: f64,
    ) -> PyResult<()> {
        let raw_port = port_to_byte(port, true)?;
        let raw_freq = i2c_frequency_to_byte(py, frequency)?;
        let py_cmd = Py::new(
            py,
            crate::python::commands::PySensorBridgeCmdSetI2cFrequency::new(raw_port, raw_freq),
        )?;
        let _ = self.execute(py, py_cmd.bind(py).clone().into_any())?;
        Ok(())
    }

    #[pyo3(signature = (port, first_address=1, last_address=127))]
    pub fn scan_i2c(
        &mut self,
        py: Python<'_>,
        port: Bound<'_, PyAny>,
        first_address: u8,
        last_address: u8,
    ) -> PyResult<PyObject> {
        let raw_port = port_to_byte(port, false)?;
        let py_cmd = Py::new(
            py,
            crate::python::commands::PySensorBridgeCmdI2cScan::new(
                raw_port,
                first_address,
                last_address,
            ),
        )?;
        let res = self.execute(py, py_cmd.bind(py).clone().into_any())?;
        let bytes: Vec<u8> = res.extract(py)?;
        let py_list = pyo3::types::PyList::new(py, bytes)?;
        Ok(py_list.unbind().into())
    }

    pub fn transceive_i2c<'py>(
        &mut self,
        py: Python<'py>,
        port: Bound<'_, PyAny>,
        address: u8,
        tx_data: &[u8],
        rx_length: usize,
        timeout_us: f64,
    ) -> PyResult<Bound<'py, PyBytes>> {
        let raw_port = port_to_byte(port, false)?;
        let max_tx_len_per_frame = 255 - 15; // 240 bytes
        let timeout_u32 = timeout_us.round() as u32;

        let tx_len_sent = std::cmp::min(tx_data.len(), max_tx_len_per_frame);
        let first_chunk = &tx_data[0..tx_len_sent];

        let py_cmd = Py::new(
            py,
            crate::python::commands::PySensorBridgeCmdFirstTransceive::new(
                raw_port,
                address,
                tx_data.len() as u32,
                rx_length as u32,
                timeout_u32,
                first_chunk,
            ),
        )?;
        let res = self.execute(py, py_cmd.bind(py).clone().into_any())?;
        let mut rx_data: Vec<u8> = res.extract(py)?;

        // Send remaining TX
        let mut offset = tx_len_sent;
        while offset < tx_data.len() {
            let chunk_len = std::cmp::min(tx_data.len() - offset, max_tx_len_per_frame);
            let py_sub = Py::new(
                py,
                crate::python::commands::PySensorBridgeCmdSubsequentTransceive::new(
                    raw_port,
                    &tx_data[offset..offset + chunk_len],
                ),
            )?;
            let sub_res = self.execute(py, py_sub.bind(py).clone().into_any())?;
            let sub_bytes: Vec<u8> = sub_res.extract(py)?;
            rx_data.extend(sub_bytes);
            offset += chunk_len;
        }

        // Receive remaining RX
        while rx_data.len() < rx_length {
            let py_sub = Py::new(
                py,
                crate::python::commands::PySensorBridgeCmdSubsequentTransceive::new(raw_port, &[]),
            )?;
            let sub_res = self.execute(py, py_sub.bind(py).clone().into_any())?;
            let sub_bytes: Vec<u8> = sub_res.extract(py)?;
            if sub_bytes.is_empty() {
                break;
            }
            rx_data.extend(sub_bytes);
        }

        Ok(PyBytes::new(py, &rx_data))
    }

    #[allow(clippy::too_many_arguments)]
    #[pyo3(signature = (port, interval_us, address, tx_data, rx_length, timeout_us, read_delay_us=0.0))]
    pub fn start_repeated_i2c_transceive(
        &mut self,
        py: Python<'_>,
        port: Bound<'_, PyAny>,
        interval_us: f64,
        address: u8,
        tx_data: &[u8],
        rx_length: usize,
        timeout_us: f64,
        read_delay_us: f64,
    ) -> PyResult<PyObject> {
        let is_all = {
            if let Ok(p) = py_port_to_sensorbridge_port(&port, true) {
                p == SensorBridgePort::All
            } else {
                false
            }
        };

        let raw_port = port_to_byte(port, true)?;
        let py_cmd = Py::new(
            py,
            crate::python::commands::PySensorBridgeCmdI2cRepeatedTransceive::new(
                interval_us.round() as u32,
                raw_port,
                address,
                tx_data.len() as u32,
                rx_length as u32,
                timeout_us.round() as u32,
                read_delay_us.round() as u32,
                tx_data,
            ),
        )?;
        let res = self.execute(py, py_cmd.bind(py).clone().into_any())?;
        let raw_handles: Vec<u8> = res.extract(py)?;

        let py_handles: Vec<PyRepeatedTransceiveHandle> = raw_handles
            .into_iter()
            .map(|h| PyRepeatedTransceiveHandle::new(h, rx_length))
            .collect();

        if is_all {
            let py_objs: Vec<PyObject> = py_handles
                .into_iter()
                .map(|h| Py::new(py, h).map(|p| p.into_any()))
                .collect::<PyResult<Vec<_>>>()?;
            let tuple = (py_objs[0].clone_ref(py), py_objs[1].clone_ref(py));
            Ok(tuple.into_pyobject(py)?.unbind().into())
        } else if let Some(h) = py_handles.into_iter().next() {
            let py_h = Py::new(py, h)?;
            Ok(py_h.into_any())
        } else {
            Ok(py.None())
        }
    }

    #[pyo3(signature = (handle=None))]
    pub fn stop_repeated_i2c_transceive(
        &mut self,
        py: Python<'_>,
        handle: Option<&PyRepeatedTransceiveHandle>,
    ) -> PyResult<()> {
        let raw_handle = handle.map(|h| h.raw_handle).unwrap_or(0xFF);
        let py_cmd = Py::new(
            py,
            crate::python::commands::PySensorBridgeCmdStopRepeatedTransceive::new(raw_handle),
        )?;
        let _ = self.execute(py, py_cmd.bind(py).clone().into_any())?;
        Ok(())
    }

    #[pyo3(signature = (handle, max_reads=100))]
    pub fn read_buffer(
        &mut self,
        py: Python<'_>,
        handle: &PyRepeatedTransceiveHandle,
        max_reads: usize,
    ) -> PyResult<PyReadBufferResponse> {
        let mut total_lost_bytes = 0;
        let mut remaining_bytes = 0;
        let mut total_rx_data = Vec::new();

        for _ in 0..max_reads {
            let py_cmd = Py::new(
                py,
                crate::python::commands::PySensorBridgeCmdReadBuffer::new(handle.raw_handle),
            )?;
            let res = self.execute(py, py_cmd.bind(py).clone().into_any())?;
            let (lost, remaining, rx): (u32, u32, Vec<u8>) = res.extract(py)?;
            total_lost_bytes += lost;
            remaining_bytes = remaining;
            total_rx_data.extend(rx);
            if remaining == 0 {
                break;
            }
        }

        PyReadBufferResponse::new(
            py,
            handle.rx_length,
            total_lost_bytes,
            remaining_bytes,
            &total_rx_data,
        )
    }

    pub fn set_spi_config(
        &mut self,
        py: Python<'_>,
        port: Bound<'_, PyAny>,
        mode: u8,
        frequency: f64,
    ) -> PyResult<()> {
        let raw_port = port_to_byte(port, true)?;
        let _ = SensorBridgeSpiMode::from_u8(mode).map_err(|e| to_py_sensorbridge_err(py, e))?;
        let raw_freq = spi_frequency_to_byte(py, frequency)?;
        let py_cmd = Py::new(
            py,
            crate::python::commands::PySensorBridgeCmdSpiConfig::new(raw_port, mode, raw_freq),
        )?;
        let _ = self.execute(py, py_cmd.bind(py).clone().into_any())?;
        Ok(())
    }

    pub fn transceive_spi<'py>(
        &mut self,
        py: Python<'py>,
        port: Bound<'_, PyAny>,
        tx_data: &[u8],
    ) -> PyResult<Bound<'py, PyBytes>> {
        let raw_port = port_to_byte(port, false)?;
        let py_cmd = Py::new(
            py,
            crate::python::commands::PySensorBridgeCmdSpiTransceive::new(raw_port, tx_data),
        )?;
        let res = self.execute(py, py_cmd.bind(py).clone().into_any())?;
        let rx: Vec<u8> = res.extract(py)?;
        Ok(PyBytes::new(py, &rx))
    }

    #[pyo3(signature = (image, emergency=false, status_callback=None, progress_callback=None))]
    pub fn update_firmware(
        slf: PyRef<'_, Self>,
        py: Python<'_>,
        image: Bound<'_, PyAny>,
        emergency: bool,
        status_callback: Option<PyObject>,
        progress_callback: Option<PyObject>,
    ) -> PyResult<()> {
        let firmware_image = if let Ok(img) = image.extract::<PySensorBridgeFirmwareImage>() {
            img
        } else {
            PySensorBridgeFirmwareImage::new(py, image)?
        };

        let rust_driver_mod = py.import("rust_shdlc_driver")?;
        let updater_cls = rust_driver_mod.getattr("ShdlcFirmwareUpdate")?;

        let py_img = Py::new(py, firmware_image)?;
        let kwargs = pyo3::types::PyDict::new(py);
        if let Some(scb) = status_callback {
            kwargs.set_item("status_callback", scb)?;
        }
        if let Some(pcb) = progress_callback {
            kwargs.set_item("progress_callback", pcb)?;
        }

        let updater = updater_cls.call((slf.connection_prop(py), py_img), Some(&kwargs))?;
        updater.call_method1("execute", (emergency,))?;
        Ok(())
    }
}

// ===========================================================================
// Asynchronous AsyncSensorBridgeShdlcDevice
// ===========================================================================
#[pyclass(name = "AsyncSensorBridgeShdlcDevice")]
pub struct PyAsyncSensorBridgeShdlcDevice {
    pub conn_obj: PyObject,
    pub slave_address: u8,
    pub last_error_flag: bool,
    pub device_errors: HashMap<u8, String>,
}

#[pymethods]
impl PyAsyncSensorBridgeShdlcDevice {
    #[new]
    #[pyo3(signature = (connection, slave_address=0))]
    pub fn new(connection: PyObject, slave_address: u8) -> Self {
        let mut device_errors = HashMap::new();
        for (code, msg) in crate::protocol::errors::sensorbridge_device_error_map() {
            device_errors.insert(code, msg);
        }
        Self {
            conn_obj: connection,
            slave_address,
            last_error_flag: false,
            device_errors,
        }
    }

    #[getter(connection)]
    pub fn connection_prop(&self, py: Python<'_>) -> PyObject {
        self.conn_obj.clone_ref(py)
    }

    #[getter(slave_address)]
    pub fn slave_address_prop(&self) -> u8 {
        self.slave_address
    }

    #[getter(last_error_flag)]
    pub fn last_error_flag_prop(&self) -> bool {
        self.last_error_flag
    }

    pub fn execute<'py>(
        &self,
        py: Python<'py>,
        command: Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let conn_py = self.conn_obj.clone_ref(py);
        let slave_addr = self.slave_address;
        let cmd_py = command.unbind();

        let coro = Python::with_gil(|py| -> PyResult<PyObject> {
            let bound_conn = conn_py.bind(py);
            let bound_cmd = cmd_py.bind(py);
            let fut = bound_conn.call_method1("execute", (slave_addr, bound_cmd, true))?;
            Ok(fut.unbind())
        })?;

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let res_py = Python::with_gil(|py| {
                pyo3_async_runtimes::tokio::into_future(coro.bind(py).clone())
            })?
            .await?;

            Python::with_gil(|py| -> PyResult<PyObject> {
                let (data, _err_flag): (PyObject, bool) = res_py.extract(py)?;
                Ok(data)
            })
        })
    }

    pub fn blink_led<'py>(
        &self,
        py: Python<'py>,
        port: Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let raw_port = port_to_byte(port, true)?;
        let py_cmd = Py::new(
            py,
            crate::python::commands::PySensorBridgeCmdBlink::new(raw_port),
        )?;
        self.execute(py, py_cmd.bind(py).clone().into_any())
    }

    pub fn measure_voltage<'py>(
        &self,
        py: Python<'py>,
        port: Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let raw_port = port_to_byte(port, false)?;
        let py_cmd = Py::new(
            py,
            crate::python::commands::PySensorBridgeCmdAnalogMeasurement::new(raw_port),
        )?;
        self.execute(py, py_cmd.bind(py).clone().into_any())
    }

    pub fn set_supply_voltage<'py>(
        &self,
        py: Python<'py>,
        port: Bound<'py, PyAny>,
        voltage: f64,
    ) -> PyResult<Bound<'py, PyAny>> {
        let raw_port = port_to_byte(port, true)?;
        let raw_voltage = voltage_to_byte(py, voltage)?;
        let py_cmd = Py::new(
            py,
            crate::python::commands::PySensorBridgeCmdSetPortVoltage::new(raw_port, raw_voltage),
        )?;
        self.execute(py, py_cmd.bind(py).clone().into_any())
    }

    pub fn switch_supply_on<'py>(
        &self,
        py: Python<'py>,
        port: Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let raw_port = port_to_byte(port, true)?;
        let py_cmd = Py::new(
            py,
            crate::python::commands::PySensorBridgeCmdPortVoltageOnOff::new(raw_port, 1),
        )?;
        self.execute(py, py_cmd.bind(py).clone().into_any())
    }

    pub fn switch_supply_off<'py>(
        &self,
        py: Python<'py>,
        port: Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let raw_port = port_to_byte(port, true)?;
        let py_cmd = Py::new(
            py,
            crate::python::commands::PySensorBridgeCmdPortVoltageOnOff::new(raw_port, 0),
        )?;
        self.execute(py, py_cmd.bind(py).clone().into_any())
    }

    pub fn set_i2c_frequency<'py>(
        &self,
        py: Python<'py>,
        port: Bound<'py, PyAny>,
        frequency: f64,
    ) -> PyResult<Bound<'py, PyAny>> {
        let raw_port = port_to_byte(port, true)?;
        let raw_freq = i2c_frequency_to_byte(py, frequency)?;
        let py_cmd = Py::new(
            py,
            crate::python::commands::PySensorBridgeCmdSetI2cFrequency::new(raw_port, raw_freq),
        )?;
        self.execute(py, py_cmd.bind(py).clone().into_any())
    }

    #[pyo3(signature = (port, first_address=1, last_address=127))]
    pub fn scan_i2c<'py>(
        &self,
        py: Python<'py>,
        port: Bound<'py, PyAny>,
        first_address: u8,
        last_address: u8,
    ) -> PyResult<Bound<'py, PyAny>> {
        let raw_port = port_to_byte(port, false)?;
        let py_cmd = Py::new(
            py,
            crate::python::commands::PySensorBridgeCmdI2cScan::new(
                raw_port,
                first_address,
                last_address,
            ),
        )?;
        let conn_py = self.conn_obj.clone_ref(py);
        let slave_addr = self.slave_address;

        let coro = Python::with_gil(|py| -> PyResult<PyObject> {
            let bound_conn = conn_py.bind(py);
            let bound_cmd = py_cmd.bind(py);
            let fut = bound_conn.call_method1("execute", (slave_addr, bound_cmd, true))?;
            Ok(fut.unbind())
        })?;

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let res_py = Python::with_gil(|py| {
                pyo3_async_runtimes::tokio::into_future(coro.bind(py).clone())
            })?
            .await?;

            Python::with_gil(|py| -> PyResult<PyObject> {
                let (data, _): (Vec<u8>, bool) = res_py.extract(py)?;
                let py_list = pyo3::types::PyList::new(py, data)?;
                Ok(py_list.unbind().into())
            })
        })
    }

    pub fn transceive_i2c<'py>(
        &self,
        py: Python<'py>,
        port: Bound<'py, PyAny>,
        address: u8,
        tx_data: Vec<u8>,
        rx_length: usize,
        timeout_us: f64,
    ) -> PyResult<Bound<'py, PyAny>> {
        let raw_port = port_to_byte(port, false)?;
        let conn_py = self.conn_obj.clone_ref(py);
        let slave_addr = self.slave_address;
        let timeout_u32 = timeout_us.round() as u32;
        let asyncio = py.import("asyncio")?;
        let event_loop = asyncio
            .call_method0("get_running_loop")
            .or_else(|_| asyncio.call_method0("get_event_loop"))?
            .unbind();

        let conn_clone1 = conn_py.clone_ref(py);
        let loop_clone1 = event_loop.clone_ref(py);
        let conn_clone2 = conn_py.clone_ref(py);
        let loop_clone2 = event_loop.clone_ref(py);
        let conn_clone3 = conn_py.clone_ref(py);
        let loop_clone3 = event_loop.clone_ref(py);

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let max_tx_len_per_frame = 255 - 15;
            let total_tx_len = tx_data.len() as u32;
            let tx_len_sent = std::cmp::min(tx_data.len(), max_tx_len_per_frame);
            let first_chunk = tx_data[0..tx_len_sent].to_vec();

            let mut rx_data = tokio::task::spawn_blocking(move || {
                Python::with_gil(|py| -> PyResult<Vec<u8>> {
                    let asyncio = py.import("asyncio")?;
                    let py_cmd = Py::new(
                        py,
                        crate::python::commands::PySensorBridgeCmdFirstTransceive::new(
                            raw_port,
                            address,
                            total_tx_len,
                            rx_length as u32,
                            timeout_u32,
                            &first_chunk,
                        ),
                    )?;
                    let coro = conn_clone1
                        .bind(py)
                        .call_method1("execute", (slave_addr, py_cmd.bind(py), true))?;
                    let loop_bound = loop_clone1.bind(py);
                    let fut =
                        asyncio.call_method1("run_coroutine_threadsafe", (coro, loop_bound))?;
                    let res = fut.call_method0("result")?;
                    let (bytes, _): (Vec<u8>, bool) = res.extract()?;
                    Ok(bytes)
                })
            })
            .await
            .map_err(|e| {
                Python::with_gil(|py| {
                    to_py_sensorbridge_err(
                        py,
                        crate::protocol::errors::SensorBridgeError::Shdlc(
                            rust_shdlc_driver::protocol::errors::ShdlcError::Other(e.to_string()),
                        ),
                    )
                })
            })??;

            // Send remaining TX
            let mut offset = tx_len_sent;
            while offset < tx_data.len() {
                let chunk_len = std::cmp::min(tx_data.len() - offset, max_tx_len_per_frame);
                let chunk = tx_data[offset..offset + chunk_len].to_vec();
                let conn_c = Python::with_gil(|py| conn_clone2.clone_ref(py));
                let loop_c = Python::with_gil(|py| loop_clone2.clone_ref(py));
                let sub_bytes = tokio::task::spawn_blocking(move || {
                    Python::with_gil(|py| -> PyResult<Vec<u8>> {
                        let asyncio = py.import("asyncio")?;
                        let py_sub = Py::new(
                            py,
                            crate::python::commands::PySensorBridgeCmdSubsequentTransceive::new(
                                raw_port, &chunk,
                            ),
                        )?;
                        let coro = conn_c
                            .bind(py)
                            .call_method1("execute", (slave_addr, py_sub.bind(py), true))?;
                        let loop_bound = loop_c.bind(py);
                        let fut =
                            asyncio.call_method1("run_coroutine_threadsafe", (coro, loop_bound))?;
                        let res = fut.call_method0("result")?;
                        let (bytes, _): (Vec<u8>, bool) = res.extract()?;
                        Ok(bytes)
                    })
                })
                .await
                .map_err(|e| {
                    Python::with_gil(|py| {
                        to_py_sensorbridge_err(
                            py,
                            crate::protocol::errors::SensorBridgeError::Shdlc(
                                rust_shdlc_driver::protocol::errors::ShdlcError::Other(
                                    e.to_string(),
                                ),
                            ),
                        )
                    })
                })??;
                rx_data.extend(sub_bytes);
                offset += chunk_len;
            }

            // Receive remaining RX
            while rx_data.len() < rx_length {
                let conn_c = Python::with_gil(|py| conn_clone3.clone_ref(py));
                let loop_c = Python::with_gil(|py| loop_clone3.clone_ref(py));
                let sub_bytes = tokio::task::spawn_blocking(move || {
                    Python::with_gil(|py| -> PyResult<Vec<u8>> {
                        let asyncio = py.import("asyncio")?;
                        let py_sub = Py::new(
                            py,
                            crate::python::commands::PySensorBridgeCmdSubsequentTransceive::new(
                                raw_port,
                                &[],
                            ),
                        )?;
                        let coro = conn_c
                            .bind(py)
                            .call_method1("execute", (slave_addr, py_sub.bind(py), true))?;
                        let loop_bound = loop_c.bind(py);
                        let fut =
                            asyncio.call_method1("run_coroutine_threadsafe", (coro, loop_bound))?;
                        let res = fut.call_method0("result")?;
                        let (bytes, _): (Vec<u8>, bool) = res.extract()?;
                        Ok(bytes)
                    })
                })
                .await
                .map_err(|e| {
                    Python::with_gil(|py| {
                        to_py_sensorbridge_err(
                            py,
                            crate::protocol::errors::SensorBridgeError::Shdlc(
                                rust_shdlc_driver::protocol::errors::ShdlcError::Other(
                                    e.to_string(),
                                ),
                            ),
                        )
                    })
                })??;
                if sub_bytes.is_empty() {
                    break;
                }
                rx_data.extend(sub_bytes);
            }

            Python::with_gil(|py| Ok(PyBytes::new(py, &rx_data).unbind()))
        })
    }

    #[allow(clippy::too_many_arguments)]
    #[pyo3(signature = (port, interval_us, address, tx_data, rx_length, timeout_us, read_delay_us=0.0))]
    pub fn start_repeated_i2c_transceive<'py>(
        &self,
        py: Python<'py>,
        port: Bound<'py, PyAny>,
        interval_us: f64,
        address: u8,
        tx_data: Vec<u8>,
        rx_length: usize,
        timeout_us: f64,
        read_delay_us: f64,
    ) -> PyResult<Bound<'py, PyAny>> {
        let is_all = {
            if let Ok(p) = py_port_to_sensorbridge_port(&port, true) {
                p == SensorBridgePort::All
            } else {
                false
            }
        };
        let raw_port = port_to_byte(port, true)?;
        let conn_py = self.conn_obj.clone_ref(py);
        let slave_addr = self.slave_address;

        let coro = Python::with_gil(|py| -> PyResult<PyObject> {
            let py_cmd = Py::new(
                py,
                crate::python::commands::PySensorBridgeCmdI2cRepeatedTransceive::new(
                    interval_us.round() as u32,
                    raw_port,
                    address,
                    tx_data.len() as u32,
                    rx_length as u32,
                    timeout_us.round() as u32,
                    read_delay_us.round() as u32,
                    &tx_data,
                ),
            )?;
            let fut = conn_py
                .bind(py)
                .call_method1("execute", (slave_addr, py_cmd.bind(py), true))?;
            Ok(fut.unbind())
        })?;

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let res = Python::with_gil(|py| {
                pyo3_async_runtimes::tokio::into_future(coro.bind(py).clone())
            })?
            .await?;
            let raw_handles: Vec<u8> =
                Python::with_gil(|py| res.extract::<(Vec<u8>, bool)>(py).map(|r| r.0))?;

            let py_handles: Vec<PyRepeatedTransceiveHandle> = raw_handles
                .into_iter()
                .map(|h| PyRepeatedTransceiveHandle::new(h, rx_length))
                .collect();

            Python::with_gil(|py| -> PyResult<PyObject> {
                if is_all {
                    let py_objs: Vec<PyObject> = py_handles
                        .into_iter()
                        .map(|h| Py::new(py, h).map(|p| p.into_any()))
                        .collect::<PyResult<Vec<_>>>()?;
                    let tuple = (py_objs[0].clone_ref(py), py_objs[1].clone_ref(py));
                    Ok(tuple.into_pyobject(py)?.unbind().into())
                } else if let Some(h) = py_handles.into_iter().next() {
                    let py_h = Py::new(py, h)?;
                    Ok(py_h.into_any())
                } else {
                    Ok(py.None())
                }
            })
        })
    }

    #[pyo3(signature = (handle=None))]
    pub fn stop_repeated_i2c_transceive<'py>(
        &self,
        py: Python<'py>,
        handle: Option<PyRepeatedTransceiveHandle>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let raw_handle = handle.map(|h| h.raw_handle).unwrap_or(0xFF);
        let py_cmd = Py::new(
            py,
            crate::python::commands::PySensorBridgeCmdStopRepeatedTransceive::new(raw_handle),
        )?;
        self.execute(py, py_cmd.bind(py).clone().into_any())
    }

    #[pyo3(signature = (handle, max_reads=100))]
    pub fn read_buffer<'py>(
        &self,
        py: Python<'py>,
        handle: PyRepeatedTransceiveHandle,
        max_reads: usize,
    ) -> PyResult<Bound<'py, PyAny>> {
        let conn_py = self.conn_obj.clone_ref(py);
        let slave_addr = self.slave_address;

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let mut total_lost_bytes = 0;
            let mut remaining_bytes = 0;
            let mut total_rx_data = Vec::new();

            for _ in 0..max_reads {
                let coro = Python::with_gil(|py| -> PyResult<PyObject> {
                    let py_cmd = Py::new(
                        py,
                        crate::python::commands::PySensorBridgeCmdReadBuffer::new(
                            handle.raw_handle,
                        ),
                    )?;
                    let fut = conn_py
                        .bind(py)
                        .call_method1("execute", (slave_addr, py_cmd.bind(py), true))?;
                    Ok(fut.unbind())
                })?;

                let res = Python::with_gil(|py| {
                    pyo3_async_runtimes::tokio::into_future(coro.bind(py).clone())
                })?
                .await?;
                let (lost, remaining, rx): (u32, u32, Vec<u8>) = Python::with_gil(|py| {
                    res.extract::<((u32, u32, Vec<u8>), bool)>(py).map(|r| r.0)
                })?;

                total_lost_bytes += lost;
                remaining_bytes = remaining;
                total_rx_data.extend(rx);
                if remaining == 0 {
                    break;
                }
            }

            Python::with_gil(|py| {
                let resp = PyReadBufferResponse::new(
                    py,
                    handle.rx_length,
                    total_lost_bytes,
                    remaining_bytes,
                    &total_rx_data,
                )?;
                Ok(resp)
            })
        })
    }

    pub fn set_spi_config<'py>(
        &self,
        py: Python<'py>,
        port: Bound<'py, PyAny>,
        mode: u8,
        frequency: f64,
    ) -> PyResult<Bound<'py, PyAny>> {
        let raw_port = port_to_byte(port, true)?;
        let _ = SensorBridgeSpiMode::from_u8(mode).map_err(|e| to_py_sensorbridge_err(py, e))?;
        let raw_freq = spi_frequency_to_byte(py, frequency)?;
        let py_cmd = Py::new(
            py,
            crate::python::commands::PySensorBridgeCmdSpiConfig::new(raw_port, mode, raw_freq),
        )?;
        self.execute(py, py_cmd.bind(py).clone().into_any())
    }

    pub fn transceive_spi<'py>(
        &self,
        py: Python<'py>,
        port: Bound<'py, PyAny>,
        tx_data: Vec<u8>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let raw_port = port_to_byte(port, false)?;
        let py_cmd = Py::new(
            py,
            crate::python::commands::PySensorBridgeCmdSpiTransceive::new(raw_port, &tx_data),
        )?;
        let conn_py = self.conn_obj.clone_ref(py);
        let slave_addr = self.slave_address;

        let coro = Python::with_gil(|py| -> PyResult<PyObject> {
            let fut = conn_py
                .bind(py)
                .call_method1("execute", (slave_addr, py_cmd.bind(py), true))?;
            Ok(fut.unbind())
        })?;

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let res = Python::with_gil(|py| {
                pyo3_async_runtimes::tokio::into_future(coro.bind(py).clone())
            })?
            .await?;

            Python::with_gil(|py| -> PyResult<PyObject> {
                let (rx_bytes, _): (Vec<u8>, bool) = res.extract(py)?;
                Ok(PyBytes::new(py, &rx_bytes).unbind().into())
            })
        })
    }
}
