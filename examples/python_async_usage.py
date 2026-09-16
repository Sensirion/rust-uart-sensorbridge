#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Example showing asynchronous usage of SEK-SensorBridge in Python with asyncio.
"""

import asyncio
import sys

from rust_shdlc_driver import AsyncShdlcConnection, ShdlcSerialPort
from sensirion_uart_sensorbridge import (
    AsyncSensorBridgeShdlcDevice,
    SensorBridgePort,
)


async def main():
    port_name = sys.argv[1] if len(sys.argv) > 1 else "/dev/ttyUSB0"
    baudrate = 460800

    print(f"Connecting to {port_name} at {baudrate} baud...")
    port = ShdlcSerialPort(port_name, baudrate)
    conn = AsyncShdlcConnection(port)
    bridge = AsyncSensorBridgeShdlcDevice(conn, slave_address=0)

    # Configure Port 1 with 3.3V and enable supply
    print("Enabling Port 1 at 3.3V...")
    await bridge.set_supply_voltage(SensorBridgePort.ONE, 3.3)
    await bridge.switch_supply_on(SensorBridgePort.ONE)
    await bridge.set_i2c_frequency(SensorBridgePort.ONE, 400000)

    await asyncio.sleep(0.05)

    # Scan I2C devices
    devices = await bridge.scan_i2c(SensorBridgePort.ONE)
    print(f"Found I2C devices on Port 1: {[hex(d) for d in devices]}")

    # Start a repeated I2C transceive measurement at 100ms interval (100,000 us)
    print("Starting repeated I2C transceive on Port 1...")
    handle = await bridge.start_repeated_i2c_transceive(
        port=SensorBridgePort.ONE,
        interval_us=100000,
        address=0x70,
        tx_data=b"\x78\x66",
        rx_length=6,
        timeout_us=10000,
    )

    print(f"Transceive running with handle {handle.raw_handle:#04x}. Collecting samples for 1s...")
    await asyncio.sleep(1.0)

    # Read buffer
    response = await bridge.read_buffer(handle, max_reads=50)
    print(
        f"Read {len(response.values)} samples from buffer "
        f"(lost: {response.lost_bytes}, remaining: {response.remaining_bytes})"
    )

    for i, sample in enumerate(response.values):
        try:
            print(f"  Sample {i}: {sample.data.hex()}")
        except Exception as e:
            print(f"  Sample {i} error: {e}")

    # Stop repeated transceive and power down
    await bridge.stop_repeated_i2c_transceive(handle)
    await bridge.switch_supply_off(SensorBridgePort.ONE)
    print("Done.")


if __name__ == "__main__":
    asyncio.run(main())
