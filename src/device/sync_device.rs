use crate::device::AsyncSensorBridgeDevice;
use crate::firmware::SensorBridgeFirmwareImage;
use crate::protocol::definitions::*;
use crate::protocol::errors::SensorBridgeError;
use crate::protocol::types::*;
use rust_shdlc_driver::connection::ShdlcConnection;
use rust_shdlc_driver::protocol::types::Version;
use rust_shdlc_driver::transport::runtime::get_runtime;

/// Synchronous blocking SEK-SensorBridge device driver.
#[derive(Clone)]
pub struct SensorBridgeDevice {
    async_dev: AsyncSensorBridgeDevice,
    conn: ShdlcConnection,
}

impl SensorBridgeDevice {
    /// Creates a new synchronous SensorBridgeDevice instance on an SHDLC connection.
    pub fn new(conn: ShdlcConnection, slave_address: u8) -> Self {
        let async_dev = AsyncSensorBridgeDevice::new(conn.async_conn().clone(), slave_address);
        Self { async_dev, conn }
    }

    pub fn slave_address(&self) -> u8 {
        self.async_dev.slave_address()
    }

    pub fn last_error_flag(&self) -> bool {
        self.async_dev.last_error_flag()
    }

    pub fn connection(&self) -> &ShdlcConnection {
        &self.conn
    }

    // --- Standard SHDLC Device Commands ---

    pub fn get_product_type(&mut self) -> Result<String, SensorBridgeError> {
        let mut dev = self.async_dev.clone();
        get_runtime().block_on(dev.get_product_type())
    }

    pub fn get_product_subtype(&mut self) -> Result<u8, SensorBridgeError> {
        let mut dev = self.async_dev.clone();
        get_runtime().block_on(dev.get_product_subtype())
    }

    pub fn get_product_name(&mut self) -> Result<String, SensorBridgeError> {
        let mut dev = self.async_dev.clone();
        get_runtime().block_on(dev.get_product_name())
    }

    pub fn get_article_code(&mut self) -> Result<String, SensorBridgeError> {
        let mut dev = self.async_dev.clone();
        get_runtime().block_on(dev.get_article_code())
    }

    pub fn get_serial_number(&mut self) -> Result<String, SensorBridgeError> {
        let mut dev = self.async_dev.clone();
        get_runtime().block_on(dev.get_serial_number())
    }

    pub fn get_version(&mut self) -> Result<Version, SensorBridgeError> {
        let mut dev = self.async_dev.clone();
        get_runtime().block_on(dev.get_version())
    }

    pub fn get_error_state(&mut self, clear: bool) -> Result<(u32, u8), SensorBridgeError> {
        let mut dev = self.async_dev.clone();
        get_runtime().block_on(dev.get_error_state(clear))
    }

    pub fn get_slave_address(&mut self) -> Result<u8, SensorBridgeError> {
        let mut dev = self.async_dev.clone();
        get_runtime().block_on(dev.get_slave_address())
    }

    pub fn set_slave_address(
        &mut self,
        slave_address: u8,
        update_driver: bool,
    ) -> Result<(), SensorBridgeError> {
        let mut dev = self.async_dev.clone();
        get_runtime().block_on(dev.set_slave_address(slave_address, update_driver))?;
        if update_driver {
            self.async_dev.base.slave_address = slave_address;
        }
        Ok(())
    }

    pub fn get_baudrate(&mut self) -> Result<u32, SensorBridgeError> {
        let mut dev = self.async_dev.clone();
        get_runtime().block_on(dev.get_baudrate())
    }

    pub fn set_baudrate(
        &mut self,
        baudrate: u32,
        update_driver: bool,
    ) -> Result<(), SensorBridgeError> {
        let mut dev = self.async_dev.clone();
        get_runtime().block_on(dev.set_baudrate(baudrate, update_driver))
    }

    pub fn get_reply_delay(&mut self) -> Result<u16, SensorBridgeError> {
        let mut dev = self.async_dev.clone();
        get_runtime().block_on(dev.get_reply_delay())
    }

    pub fn set_reply_delay(&mut self, reply_delay_us: u16) -> Result<(), SensorBridgeError> {
        let mut dev = self.async_dev.clone();
        get_runtime().block_on(dev.set_reply_delay(reply_delay_us))
    }

    pub fn get_system_up_time(&mut self) -> Result<u32, SensorBridgeError> {
        let mut dev = self.async_dev.clone();
        get_runtime().block_on(dev.get_system_up_time())
    }

    pub fn device_reset(&mut self) -> Result<(), SensorBridgeError> {
        let mut dev = self.async_dev.clone();
        get_runtime().block_on(dev.device_reset())
    }

    pub fn factory_reset(&mut self) -> Result<(), SensorBridgeError> {
        let mut dev = self.async_dev.clone();
        get_runtime().block_on(dev.factory_reset())
    }

    // --- SensorBridge Specific Commands ---

    pub fn blink_led(&mut self, port: SensorBridgePort) -> Result<(), SensorBridgeError> {
        let mut dev = self.async_dev.clone();
        get_runtime().block_on(dev.blink_led(port))
    }

    pub fn measure_voltage(&mut self, port: SensorBridgePort) -> Result<f32, SensorBridgeError> {
        let mut dev = self.async_dev.clone();
        get_runtime().block_on(dev.measure_voltage(port))
    }

    pub fn set_supply_voltage(
        &mut self,
        port: SensorBridgePort,
        voltage_volts: f64,
    ) -> Result<(), SensorBridgeError> {
        let mut dev = self.async_dev.clone();
        get_runtime().block_on(dev.set_supply_voltage(port, voltage_volts))
    }

    pub fn switch_supply_on(&mut self, port: SensorBridgePort) -> Result<(), SensorBridgeError> {
        let mut dev = self.async_dev.clone();
        get_runtime().block_on(dev.switch_supply_on(port))
    }

    pub fn switch_supply_off(&mut self, port: SensorBridgePort) -> Result<(), SensorBridgeError> {
        let mut dev = self.async_dev.clone();
        get_runtime().block_on(dev.switch_supply_off(port))
    }

    pub fn set_i2c_frequency(
        &mut self,
        port: SensorBridgePort,
        frequency_hz: u32,
    ) -> Result<(), SensorBridgeError> {
        let mut dev = self.async_dev.clone();
        get_runtime().block_on(dev.set_i2c_frequency(port, frequency_hz))
    }

    pub fn scan_i2c(
        &mut self,
        port: SensorBridgePort,
        first_address: u8,
        last_address: u8,
    ) -> Result<Vec<u8>, SensorBridgeError> {
        let mut dev = self.async_dev.clone();
        get_runtime().block_on(dev.scan_i2c(port, first_address, last_address))
    }

    pub fn transceive_i2c(
        &mut self,
        port: SensorBridgePort,
        address: u8,
        tx_data: &[u8],
        rx_length: usize,
        timeout_us: u32,
    ) -> Result<Vec<u8>, SensorBridgeError> {
        let mut dev = self.async_dev.clone();
        get_runtime().block_on(dev.transceive_i2c(port, address, tx_data, rx_length, timeout_us))
    }

    #[allow(clippy::too_many_arguments)]
    pub fn start_repeated_i2c_transceive(
        &mut self,
        port: SensorBridgePort,
        interval_us: u32,
        address: u8,
        tx_data: &[u8],
        rx_length: usize,
        timeout_us: u32,
        read_delay_us: u32,
    ) -> Result<Vec<RepeatedTransceiveHandle>, SensorBridgeError> {
        let mut dev = self.async_dev.clone();
        get_runtime().block_on(dev.start_repeated_i2c_transceive(
            port,
            interval_us,
            address,
            tx_data,
            rx_length,
            timeout_us,
            read_delay_us,
        ))
    }

    pub fn stop_repeated_i2c_transceive(
        &mut self,
        handle: Option<RepeatedTransceiveHandle>,
    ) -> Result<(), SensorBridgeError> {
        let mut dev = self.async_dev.clone();
        get_runtime().block_on(dev.stop_repeated_i2c_transceive(handle))
    }

    pub fn read_buffer(
        &mut self,
        handle: RepeatedTransceiveHandle,
        max_reads: usize,
    ) -> Result<ReadBufferResponse, SensorBridgeError> {
        let mut dev = self.async_dev.clone();
        get_runtime().block_on(dev.read_buffer(handle, max_reads))
    }

    pub fn set_spi_config(
        &mut self,
        port: SensorBridgePort,
        mode: SensorBridgeSpiMode,
        frequency_hz: u32,
    ) -> Result<(), SensorBridgeError> {
        let mut dev = self.async_dev.clone();
        get_runtime().block_on(dev.set_spi_config(port, mode, frequency_hz))
    }

    pub fn transceive_spi(
        &mut self,
        port: SensorBridgePort,
        tx_data: &[u8],
    ) -> Result<Vec<u8>, SensorBridgeError> {
        let mut dev = self.async_dev.clone();
        get_runtime().block_on(dev.transceive_spi(port, tx_data))
    }

    pub fn update_firmware<F, P>(
        &mut self,
        image: &SensorBridgeFirmwareImage,
        emergency: bool,
        status_cb: Option<F>,
        progress_cb: Option<P>,
    ) -> Result<(), SensorBridgeError>
    where
        F: Fn(&str) + Send + Sync + 'static,
        P: Fn(f64) + Send + Sync + 'static,
    {
        let mut dev = self.async_dev.clone();
        get_runtime().block_on(dev.update_firmware(image, emergency, status_cb, progress_cb))
    }
}
