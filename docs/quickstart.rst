Quickstart
==========

Installation
------------

Rust (Cargo)
^^^^^^^^^^^^

Add the dependency to your ``Cargo.toml``:

.. code-block:: toml

   [dependencies]
   rust-uart-sensorbridge = "0.1.0"
   rust-shdlc-driver = "0.1.0"
   tokio = { version = "1.0", features = ["full"] }

Or add it using the ``cargo`` CLI:

.. code-block:: bash

   cargo add rust-uart-sensorbridge
   cargo add rust-shdlc-driver
   cargo add tokio --features full

Python (pip)
^^^^^^^^^^^^

It is recommended to install the library inside an isolated Python virtual environment:

.. code-block:: bash

   # Create and activate a virtual environment
   python3 -m venv .venv
   source .venv/bin/activate  # On Windows: .\.venv\Scripts\activate

   # Install the package
   pip install sensirion-uart-sensorbridge

Basic Usage
-----------

Rust Examples
^^^^^^^^^^^^^

Synchronous Rust
~~~~~~~~~~~~~~~~

.. code-block:: rust

   use sensirion_uart_sensorbridge::device::SensorBridgeDevice;
   use sensirion_uart_sensorbridge::protocol::definitions::SensorBridgePort;
   use rust_shdlc_driver::connection::ShdlcConnection;
   use rust_shdlc_driver::transport::serial::AsyncSerialPort;

   fn main() -> Result<(), Box<dyn std::error::Error>> {
       let mut port = AsyncSerialPort::new("/dev/ttyUSB0", 460_800);
       port.open()?;
       let conn = ShdlcConnection::new(Box::new(port))?;
       let mut bridge = SensorBridgeDevice::new(conn, 0x00);

       // Configure Port 1 with 3.3V and enable supply
       bridge.set_supply_voltage(SensorBridgePort::One, 3.3)?;
       bridge.switch_supply_on(SensorBridgePort::One)?;
       bridge.set_i2c_frequency(SensorBridgePort::One, 400_000)?;

       // Scan for connected I2C devices
       let devices = bridge.scan_i2c(SensorBridgePort::One, 1, 127)?;
       println!("Found I2C devices on Port 1: {:?}", devices);

       // Power off before exiting
       bridge.switch_supply_off(SensorBridgePort::One)?;
       Ok(())
   }

Asynchronous Rust
~~~~~~~~~~~~~~~~~

.. code-block:: rust

   use sensirion_uart_sensorbridge::device::AsyncSensorBridgeDevice;
   use sensirion_uart_sensorbridge::protocol::definitions::SensorBridgePort;
   use rust_shdlc_driver::connection::AsyncShdlcConnection;
   use rust_shdlc_driver::transport::serial::AsyncSerialPort;

   #[tokio::main]
   async fn main() -> Result<(), Box<dyn std::error::Error>> {
       let mut port = AsyncSerialPort::new("/dev/ttyUSB0", 460_800);
       port.open()?;
       let conn = AsyncShdlcConnection::new(Box::new(port));
       let mut bridge = AsyncSensorBridgeDevice::new(conn, 0x00);

       // Power Port 1
       bridge.set_supply_voltage(SensorBridgePort::One, 3.3).await?;
       bridge.switch_supply_on(SensorBridgePort::One).await?;

       // Measure AIN analog pin voltage
       let voltage = bridge.measure_voltage(SensorBridgePort::One).await?;
       println!("Measured analog voltage on Port 1: {:.3} V", voltage);

       bridge.switch_supply_off(SensorBridgePort::One).await?;
       Ok(())
   }

Python Examples
^^^^^^^^^^^^^^^

Synchronous Python
~~~~~~~~~~~~~~~~~~

.. code-block:: python

   from rust_shdlc_driver import ShdlcSerialPort, ShdlcConnection
   from sensirion_uart_sensorbridge import SensorBridgeShdlcDevice, SensorBridgePort

   port = ShdlcSerialPort.open("/dev/ttyUSB0", 460800)
   conn = ShdlcConnection(port)
   bridge = SensorBridgeShdlcDevice(conn, slave_address=0)

   # Enable 3.3V on Port 1 and set 400kHz I2C frequency
   bridge.set_supply_voltage(SensorBridgePort.ONE, 3.3)
   bridge.switch_supply_on(SensorBridgePort.ONE)
   bridge.set_i2c_frequency(SensorBridgePort.ONE, 400000)

   # Scan for I2C sensors
   devices = bridge.scan_i2c(SensorBridgePort.ONE)
   print("Found I2C devices:", [hex(d) for d in devices])

   # Power down
   bridge.switch_supply_off(SensorBridgePort.ONE)

Asynchronous Python
~~~~~~~~~~~~~~~~~~~

.. code-block:: python

   import asyncio
   from rust_shdlc_driver import ShdlcSerialPort, AsyncShdlcConnection
   from sensirion_uart_sensorbridge import AsyncSensorBridgeShdlcDevice, SensorBridgePort

   async def main():
       port = ShdlcSerialPort.open("/dev/ttyUSB0", 460800)
       conn = AsyncShdlcConnection(port)
       bridge = AsyncSensorBridgeShdlcDevice(conn, slave_address=0)

       await bridge.set_supply_voltage(SensorBridgePort.ONE, 3.3)
       await bridge.switch_supply_on(SensorBridgePort.ONE)

       # Measure AIN analog pin voltage
       val = await bridge.measure_voltage(SensorBridgePort.ONE)
       print(f"Port 1 Voltage: {val:.3f} V")

       await bridge.switch_supply_off(SensorBridgePort.ONE)

   asyncio.run(main())
