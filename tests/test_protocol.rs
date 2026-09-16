use byteorder::{ByteOrder, LittleEndian};
use rust_shdlc_driver::protocol::commands::ShdlcCommand;
use sensirion_uart_sensorbridge::protocol::commands::*;
use sensirion_uart_sensorbridge::protocol::definitions::*;
use sensirion_uart_sensorbridge::protocol::errors::SensorBridgeError;
use sensirion_uart_sensorbridge::protocol::types::*;

#[test]
fn test_port_conversion() {
    assert_eq!(SensorBridgePort::One.to_byte(false).unwrap(), 0x00);
    assert_eq!(SensorBridgePort::Two.to_byte(false).unwrap(), 0x01);
    assert!(SensorBridgePort::All.to_byte(false).is_err());
    assert_eq!(SensorBridgePort::All.to_byte(true).unwrap(), 0xFF);

    assert_eq!(
        SensorBridgePort::from_byte(0x00, false).unwrap(),
        SensorBridgePort::One
    );
    assert_eq!(
        SensorBridgePort::from_byte(0x01, false).unwrap(),
        SensorBridgePort::Two
    );
    assert!(SensorBridgePort::from_byte(0xFF, false).is_err());
    assert_eq!(
        SensorBridgePort::from_byte(0xFF, true).unwrap(),
        SensorBridgePort::All
    );
    assert!(SensorBridgePort::from_byte(0x02, true).is_err());
}

#[test]
fn test_voltage_conversions() {
    for &v in &SUPPORTED_VOLTAGES {
        let b = voltage_to_byte(v).unwrap();
        let decoded = byte_to_voltage(b).unwrap();
        assert!((v - decoded).abs() < 1e-5);
    }
    assert!(voltage_to_byte(1.3).is_err());
    assert!(byte_to_voltage(0x10).is_err());
}

#[test]
fn test_i2c_frequency_conversions() {
    for &freq in &SUPPORTED_I2C_FREQUENCIES {
        let b = i2c_frequency_to_byte(freq).unwrap();
        let decoded = byte_to_i2c_frequency(b).unwrap();
        assert_eq!(freq, decoded);
    }
    assert!(i2c_frequency_to_byte(200_000).is_err());
    assert!(byte_to_i2c_frequency(0x09).is_err());
}

#[test]
fn test_spi_frequency_conversions() {
    for &freq in &SUPPORTED_SPI_FREQUENCIES {
        let b = spi_frequency_to_byte(freq).unwrap();
        let decoded = byte_to_spi_frequency(b).unwrap();
        assert_eq!(freq, decoded);
    }
    assert!(spi_frequency_to_byte(500_000).is_err());
    assert!(byte_to_spi_frequency(0x08).is_err());
}

#[test]
fn test_spi_mode() {
    assert_eq!(
        SensorBridgeSpiMode::from_u8(0).unwrap(),
        SensorBridgeSpiMode::Mode0
    );
    assert_eq!(
        SensorBridgeSpiMode::from_u8(1).unwrap(),
        SensorBridgeSpiMode::Mode1
    );
    assert_eq!(
        SensorBridgeSpiMode::from_u8(2).unwrap(),
        SensorBridgeSpiMode::Mode2
    );
    assert_eq!(
        SensorBridgeSpiMode::from_u8(3).unwrap(),
        SensorBridgeSpiMode::Mode3
    );
    assert!(SensorBridgeSpiMode::from_u8(4).is_err());
}

#[test]
fn test_command_serialization() {
    let cmd = SetPortVoltage::new(0x00, 0x02);
    assert_eq!(cmd.id(), 0x00);
    assert_eq!(cmd.data(), vec![0x00, 0x02]);

    let cmd = PortVoltageOnOff::new(0xFF, 0x01);
    assert_eq!(cmd.id(), 0x01);
    assert_eq!(cmd.data(), vec![0xFF, 0x01]);

    let cmd = SetI2cFrequency::new(0x01, 0x01);
    assert_eq!(cmd.id(), 0x02);
    assert_eq!(cmd.data(), vec![0x01, 0x01]);

    let cmd = I2cScan::new(0x00, 0x01, 0x7F);
    assert_eq!(cmd.id(), 0x10);
    assert_eq!(cmd.data(), vec![0x00, 0x01, 0x7F]);
    assert_eq!(
        cmd.interpret_response(&[0x69, 0x70]).unwrap(),
        vec![0x69, 0x70]
    );

    let cmd = FirstTransceive::new(0x00, 0x69, 2, 6, 1000, &[0x20, 0x32]);
    assert_eq!(cmd.id(), 0x11);
    assert_eq!(cmd.data()[0], 0x00); // Subcommand 0x00
    assert_eq!(cmd.data()[1], 0x00); // Port
    assert_eq!(cmd.data()[2], 0x69); // Addr

    let cmd = SubsequentTransceive::new(0x01, &[0x11, 0x22]);
    assert_eq!(cmd.id(), 0x11);
    assert_eq!(cmd.data()[0], 0x01); // Subcommand 0x01
    assert_eq!(cmd.data()[1], 0x01); // Port

    let cmd = I2cRepeatedTransceive::new(10000, 0x00, 0x69, 2, 6, 1000, 500, &[0x20, 0x32]);
    assert_eq!(cmd.id(), 0x12);
    assert_eq!(cmd.interpret_response(&[0x10]).unwrap(), vec![0x10]);

    let cmd = ReadBuffer::new(0x10);
    assert_eq!(cmd.id(), 0x50);
    let resp_data = vec![0, 0, 0, 5, 0, 0, 0, 0, 0x00, 0xAA, 0xBB];
    let (lost, remaining, data) = cmd.interpret_response(&resp_data).unwrap();
    assert_eq!(lost, 5);
    assert_eq!(remaining, 0);
    assert_eq!(data, vec![0x00, 0xAA, 0xBB]);

    let cmd = StopRepeatedTransceive::new(0xFF);
    assert_eq!(cmd.id(), 0x51);
    assert_eq!(cmd.data(), vec![0xFF]);

    let cmd = SpiConfig::new(0x00, 0x00, 0x02);
    assert_eq!(cmd.id(), 0x60);
    assert_eq!(cmd.data(), vec![0x00, 0x00, 0x02]);

    let cmd = SpiTransceive::new(0x00, &[0x01, 0x02, 0x03]);
    assert_eq!(cmd.id(), 0x61);

    let cmd = Blink::new(0xFF);
    assert_eq!(cmd.id(), 0x70);
    assert_eq!(cmd.data(), vec![0xFF]);

    let cmd = AnalogMeasurement::new(0x00);
    assert_eq!(cmd.id(), 0x80);
    let mut float_buf = [0u8; 4];
    LittleEndian::write_f32(&mut float_buf, 3.295);
    let val = cmd.interpret_response(&float_buf).unwrap();
    assert!((val - 3.295).abs() < 1e-4);
}

#[test]
fn test_types_and_buffer_parsing() {
    let handle = RepeatedTransceiveHandle::new(0x31, 2);
    assert_eq!(handle.slot(), 3);
    assert_eq!(handle.channel(), 1);
    assert_eq!(handle.port(), SensorBridgePort::Two);

    let raw_packet_ok = [0x00, 0x12, 0x34];
    let bval = BufferedValue::new(&raw_packet_ok).unwrap();
    assert!(bval.is_ok());
    assert_eq!(bval.data().unwrap(), &[0x12, 0x34]);

    let raw_packet_nack = [0x01, 0x00, 0x00];
    let bval_err = BufferedValue::new(&raw_packet_nack).unwrap();
    assert!(!bval_err.is_ok());
    match bval_err.data().unwrap_err() {
        SensorBridgeError::I2cNack => {}
        _ => panic!("Expected I2cNack"),
    }

    // ReadBufferResponse
    let raw_bytes = vec![0x00, 0x11, 0x22, 0x00, 0x33, 0x44]; // 2 packets of length 3 (1 status + 2 rx)
    let resp = ReadBufferResponse::from_raw(2, 0, 0, &raw_bytes).unwrap();
    assert_eq!(resp.values.len(), 2);
    assert_eq!(resp.values[0].data().unwrap(), &[0x11, 0x22]);
    assert_eq!(resp.values[1].data().unwrap(), &[0x33, 0x44]);

    // Invalid length (not multiple of 3)
    let invalid_raw = vec![0x00, 0x11];
    assert!(ReadBufferResponse::from_raw(2, 0, 0, &invalid_raw).is_err());
}
