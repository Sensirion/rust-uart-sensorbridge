# -*- coding: utf-8 -*-
from struct import pack

import pytest
from conftest import make_resp
from rust_shdlc_driver import AsyncShdlcConnection, ShdlcMockPort
from sensirion_uart_sensorbridge import (
    AsyncSensorBridgeShdlcDevice,
    SensorBridgePort,
)


@pytest.mark.asyncio
async def test_async_sensorbridge_commands():
    port = ShdlcMockPort(bitrate=460800)
    conn = AsyncShdlcConnection(port)
    bridge = AsyncSensorBridgeShdlcDevice(conn, slave_address=0)

    # 1. Blink
    port.push_rx_data(make_resp(0, 0x70, 0x00, b""))
    await bridge.blink_led(SensorBridgePort.ONE)

    # 2. Measure Voltage
    float_bytes = pack("<f", 5.02)
    port.push_rx_data(make_resp(0, 0x80, 0x00, float_bytes))
    v = await bridge.measure_voltage(SensorBridgePort.TWO)
    assert abs(v - 5.02) < 1e-4

    # 3. Set Supply Voltage
    port.push_rx_data(make_resp(0, 0x00, 0x00, b""))
    await bridge.set_supply_voltage(SensorBridgePort.ONE, 5.0)

    # 4. Switch Supply On / Off
    port.push_rx_data(make_resp(0, 0x01, 0x00, b""))
    await bridge.switch_supply_on(SensorBridgePort.ONE)

    port.push_rx_data(make_resp(0, 0x01, 0x00, b""))
    await bridge.switch_supply_off(SensorBridgePort.ONE)

    # 5. Set I2C Frequency
    port.push_rx_data(make_resp(0, 0x02, 0x00, b""))
    await bridge.set_i2c_frequency(SensorBridgePort.ONE, 1000000)

    # 6. Scan I2C
    port.push_rx_data(make_resp(0, 0x10, 0x00, b"\x44"))
    addrs = await bridge.scan_i2c(SensorBridgePort.ONE, 1, 127)
    assert addrs == [0x44]

    # 7. Transceive I2C
    port.push_rx_data(make_resp(0, 0x11, 0x00, b"\x12\x34\x56"))
    rx = await bridge.transceive_i2c(SensorBridgePort.ONE, 0x44, b"\x2c\x06", 3, 5000)
    assert rx == b"\x12\x34\x56"

    # 8. Start Repeated I2C Transceive
    port.push_rx_data(make_resp(0, 0x12, 0x00, b"\x01"))
    handle = await bridge.start_repeated_i2c_transceive(
        port=SensorBridgePort.TWO,
        interval_us=50000,
        address=0x44,
        tx_data=b"",
        rx_length=2,
        timeout_us=1000,
    )
    assert handle.raw_handle == 0x01

    # 9. Read Buffer
    buf_payload = b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\xde\xad"
    port.push_rx_data(make_resp(0, 0x50, 0x00, buf_payload))
    res = await bridge.read_buffer(handle, 10)
    assert len(res.values) == 1
    assert res.values[0].data == b"\xde\xad"

    # 10. Stop Repeated Transceive
    port.push_rx_data(make_resp(0, 0x51, 0x00, b""))
    await bridge.stop_repeated_i2c_transceive(handle)

    # 11. SPI Config & Transceive
    port.push_rx_data(make_resp(0, 0x60, 0x00, b""))
    await bridge.set_spi_config(SensorBridgePort.ONE, 1, 5000000)

    port.push_rx_data(make_resp(0, 0x61, 0x00, b"\x99"))
    spi_rx = await bridge.transceive_spi(SensorBridgePort.ONE, b"\x88")
    assert spi_rx == b"\x99"
