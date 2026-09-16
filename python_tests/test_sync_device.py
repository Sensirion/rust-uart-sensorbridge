# -*- coding: utf-8 -*-
import threading
import time
from struct import pack

from conftest import make_resp
from rust_shdlc_driver import ShdlcConnection, ShdlcMockPort
from sensirion_uart_sensorbridge import SensorBridgePort, SensorBridgeShdlcDevice


def test_blocking_sensorbridge_requests_detach_gil():
    port = ShdlcMockPort(bitrate=460800)
    conn = ShdlcConnection(port)
    bridge = SensorBridgeShdlcDevice(conn, slave_address=0)

    thread_ran = False

    def delayed_response():
        nonlocal thread_ran
        time.sleep(0.05)
        thread_ran = True
        port.push_rx_data(make_resp(0, 0x11, 0x00, b"\xaa\xbb"))

    t = threading.Thread(target=delayed_response)
    t.start()

    rx = bridge.transceive_i2c(SensorBridgePort.ONE, 0x70, b"\x01", 2, 1000)
    t.join()

    assert rx == b"\xaa\xbb"
    assert thread_ran is True


def test_sync_sensorbridge_commands():
    port = ShdlcMockPort(bitrate=460800)
    conn = ShdlcConnection(port)
    bridge = SensorBridgeShdlcDevice(conn, slave_address=0)

    # 1. Blink
    port.push_rx_data(make_resp(0, 0x70, 0x00, b""))
    bridge.blink_led(SensorBridgePort.ONE)

    # 2. Measure Voltage (little-endian float in payload)
    float_bytes = pack("<f", 3.295)
    port.push_rx_data(make_resp(0, 0x80, 0x00, float_bytes))
    v = bridge.measure_voltage(SensorBridgePort.ONE)
    assert abs(v - 3.295) < 1e-4

    # 3. Set Supply Voltage
    port.push_rx_data(make_resp(0, 0x00, 0x00, b""))
    bridge.set_supply_voltage(SensorBridgePort.ALL, 3.3)

    # 4. Switch Supply On / Off
    port.push_rx_data(make_resp(0, 0x01, 0x00, b""))
    bridge.switch_supply_on(SensorBridgePort.ONE)

    port.push_rx_data(make_resp(0, 0x01, 0x00, b""))
    bridge.switch_supply_off(SensorBridgePort.ONE)

    # 5. Set I2C Frequency
    port.push_rx_data(make_resp(0, 0x02, 0x00, b""))
    bridge.set_i2c_frequency(SensorBridgePort.TWO, 400000)

    # 6. Scan I2C
    port.push_rx_data(make_resp(0, 0x10, 0x00, b"\x69\x70"))
    addrs = bridge.scan_i2c(SensorBridgePort.ONE)
    assert addrs == [0x69, 0x70]

    # 7. Transceive I2C
    port.push_rx_data(make_resp(0, 0x11, 0x00, b"\xaa\xbb"))
    rx = bridge.transceive_i2c(SensorBridgePort.ONE, 0x70, b"\x01", 2, 1000)
    assert rx == b"\xaa\xbb"

    # 8. Start Repeated I2C Transceive
    port.push_rx_data(make_resp(0, 0x12, 0x00, b"\x10\x11"))
    handles = bridge.start_repeated_i2c_transceive(
        port=SensorBridgePort.ALL,
        interval_us=10000,
        address=0x70,
        tx_data=b"\x00",
        rx_length=4,
        timeout_us=1000,
    )
    assert isinstance(handles, tuple)
    assert len(handles) == 2
    assert handles[0].raw_handle == 0x10

    # 9. Read Buffer
    buf_payload = b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x02\x03\x04"
    port.push_rx_data(make_resp(0, 0x50, 0x00, buf_payload))
    res = bridge.read_buffer(handles[0])
    assert res.lost_bytes == 0
    assert len(res.values) == 1
    assert res.values[0].data == b"\x01\x02\x03\x04"

    # 10. Stop Repeated Transceive
    port.push_rx_data(make_resp(0, 0x51, 0x00, b""))
    bridge.stop_repeated_i2c_transceive(handles[0])

    # 11. SPI Config & Transceive
    port.push_rx_data(make_resp(0, 0x60, 0x00, b""))
    bridge.set_spi_config(SensorBridgePort.ONE, 0, 1000000)

    port.push_rx_data(make_resp(0, 0x61, 0x00, b"\xca\xfe"))
    spi_rx = bridge.transceive_spi(SensorBridgePort.ONE, b"\x01\x02")
    assert spi_rx == b"\xca\xfe"
