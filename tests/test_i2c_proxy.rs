use rust_shdlc_driver::connection::{AsyncShdlcConnection, ShdlcConnection};
use rust_shdlc_driver::protocol::*;
use rust_shdlc_driver::transport::MockTransport;
use sensirion_uart_sensorbridge::device::{
    AsyncSensorBridgeDevice, AsyncSensorBridgeI2cProxy, SensorBridgeDevice, SensorBridgeI2cProxy,
    I2C_STATUS_NACK, I2C_STATUS_OK,
};
use sensirion_uart_sensorbridge::protocol::definitions::SensorBridgePort;

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
async fn test_async_i2c_proxy() {
    let mock = MockTransport::new();
    let conn = AsyncShdlcConnection::new(Box::new(mock.clone()));
    let dev = AsyncSensorBridgeDevice::new(conn, 0x00);
    let mut proxy = AsyncSensorBridgeI2cProxy::new(dev, SensorBridgePort::One).unwrap();

    assert_eq!(proxy.description(), "SensorBridge");
    assert_eq!(proxy.channel_count(), None);
    assert_eq!(proxy.port(), SensorBridgePort::One);

    // Successful transceive
    mock.push_rx_data(&make_response_frame(0x00, 0x11, 0x00, &[0x12, 0x34]));
    let res = proxy.transceive(0x70, &[0xAA], 2, 0.001, 0.05).await;
    assert_eq!(res.status, I2C_STATUS_OK);
    assert_eq!(res.rx_data, vec![0x12, 0x34]);
    assert!(res.error.is_none());

    // NACK device error (error code 0x29)
    mock.push_rx_data(&make_response_frame(0x00, 0x11, 0x29, &[]));
    let res = proxy.transceive(0x70, &[0xAA], 2, 0.001, 0.05).await;
    assert_eq!(res.status, I2C_STATUS_NACK);
    assert!(res.rx_data.is_empty());
}

#[test]
fn test_sync_i2c_proxy() {
    let mock = MockTransport::new();
    let conn = ShdlcConnection::new(Box::new(mock.clone())).unwrap();
    let dev = SensorBridgeDevice::new(conn, 0x00);
    let mut proxy = SensorBridgeI2cProxy::new(dev, SensorBridgePort::Two).unwrap();

    mock.push_rx_data(&make_response_frame(0x00, 0x11, 0x00, &[0x55, 0x66]));
    let res = proxy.transceive(0x38, &[], 2, 0.0, 0.05);
    assert_eq!(res.status, I2C_STATUS_OK);
    assert_eq!(res.rx_data, vec![0x55, 0x66]);
}
