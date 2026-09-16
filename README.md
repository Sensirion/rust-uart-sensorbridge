# Sensirion SEK-SensorBridge Driver (Rust & Python)

[![CI](https://github.com/Sensirion/rust-uart-sensorbridge/actions/workflows/ci.yml/badge.svg)](https://github.com/Sensirion/rust-uart-sensorbridge/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/license-BSD--3--Clause-blue.svg)](LICENSE)

High-performance, async and sync SEK-SensorBridge hardware driver written in Rust with native PyO3 / Maturin bindings for Python.

## Overview

The Sensirion SEK-SensorBridge enables connecting Sensirion sensors (such as environmental, flow, and humidity sensors) to a computer via USB/UART using the SHDLC (Sensirion High-Level Data Link Control) protocol.

This library provides:
- **Clean 4-Layer Architecture**:
  - **Transport Layer**: Non-blocking serial, TCP, and mock transport interfaces (powered by `rust-shdlc-driver`).
  - **Connection Layer**: Async and sync SHDLC connection managers.
  - **Protocol Layer**: Type-safe command encoding, decoding, port/voltage/frequency definitions, and structured error hierarchies.
  - **Application Layer**: High-level driver (`SensorBridgeDevice`, `AsyncSensorBridgeDevice`), unified I²C proxies (`SensorBridgeI2cProxy`, `AsyncSensorBridgeI2cProxy`), and firmware update capabilities (`SensorBridgeFirmwareImage`).
- **Synchronous and Asynchronous APIs** in both Rust and Python (native `asyncio` integration).
- **Zero-cost FFI bindings** using PyO3 and Maturin.
- **Built-in Mock Transports** for unit testing and CI without physical hardware attached.

---

## Installation & Usage

### Rust (Cargo)

Add `rust-uart-sensorbridge` to your project's `Cargo.toml`:

```toml
[dependencies]
rust-uart-sensorbridge = "0.1.0"
rust-shdlc-driver = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
```

Or add the dependencies via the Cargo CLI:

```bash
cargo add rust-uart-sensorbridge
cargo add rust-shdlc-driver
cargo add tokio --features full
```

### Python (pip)

It is strongly recommended to install and run Python packages inside a virtual environment to prevent dependency conflicts:

```bash
# 1. Create a virtual environment
python3 -m venv .venv

# 2. Activate the virtual environment
# On Linux / macOS:
source .venv/bin/activate
# On Windows (PowerShell):
# .\.venv\Scripts\Activate.ps1
# On Windows (cmd):
# .\.venv\Scripts\activate.bat

# 3. Install the package via pip
pip install sensirion-uart-sensorbridge
```

---

## Quickstart

### Rust (Sync)

```rust
use sensirion_uart_sensorbridge::device::SensorBridgeDevice;
use sensirion_uart_sensorbridge::protocol::definitions::SensorBridgePort;
use rust_shdlc_driver::connection::ShdlcConnection;
use rust_shdlc_driver::transport::serial::AsyncSerialPort;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut port = AsyncSerialPort::new("/dev/ttyUSB0", 460_800);
    port.open()?;
    let conn = ShdlcConnection::new(Box::new(port))?;
    let mut bridge = SensorBridgeDevice::new(conn, 0x00);

    // Configure Port 1 with 3.3V supply voltage and enable power
    bridge.set_supply_voltage(SensorBridgePort::One, 3.3)?;
    bridge.switch_supply_on(SensorBridgePort::One)?;
    bridge.set_i2c_frequency(SensorBridgePort::One, 400_000)?;

    // Scan for connected I2C devices on Port 1
    let devices = bridge.scan_i2c(SensorBridgePort::One, 1, 127)?;
    println!("Found devices on Port 1: {:?}", devices);

    // Turn off power before exiting
    bridge.switch_supply_off(SensorBridgePort::One)?;
    Ok(())
}
```

### Rust (Async)

```rust
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

    // Measure analog voltage on Port 1
    let voltage = bridge.measure_voltage(SensorBridgePort::One).await?;
    println!("Measured analog voltage on Port 1: {:.3} V", voltage);

    bridge.switch_supply_off(SensorBridgePort::One).await?;
    Ok(())
}
```

### Python (Sync)

```python
from rust_shdlc_driver import ShdlcSerialPort, ShdlcConnection
from sensirion_uart_sensorbridge import SensorBridgeShdlcDevice, SensorBridgePort

# Open serial port and establish connection
port = ShdlcSerialPort.open("/dev/ttyUSB0", 460800)
conn = ShdlcConnection(port)
bridge = SensorBridgeShdlcDevice(conn, slave_address=0)

# Configure Port 1 with 3.3V power supply and 400kHz I2C frequency
bridge.set_supply_voltage(SensorBridgePort.ONE, 3.3)
bridge.switch_supply_on(SensorBridgePort.ONE)
bridge.set_i2c_frequency(SensorBridgePort.ONE, 400000)

# Scan for connected I2C devices
devices = bridge.scan_i2c(SensorBridgePort.ONE)
print(f"Found I2C devices on Port 1: {[hex(d) for d in devices]}")

# Blink Port 1 LED
bridge.blink_led(SensorBridgePort.ONE)

# Power down
bridge.switch_supply_off(SensorBridgePort.ONE)
```

### Python (Async)

```python
import asyncio
from rust_shdlc_driver import ShdlcSerialPort, AsyncShdlcConnection
from sensirion_uart_sensorbridge import AsyncSensorBridgeShdlcDevice, SensorBridgePort


async def main():
    port = ShdlcSerialPort.open("/dev/ttyUSB0", 460800)
    conn = AsyncShdlcConnection(port)
    bridge = AsyncSensorBridgeShdlcDevice(conn, slave_address=0)

    # Power Port 1
    await bridge.set_supply_voltage(SensorBridgePort.ONE, 3.3)
    await bridge.switch_supply_on(SensorBridgePort.ONE)

    # Start repeated I2C transceive measurement at 100ms interval
    handle = await bridge.start_repeated_i2c_transceive(
        port=SensorBridgePort.ONE,
        interval_us=100000,
        address=0x70,
        tx_data=b"\x78\x66",
        rx_length=6,
        timeout_us=10000,
    )

    await asyncio.sleep(0.5)

    # Read buffered measurements
    response = await bridge.read_buffer(handle, max_reads=50)
    for sample in response.values:
        print("Sample:", sample.data.hex())

    await bridge.stop_repeated_i2c_transceive(handle)
    await bridge.switch_supply_off(SensorBridgePort.ONE)


asyncio.run(main())
```

---

## Testing & Quality Assurance

```bash
# Run Rust tests (unit, mock integration, protocol tests)
cargo test

# Run Rust formatting and linter
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings

# Build Python bindings in local virtual environment
maturin develop

# Run Python test suite
pytest python_tests

# Run Python linting and formatting
ruff check .
ruff format --check .
```

---

## Documentation & Developer Build Instructions

Detailed documentation, architecture overviews, API references, and comprehensive build instructions for developers are available in the Sphinx documentation under [`docs/`](docs/).

For step-by-step developer build instructions (including prerequisites, setting up virtual environments, building with Maturin, and compiling Sphinx docs), refer to the [Developer Build Guide](docs/development.rst).

To build the HTML documentation locally:

```bash
# Install doc dependencies inside your virtual environment
pip install -r docs/requirements.txt

# Build Sphinx documentation
cd docs
make html
```

The output will be generated in `docs/_build/html/index.html`.

---

## License

See [LICENSE](LICENSE). Distributed under the BSD 3-Clause License.
