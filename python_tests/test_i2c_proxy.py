# -*- coding: utf-8 -*-
import pytest
from conftest import make_resp
from rust_shdlc_driver import (
    AsyncShdlcConnection,
    ShdlcConnection,
    ShdlcMockPort,
)
from sensirion_uart_sensorbridge import (
    AsyncSensorBridgeI2cProxy,
    AsyncSensorBridgeShdlcDevice,
    SensorBridgeI2cProxy,
    SensorBridgePort,
    SensorBridgeShdlcDevice,
)


def test_sync_i2c_proxy():
    port = ShdlcMockPort(bitrate=460800)
    conn = ShdlcConnection(port)
    bridge = SensorBridgeShdlcDevice(conn, slave_address=0)
    proxy = SensorBridgeI2cProxy(bridge, SensorBridgePort.ONE)

    assert proxy.description == "SensorBridge"
    assert proxy.channel_count is None

    # Success transceive
    port.push_rx_data(make_resp(0, 0x11, 0x00, b"\xaa\xbb"))
    status, err, rx = proxy.transceive(
        slave_address=0x70,
        tx_data=b"\x01",
        rx_length=2,
        read_delay=0.001,
        timeout=0.05,
    )
    assert status == SensorBridgeI2cProxy.STATUS_OK
    assert err is None
    assert rx == b"\xaa\xbb"

    # NACK error (0x29)
    port.push_rx_data(make_resp(0, 0x11, 0x29, b""))
    status, err, rx = proxy.transceive(
        slave_address=0x70,
        tx_data=b"\x01",
        rx_length=2,
        read_delay=0.001,
        timeout=0.05,
    )
    assert status == SensorBridgeI2cProxy.STATUS_NACK
    assert rx == b""

    # Timeout error (0x2B)
    port.push_rx_data(make_resp(0, 0x11, 0x2B, b""))
    status, err, rx = proxy.transceive(
        slave_address=0x70,
        tx_data=b"\x01",
        rx_length=2,
        read_delay=0.001,
        timeout=0.05,
    )
    assert status == SensorBridgeI2cProxy.STATUS_TIMEOUT
    assert rx == b""


@pytest.mark.asyncio
async def test_async_i2c_proxy():
    port = ShdlcMockPort(bitrate=460800)
    conn = AsyncShdlcConnection(port)
    bridge = AsyncSensorBridgeShdlcDevice(conn, slave_address=0)
    proxy = AsyncSensorBridgeI2cProxy(bridge, SensorBridgePort.TWO)

    assert proxy.description == "SensorBridge"
    assert proxy.channel_count is None

    # Success transceive
    port.push_rx_data(make_resp(0, 0x11, 0x00, b"\x11\x22"))
    status, err, rx = await proxy.transceive(
        slave_address=0x44,
        tx_data=b"\x00",
        rx_length=2,
        read_delay=0.001,
        timeout=0.05,
    )
    assert status == AsyncSensorBridgeI2cProxy.STATUS_OK
    assert err is None
    assert rx == b"\x11\x22"
