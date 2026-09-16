use rust_shdlc_driver::connection::{AsyncShdlcConnection, ShdlcConnection};
use rust_shdlc_driver::transport::serial::AsyncSerialPort;
use sensirion_uart_sensorbridge::device::{
    AsyncSensorBridgeDevice, AsyncSensorBridgeI2cProxy, SensorBridgeDevice, SensorBridgeI2cProxy,
};
use sensirion_uart_sensorbridge::protocol::definitions::{SensorBridgePort, SensorBridgeSpiMode};
use sensirion_uart_sensorbridge::protocol::errors::SensorBridgeError;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n========================================================");
    println!("Testing Rust Driver against Hardware on /dev/ttyUSB0");
    println!("========================================================\n");

    // -----------------------------------------------------------------------
    // 1. Synchronous Rust Driver
    // -----------------------------------------------------------------------
    {
        println!("--- 1. Testing Synchronous Rust Driver ---");
        let mut port = AsyncSerialPort::new("/dev/ttyUSB0", 460_800);
        port.open()?;
        let conn = ShdlcConnection::new(Box::new(port))?;
        let mut dev = SensorBridgeDevice::new(conn, 0x00);

        test_sync_command("get_product_name", || dev.get_product_name());
        test_sync_command("get_product_type", || dev.get_product_type());
        test_sync_command("get_article_code", || dev.get_article_code());
        test_sync_command("get_serial_number", || dev.get_serial_number());
        test_sync_command("get_product_subtype", || {
            dev.get_product_subtype().map(|v| v.to_string())
        });
        test_sync_command("get_version", || {
            dev.get_version().map(|v| {
                format!(
                    "FW {}.{}, HW {}.{}, Proto {}.{}",
                    v.firmware.major,
                    v.firmware.minor,
                    v.hardware.major,
                    v.hardware.minor,
                    v.protocol.major,
                    v.protocol.minor
                )
            })
        });
        test_sync_command("get_error_state", || {
            dev.get_error_state(true)
                .map(|(s, e)| format!("state={}, err={}", s, e))
        });
        test_sync_command("get_slave_address", || {
            dev.get_slave_address().map(|a| a.to_string())
        });
        test_sync_command("get_baudrate", || dev.get_baudrate().map(|b| b.to_string()));
        test_sync_command("get_reply_delay", || {
            dev.get_reply_delay().map(|d| d.to_string())
        });
        test_sync_command("get_system_up_time", || {
            dev.get_system_up_time().map(|u| format!("{} s", u))
        });

        test_sync_command("blink_led(Port 1)", || {
            dev.blink_led(SensorBridgePort::One).map(|_| "OK".into())
        });
        test_sync_command("set_supply_voltage(Port 1, 3.3V)", || {
            dev.set_supply_voltage(SensorBridgePort::One, 3.3)
                .map(|_| "OK".into())
        });
        test_sync_command("switch_supply_on(Port 1)", || {
            dev.switch_supply_on(SensorBridgePort::One)
                .map(|_| "OK".into())
        });
        test_sync_command("measure_voltage(Port 1)", || {
            dev.measure_voltage(SensorBridgePort::One)
                .map(|v| format!("{:.3} V", v))
        });
        test_sync_command("set_i2c_frequency(Port 1, 400kHz)", || {
            dev.set_i2c_frequency(SensorBridgePort::One, 400_000)
                .map(|_| "OK".into())
        });
        test_sync_command("scan_i2c(Port 1)", || {
            dev.scan_i2c(SensorBridgePort::One, 1, 127)
                .map(|addrs| format!("{:?}", addrs))
        });
        test_sync_command("set_spi_config(Port 1, Mode 0, 1MHz)", || {
            dev.set_spi_config(SensorBridgePort::One, SensorBridgeSpiMode::Mode0, 1_000_000)
                .map(|_| "OK".into())
        });
        test_sync_command("transceive_spi(Port 1, [0x00])", || {
            dev.transceive_spi(SensorBridgePort::One, &[0x00])
                .map(|rx| format!("{:02X?}", rx))
        });
        test_sync_command("switch_supply_off(Port 1)", || {
            dev.switch_supply_off(SensorBridgePort::One)
                .map(|_| "OK".into())
        });

        let mut sync_proxy = SensorBridgeI2cProxy::new(dev, SensorBridgePort::One)?;
        let res = sync_proxy.transceive(0x70, &[0x00], 0, 0.001, 0.05);
        println!(
            "  {:<35} -> Result: status={}, err={:?}, rx={:02X?}",
            "SyncI2cProxy.transceive(0x70)", res.status, res.error, res.rx_data
        );
    }

    // -----------------------------------------------------------------------
    // 2. Asynchronous Rust Driver
    // -----------------------------------------------------------------------
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;

    rt.block_on(async {
        println!("\n--- 2. Testing Asynchronous Rust Driver ---");
        let mut port_async = AsyncSerialPort::new("/dev/ttyUSB0", 460_800);
        port_async.open().unwrap();
        let conn_async = AsyncShdlcConnection::new(Box::new(port_async));
        let mut adev = AsyncSensorBridgeDevice::new(conn_async, 0x00);

        test_async_command(
            "blink_led(Port 2)",
            adev.blink_led(SensorBridgePort::Two)
                .await
                .map(|_| "OK".to_string()),
        )
        .await;
        test_async_command(
            "set_supply_voltage(Port 2, 5.0V)",
            adev.set_supply_voltage(SensorBridgePort::Two, 5.0)
                .await
                .map(|_| "OK".to_string()),
        )
        .await;
        test_async_command(
            "switch_supply_on(Port 2)",
            adev.switch_supply_on(SensorBridgePort::Two)
                .await
                .map(|_| "OK".to_string()),
        )
        .await;
        test_async_command(
            "measure_voltage(Port 2)",
            adev.measure_voltage(SensorBridgePort::Two)
                .await
                .map(|v| format!("{:.3} V", v)),
        )
        .await;
        test_async_command(
            "set_i2c_frequency(Port 2, 1MHz)",
            adev.set_i2c_frequency(SensorBridgePort::Two, 1_000_000)
                .await
                .map(|_| "OK".to_string()),
        )
        .await;
        test_async_command(
            "scan_i2c(Port 2)",
            adev.scan_i2c(SensorBridgePort::Two, 1, 127)
                .await
                .map(|addrs| format!("{:?}", addrs)),
        )
        .await;
        test_async_command(
            "transceive_spi(Port 2, [0x00])",
            adev.transceive_spi(SensorBridgePort::Two, &[0x00])
                .await
                .map(|rx| format!("{:02X?}", rx)),
        )
        .await;
        test_async_command(
            "switch_supply_off(Port 2)",
            adev.switch_supply_off(SensorBridgePort::Two)
                .await
                .map(|_| "OK".to_string()),
        )
        .await;

        let mut async_proxy = AsyncSensorBridgeI2cProxy::new(adev, SensorBridgePort::Two).unwrap();
        let res = async_proxy.transceive(0x44, &[], 0, 0.001, 0.05).await;
        println!(
            "  {:<35} -> Result: status={}, err={:?}, rx={:02X?}",
            "AsyncI2cProxy.transceive(0x44)", res.status, res.error, res.rx_data
        );
    });

    println!("\nAll hardware tests completed.");
    Ok(())
}

fn test_sync_command<F>(name: &str, f: F)
where
    F: FnOnce() -> Result<String, SensorBridgeError>,
{
    match f() {
        Ok(val) => println!("  {:<35} -> Success: {}", name, val),
        Err(e) => print_error(name, e),
    }
}

async fn test_async_command<T, E>(name: &str, res: Result<T, E>)
where
    T: std::fmt::Display,
    E: std::fmt::Display + Into<SensorBridgeError>,
{
    match res {
        Ok(val) => println!("  {:<35} -> Success: {}", name, val),
        Err(e) => print_error(name, e.into()),
    }
}

fn print_error(name: &str, err: SensorBridgeError) {
    match err {
        SensorBridgeError::Shdlc(
            rust_shdlc_driver::protocol::errors::ShdlcError::DeviceError { code, message },
        ) => {
            println!("  {:<35} -> Error: code=0x{:02X} ({})", name, code, message);
        }
        other => println!("  {:<35} -> Error: {}", name, other),
    }
}
