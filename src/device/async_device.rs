use crate::firmware::SensorBridgeFirmwareImage;
use crate::protocol::commands::*;
use crate::protocol::definitions::*;
use crate::protocol::errors::{sensorbridge_device_error_map, SensorBridgeError};
use crate::protocol::types::*;
use rust_shdlc_driver::connection::AsyncShdlcConnection;
use rust_shdlc_driver::device::AsyncShdlcDeviceBase;
use rust_shdlc_driver::firmware::ShdlcFirmwareUpdate;
use rust_shdlc_driver::protocol::commands::*;
use rust_shdlc_driver::protocol::types::Version;

/// Asynchronous SEK-SensorBridge device driver.
#[derive(Clone)]
pub struct AsyncSensorBridgeDevice {
    pub base: AsyncShdlcDeviceBase,
}

impl AsyncSensorBridgeDevice {
    /// Creates a new AsyncSensorBridgeDevice instance on an SHDLC connection.
    pub fn new(connection: AsyncShdlcConnection, slave_address: u8) -> Self {
        let mut base = AsyncShdlcDeviceBase::new(connection, slave_address);
        for (code, msg) in sensorbridge_device_error_map() {
            base.register_device_error(code, msg);
        }
        Self { base }
    }

    pub fn slave_address(&self) -> u8 {
        self.base.slave_address
    }

    pub fn last_error_flag(&self) -> bool {
        self.base.last_error_flag
    }

    pub fn connection(&self) -> &AsyncShdlcConnection {
        &self.base.connection
    }

    // --- Standard SHDLC Device Commands ---

    pub async fn get_product_type(&mut self) -> Result<String, SensorBridgeError> {
        Ok(self.base.execute(&GetProductType).await?)
    }

    pub async fn get_product_subtype(&mut self) -> Result<u8, SensorBridgeError> {
        Ok(self.base.execute(&GetProductSubType).await?)
    }

    pub async fn get_product_name(&mut self) -> Result<String, SensorBridgeError> {
        Ok(self.base.execute(&GetProductName).await?)
    }

    pub async fn get_article_code(&mut self) -> Result<String, SensorBridgeError> {
        Ok(self.base.execute(&GetArticleCode).await?)
    }

    pub async fn get_serial_number(&mut self) -> Result<String, SensorBridgeError> {
        Ok(self.base.execute(&GetSerialNumber).await?)
    }

    pub async fn get_version(&mut self) -> Result<Version, SensorBridgeError> {
        Ok(self.base.execute(&GetVersion).await?)
    }

    pub async fn get_error_state(&mut self, clear: bool) -> Result<(u32, u8), SensorBridgeError> {
        Ok(self.base.execute(&GetErrorState::new(clear)).await?)
    }

    pub async fn get_slave_address(&mut self) -> Result<u8, SensorBridgeError> {
        Ok(self.base.execute(&GetSlaveAddress).await?)
    }

    pub async fn set_slave_address(
        &mut self,
        slave_address: u8,
        update_driver: bool,
    ) -> Result<(), SensorBridgeError> {
        self.base
            .execute(&SetSlaveAddress::new(slave_address))
            .await?;
        if update_driver {
            self.base.slave_address = slave_address;
        }
        Ok(())
    }

    pub async fn get_baudrate(&mut self) -> Result<u32, SensorBridgeError> {
        Ok(self.base.execute(&GetBaudrate).await?)
    }

    pub async fn set_baudrate(
        &mut self,
        baudrate: u32,
        update_driver: bool,
    ) -> Result<(), SensorBridgeError> {
        self.base.execute(&SetBaudrate::new(baudrate)).await?;
        if update_driver {
            let transport_arc = self.base.connection.transport();
            let mut transport = transport_arc.lock().await;
            let _ = transport.set_bitrate(baudrate).await;
        }
        Ok(())
    }

    pub async fn get_reply_delay(&mut self) -> Result<u16, SensorBridgeError> {
        Ok(self.base.execute(&GetReplyDelay).await?)
    }

    pub async fn set_reply_delay(&mut self, reply_delay_us: u16) -> Result<(), SensorBridgeError> {
        Ok(self
            .base
            .execute(&SetReplyDelay::new(reply_delay_us))
            .await?)
    }

    pub async fn get_system_up_time(&mut self) -> Result<u32, SensorBridgeError> {
        Ok(self.base.execute(&GetSystemUpTime).await?)
    }

    pub async fn device_reset(&mut self) -> Result<(), SensorBridgeError> {
        Ok(self.base.execute(&DeviceReset).await?)
    }

    pub async fn factory_reset(&mut self) -> Result<(), SensorBridgeError> {
        Ok(self.base.execute(&FactoryReset).await?)
    }

    // --- SensorBridge Specific Commands ---

    /// Let the LEDs on the device blink.
    pub async fn blink_led(&mut self, port: SensorBridgePort) -> Result<(), SensorBridgeError> {
        let raw_port = port.to_byte(true)?;
        self.base.execute(&Blink::new(raw_port)).await?;
        Ok(())
    }

    /// Triggers an analog measurement at the AIN pin. The measured voltage is always positive (0-5.5V).
    pub async fn measure_voltage(
        &mut self,
        port: SensorBridgePort,
    ) -> Result<f32, SensorBridgeError> {
        let raw_port = port.to_byte(false)?;
        let val = self.base.execute(&AnalogMeasurement::new(raw_port)).await?;
        Ok(val)
    }

    /// Sets the supply voltage of a port to a specified value (in Volts).
    pub async fn set_supply_voltage(
        &mut self,
        port: SensorBridgePort,
        voltage_volts: f64,
    ) -> Result<(), SensorBridgeError> {
        let raw_port = port.to_byte(true)?;
        let raw_voltage = voltage_to_byte(voltage_volts)?;
        self.base
            .execute(&SetPortVoltage::new(raw_port, raw_voltage))
            .await?;
        Ok(())
    }

    /// Switches a port supply on. The previously configured voltage is applied.
    pub async fn switch_supply_on(
        &mut self,
        port: SensorBridgePort,
    ) -> Result<(), SensorBridgeError> {
        let raw_port = port.to_byte(true)?;
        self.base
            .execute(&PortVoltageOnOff::new(raw_port, 1))
            .await?;
        Ok(())
    }

    /// Switches a port supply off.
    pub async fn switch_supply_off(
        &mut self,
        port: SensorBridgePort,
    ) -> Result<(), SensorBridgeError> {
        let raw_port = port.to_byte(true)?;
        self.base
            .execute(&PortVoltageOnOff::new(raw_port, 0))
            .await?;
        Ok(())
    }

    /// Sets the I2C frequency of a port (in Hz).
    pub async fn set_i2c_frequency(
        &mut self,
        port: SensorBridgePort,
        frequency_hz: u32,
    ) -> Result<(), SensorBridgeError> {
        let raw_port = port.to_byte(true)?;
        let raw_freq = i2c_frequency_to_byte(frequency_hz)?;
        self.base
            .execute(&SetI2cFrequency::new(raw_port, raw_freq))
            .await?;
        Ok(())
    }

    /// Scans for I2C devices on a specific port within a certain address range (1..127).
    pub async fn scan_i2c(
        &mut self,
        port: SensorBridgePort,
        first_address: u8,
        last_address: u8,
    ) -> Result<Vec<u8>, SensorBridgeError> {
        let raw_port = port.to_byte(false)?;
        let addrs = self
            .base
            .execute(&I2cScan::new(raw_port, first_address, last_address))
            .await?;
        Ok(addrs)
    }

    /// Transceives an I2C frame on a specific port.
    /// Handles splitting the TX and RX data across multiple SHDLC frames if needed.
    pub async fn transceive_i2c(
        &mut self,
        port: SensorBridgePort,
        address: u8,
        tx_data: &[u8],
        rx_length: usize,
        timeout_us: u32,
    ) -> Result<Vec<u8>, SensorBridgeError> {
        let raw_port = port.to_byte(false)?;
        let max_tx_len_per_frame = 255 - 15; // 240 bytes

        let tx_len_sent = std::cmp::min(tx_data.len(), max_tx_len_per_frame);
        let first_chunk = &tx_data[0..tx_len_sent];

        let mut rx_data = self
            .base
            .execute(&FirstTransceive::new(
                raw_port,
                address,
                tx_data.len() as u32,
                rx_length as u32,
                timeout_us,
                first_chunk,
            ))
            .await?;

        // Send remaining TX data
        let mut offset = tx_len_sent;
        while offset < tx_data.len() {
            let chunk_len = std::cmp::min(tx_data.len() - offset, max_tx_len_per_frame);
            let sub_rx = self
                .base
                .execute(&SubsequentTransceive::new(
                    raw_port,
                    &tx_data[offset..offset + chunk_len],
                ))
                .await?;
            rx_data.extend(sub_rx);
            offset += chunk_len;
        }

        // Receive remaining RX data
        while rx_data.len() < rx_length {
            let sub_rx = self
                .base
                .execute(&SubsequentTransceive::new(raw_port, &[]))
                .await?;
            if sub_rx.is_empty() {
                break;
            }
            rx_data.extend(sub_rx);
        }

        Ok(rx_data)
    }

    /// Starts an asynchronous repeated I2C transceive operation.
    #[allow(clippy::too_many_arguments)]
    pub async fn start_repeated_i2c_transceive(
        &mut self,
        port: SensorBridgePort,
        interval_us: u32,
        address: u8,
        tx_data: &[u8],
        rx_length: usize,
        timeout_us: u32,
        read_delay_us: u32,
    ) -> Result<Vec<RepeatedTransceiveHandle>, SensorBridgeError> {
        let raw_port = port.to_byte(true)?;
        let raw_handles = self
            .base
            .execute(&I2cRepeatedTransceive::new(
                interval_us,
                raw_port,
                address,
                tx_data.len() as u32,
                rx_length as u32,
                timeout_us,
                read_delay_us,
                tx_data,
            ))
            .await?;

        let handles = raw_handles
            .into_iter()
            .map(|h| RepeatedTransceiveHandle::new(h, rx_length))
            .collect();
        Ok(handles)
    }

    /// Stops a repeated transceive operation (or all if handle is None).
    pub async fn stop_repeated_i2c_transceive(
        &mut self,
        handle: Option<RepeatedTransceiveHandle>,
    ) -> Result<(), SensorBridgeError> {
        let raw_handle = handle.map(|h| h.raw_handle).unwrap_or(0xFF);
        self.base
            .execute(&StopRepeatedTransceive::new(raw_handle))
            .await?;
        Ok(())
    }

    /// Reads data stored in the buffer of a repeated transceive operation.
    pub async fn read_buffer(
        &mut self,
        handle: RepeatedTransceiveHandle,
        max_reads: usize,
    ) -> Result<ReadBufferResponse, SensorBridgeError> {
        let mut total_lost_bytes = 0;
        let mut remaining_bytes = 0;
        let mut total_rx_data = Vec::new();

        for _ in 0..max_reads {
            let (lost, remaining, rx) = self
                .base
                .execute(&ReadBuffer::new(handle.raw_handle))
                .await?;
            total_lost_bytes += lost;
            remaining_bytes = remaining;
            total_rx_data.extend(rx);
            if remaining == 0 {
                break;
            }
        }

        ReadBufferResponse::from_raw(
            handle.rx_length,
            total_lost_bytes,
            remaining_bytes,
            &total_rx_data,
        )
    }

    /// Sets SPI mode and frequency.
    pub async fn set_spi_config(
        &mut self,
        port: SensorBridgePort,
        mode: SensorBridgeSpiMode,
        frequency_hz: u32,
    ) -> Result<(), SensorBridgeError> {
        let raw_port = port.to_byte(true)?;
        let raw_freq = spi_frequency_to_byte(frequency_hz)?;
        self.base
            .execute(&SpiConfig::new(raw_port, mode.to_u8(), raw_freq))
            .await?;
        Ok(())
    }

    /// Transceives an SPI frame on a specific port.
    pub async fn transceive_spi(
        &mut self,
        port: SensorBridgePort,
        tx_data: &[u8],
    ) -> Result<Vec<u8>, SensorBridgeError> {
        let raw_port = port.to_byte(false)?;
        let rx = self
            .base
            .execute(&SpiTransceive::new(raw_port, tx_data))
            .await?;
        Ok(rx)
    }

    /// Updates the firmware of the SEK-SensorBridge device.
    pub async fn update_firmware<F, P>(
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
        let shdlc_dev = rust_shdlc_driver::device::AsyncShdlcDevice {
            base: self.base.clone(),
        };
        let mut updater = ShdlcFirmwareUpdate::new(shdlc_dev, image.inner.clone());
        if let Some(scb) = status_cb {
            updater.set_status_callback(scb);
        }
        if let Some(pcb) = progress_cb {
            updater.set_progress_callback(pcb);
        }
        updater.execute(emergency).await?;
        Ok(())
    }
}
