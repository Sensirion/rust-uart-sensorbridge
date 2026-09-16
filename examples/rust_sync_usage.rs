use rust_shdlc_driver::connection::ShdlcConnection;
use rust_shdlc_driver::transport::serial::AsyncSerialPort;
use sensirion_uart_sensorbridge::device::SensorBridgeDevice;
use sensirion_uart_sensorbridge::protocol::definitions::SensorBridgePort;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("SEK-SensorBridge Synchronous Rust Example");

    let port_name = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "/dev/ttyUSB0".to_string());
    let baudrate = 460_800;

    println!("Connecting to {} at {} baud...", port_name, baudrate);
    let mut port = AsyncSerialPort::new(&port_name, baudrate);
    port.open()?;
    let conn = ShdlcConnection::new(Box::new(port))?;
    let mut bridge = SensorBridgeDevice::new(conn, 0x00);

    // Get Device Information
    let product_name = bridge.get_product_name()?;
    let serial_number = bridge.get_serial_number()?;
    let version = bridge.get_version()?;
    println!("Connected to: {} (S/N: {})", product_name, serial_number);
    println!(
        "Firmware version: {}.{}",
        version.firmware.major, version.firmware.minor
    );

    // Blink Port 1 LED
    println!("Blinking LED on Port 1...");
    bridge.blink_led(SensorBridgePort::One)?;

    // Configure Port 1: 3.3V supply voltage
    println!("Setting Port 1 supply voltage to 3.3V and enabling power...");
    bridge.set_supply_voltage(SensorBridgePort::One, 3.3)?;
    bridge.switch_supply_on(SensorBridgePort::One)?;

    // Set I2C frequency to 400kHz
    bridge.set_i2c_frequency(SensorBridgePort::One, 400_000)?;

    // Scan for connected I2C devices on Port 1
    println!("Scanning for I2C devices on Port 1...");
    let devices = bridge.scan_i2c(SensorBridgePort::One, 1, 127)?;
    println!("Found {} device(s): {:?}", devices.len(), devices);

    // Read analog voltage on Port 1
    let voltage = bridge.measure_voltage(SensorBridgePort::One)?;
    println!("Measured analog voltage on Port 1: {:.3} V", voltage);

    // Turn off power before exiting
    bridge.switch_supply_off(SensorBridgePort::One)?;
    println!("Done.");

    Ok(())
}
