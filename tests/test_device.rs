use byteorder::{ByteOrder, LittleEndian};
use rust_shdlc_driver::connection::{AsyncShdlcConnection, ShdlcConnection};
use rust_shdlc_driver::protocol::*;
use rust_shdlc_driver::transport::MockTransport;
use sensirion_uart_sensorbridge::device::{AsyncSensorBridgeDevice, SensorBridgeDevice};
use sensirion_uart_sensorbridge::protocol::definitions::*;

fn make_response_frame(slave_addr: u8, cmd_id: u8, state: u8, data: &[u8]) -> Vec<u8> {
    let mut content = vec![slave_addr, cmd_id, state, data.len() as u8];
    content.extend_from_slice(data);
    let checksum = calculate_checksum(&content);
    content.push(checksum);
    let stuffed = stuff_data_bytes(&content);
    let mut raw = vec![START_STOP_BYTE];
    raw.extend_from_slice(&stuffed);
    raw.push(START_STOP_BYTE);
    raw
}

#[tokio::test]
async fn test_async_sensorbridge_device() {
    let mock = MockTransport::new();
    let conn = AsyncShdlcConnection::new(Box::new(mock.clone()));
    let mut dev = AsyncSensorBridgeDevice::new(conn, 0x00);

    // 1. Blink
    mock.push_rx_data(&make_response_frame(0x00, 0x70, 0x00, &[]));
    dev.blink_led(SensorBridgePort::One).await.unwrap();

    // 2. Measure Voltage
    let mut float_bytes = [0u8; 4];
    LittleEndian::write_f32(&mut float_bytes, 3.31);
    mock.push_rx_data(&make_response_frame(0x00, 0x80, 0x00, &float_bytes));
    let v = dev.measure_voltage(SensorBridgePort::One).await.unwrap();
    assert!((v - 3.31).abs() < 1e-4);

    // 3. Set Supply Voltage
    mock.push_rx_data(&make_response_frame(0x00, 0x00, 0x00, &[]));
    dev.set_supply_voltage(SensorBridgePort::All, 3.3)
        .await
        .unwrap();

    // 4. Switch Supply On / Off
    mock.push_rx_data(&make_response_frame(0x00, 0x01, 0x00, &[]));
    dev.switch_supply_on(SensorBridgePort::One).await.unwrap();

    mock.push_rx_data(&make_response_frame(0x00, 0x01, 0x00, &[]));
    dev.switch_supply_off(SensorBridgePort::One).await.unwrap();

    // 5. Set I2C Frequency
    mock.push_rx_data(&make_response_frame(0x00, 0x02, 0x00, &[]));
    dev.set_i2c_frequency(SensorBridgePort::Two, 400_000)
        .await
        .unwrap();

    // 6. Scan I2C
    mock.push_rx_data(&make_response_frame(0x00, 0x10, 0x00, &[0x69, 0x70]));
    let addrs = dev.scan_i2c(SensorBridgePort::One, 1, 127).await.unwrap();
    assert_eq!(addrs, vec![0x69, 0x70]);

    // 7. Transceive I2C (single frame)
    mock.push_rx_data(&make_response_frame(0x00, 0x11, 0x00, &[0xBE, 0xEF, 0x92]));
    let rx = dev
        .transceive_i2c(SensorBridgePort::One, 0x70, &[0x01, 0x02], 3, 1000)
        .await
        .unwrap();
    assert_eq!(rx, vec![0xBE, 0xEF, 0x92]);

    // 8. Repeated I2C Transceive
    mock.push_rx_data(&make_response_frame(0x00, 0x12, 0x00, &[0x20, 0x21]));
    let handles = dev
        .start_repeated_i2c_transceive(SensorBridgePort::All, 10000, 0x70, &[0xAA], 6, 1000, 500)
        .await
        .unwrap();
    assert_eq!(handles.len(), 2);
    assert_eq!(handles[0].raw_handle, 0x20);
    assert_eq!(handles[1].raw_handle, 0x21);

    // 9. Read Buffer
    let mut buf_resp = vec![0, 0, 0, 0, 0, 0, 0, 0]; // 0 lost, 0 remaining
    buf_resp.extend_from_slice(&[0x00, 1, 2, 3, 4, 5, 6]); // 1 status byte + 6 bytes data
    mock.push_rx_data(&make_response_frame(0x00, 0x50, 0x00, &buf_resp));
    let buf_data = dev.read_buffer(handles[0], 10).await.unwrap();
    assert_eq!(buf_data.lost_bytes, 0);
    assert_eq!(buf_data.remaining_bytes, 0);
    assert_eq!(buf_data.values.len(), 1);
    assert_eq!(buf_data.values[0].data().unwrap(), &[1, 2, 3, 4, 5, 6]);

    // 10. Stop Repeated Transceive
    mock.push_rx_data(&make_response_frame(0x00, 0x51, 0x00, &[]));
    dev.stop_repeated_i2c_transceive(Some(handles[0]))
        .await
        .unwrap();

    // 11. SPI Config & Transceive
    mock.push_rx_data(&make_response_frame(0x00, 0x60, 0x00, &[]));
    dev.set_spi_config(SensorBridgePort::One, SensorBridgeSpiMode::Mode0, 1_000_000)
        .await
        .unwrap();

    mock.push_rx_data(&make_response_frame(0x00, 0x61, 0x00, &[0xDE, 0xAD]));
    let spi_rx = dev
        .transceive_spi(SensorBridgePort::One, &[0x01, 0x02])
        .await
        .unwrap();
    assert_eq!(spi_rx, vec![0xDE, 0xAD]);
}

#[test]
fn test_sync_sensorbridge_device() {
    let mock = MockTransport::new();
    let conn = ShdlcConnection::new(Box::new(mock.clone())).unwrap();
    let mut dev = SensorBridgeDevice::new(conn, 0x00);

    // Measure Voltage
    let mut float_bytes = [0u8; 4];
    LittleEndian::write_f32(&mut float_bytes, 4.95);
    mock.push_rx_data(&make_response_frame(0x00, 0x80, 0x00, &float_bytes));
    let v = dev.measure_voltage(SensorBridgePort::Two).unwrap();
    assert!((v - 4.95).abs() < 1e-4);

    // Set Supply Voltage
    mock.push_rx_data(&make_response_frame(0x00, 0x00, 0x00, &[]));
    dev.set_supply_voltage(SensorBridgePort::One, 5.0).unwrap();

    // Scan I2C
    mock.push_rx_data(&make_response_frame(0x00, 0x10, 0x00, &[0x38]));
    let addrs = dev.scan_i2c(SensorBridgePort::One, 1, 127).unwrap();
    assert_eq!(addrs, vec![0x38]);
}
