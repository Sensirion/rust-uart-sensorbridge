use rust_shdlc_driver::connection::AsyncShdlcConnection;
use rust_shdlc_driver::transport::serial::AsyncSerialPort;
use sensirion_uart_sensorbridge::device::AsyncSensorBridgeDevice;
use sensirion_uart_sensorbridge::protocol::definitions::SensorBridgePort;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("SEK-SensorBridge Asynchronous Rust Example");

    let port_name = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "/dev/ttyUSB0".to_string());
    let baudrate = 460_800;

    println!("Connecting to {} at {} baud...", port_name, baudrate);
    let mut port = AsyncSerialPort::new(&port_name, baudrate);
    port.open()?;
    let conn = AsyncShdlcConnection::new(Box::new(port));
    let mut bridge = AsyncSensorBridgeDevice::new(conn, 0x00);

    // Get Device Information
    let product_name = bridge.get_product_name().await?;
    let serial_number = bridge.get_serial_number().await?;
    let version = bridge.get_version().await?;
    println!("Connected to: {} (S/N: {})", product_name, serial_number);
    println!(
        "Firmware version: {}.{}",
        version.firmware.major, version.firmware.minor
    );

    // Configure and power Port 1
    bridge
        .set_supply_voltage(SensorBridgePort::One, 3.3)
        .await?;
    bridge.switch_supply_on(SensorBridgePort::One).await?;
    bridge
        .set_i2c_frequency(SensorBridgePort::One, 400_000)
        .await?;

    // Start a repeated I2C transceive measurement at 100ms interval (100_000 us)
    // Example reading 6 bytes from I2C address 0x70 with 1000us timeout
    println!("Starting repeated I2C transceive on Port 1...");
    let handles = bridge
        .start_repeated_i2c_transceive(
            SensorBridgePort::One,
            100_000,
            0x70,
            &[0x78, 0x66], // example measurement trigger command
            6,
            10_000,
            0,
        )
        .await?;

    if let Some(handle) = handles.first() {
        println!(
            "Transceive running with handle {:#04x}. Collecting samples...",
            handle.raw_handle
        );
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

        let buffer_response = bridge.read_buffer(*handle, 50).await?;
        println!(
            "Read {} samples from buffer (lost: {}, remaining: {})",
            buffer_response.values.len(),
            buffer_response.lost_bytes,
            buffer_response.remaining_bytes
        );

        for (idx, val) in buffer_response.values.iter().enumerate() {
            match val.data() {
                Ok(data) => println!("  Sample {}: {:02X?}", idx, data),
                Err(err) => eprintln!("  Sample {} error: {}", idx, err),
            }
        }

        // Stop repeated transceive
        bridge.stop_repeated_i2c_transceive(Some(*handle)).await?;
        println!("Repeated transceive stopped.");
    }

    // Power off Port 1
    bridge.switch_supply_off(SensorBridgePort::One).await?;
    println!("Done.");

    Ok(())
}
