#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Example showing synchronous usage of SEK-SensorBridge in Python.
"""

import sys
import time

from rust_shdlc_driver import ShdlcConnection, ShdlcSerialPort
from sensirion_uart_sensorbridge import SensorBridgePort, SensorBridgeShdlcDevice


def main():
    port_name = sys.argv[1] if len(sys.argv) > 1 else "/dev/ttyUSB0"
    baudrate = 460800

    print(f"Connecting to {port_name} at {baudrate} baud...")
    port = ShdlcSerialPort(port_name, baudrate)
    conn = ShdlcConnection(port)
    bridge = SensorBridgeShdlcDevice(conn, slave_address=0)

    # Device Info
    product_name = bridge.get_product_name()
    serial_number = bridge.get_serial_number()
    version = bridge.get_version()
    print(f"Connected to {product_name} (S/N: {serial_number})")
    print(f"Firmware Version: {version.firmware.major}.{version.firmware.minor}")

    # Blink Port 1 LED
    print("Blinking Port 1 LED...")
    bridge.blink_led(SensorBridgePort.ONE)

    # Configure Port 1 with 3.3V power supply and 400kHz I2C frequency
    print("Setting Port 1 to 3.3V and enabling power supply...")
    bridge.set_supply_voltage(SensorBridgePort.ONE, 3.3)
    bridge.switch_supply_on(SensorBridgePort.ONE)
    bridge.set_i2c_frequency(SensorBridgePort.ONE, 400000)

    # Wait for sensor startup
    time.sleep(0.05)

    # Scan for connected I2C devices on Port 1
    devices = bridge.scan_i2c(SensorBridgePort.ONE)
    print(f"Found I2C devices on Port 1: {[hex(d) for d in devices]}")

    # Measure analog voltage on Port 1 AIN pin
    voltage = bridge.measure_voltage(SensorBridgePort.ONE)
    print(f"Measured analog voltage on Port 1: {voltage:.3f} V")

    # Power down
    print("Switching off Port 1 power supply...")
    bridge.switch_supply_off(SensorBridgePort.ONE)
    print("Done.")


if __name__ == "__main__":
    main()
