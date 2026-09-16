# -*- coding: utf-8 -*-
import pytest
from sensirion_uart_sensorbridge import (
    BufferedValue,
    ReadBufferResponse,
    RepeatedTransceiveHandle,
    SensorBridgeI2cNackError,
)


def test_repeated_transceive_handle():
    h = RepeatedTransceiveHandle(0x31, 6)
    assert h.raw_handle == 0x31
    assert h.rx_length == 6
    assert h.slot == 3
    assert h.channel == 1


def test_buffered_value_valid():
    bv = BufferedValue(b"\x00\x11\x22")
    assert bv.raw_status == 0
    assert bv.raw_data == b"\x11\x22"
    assert bv.error is None
    assert bv.data == b"\x11\x22"


def test_buffered_value_error():
    bv = BufferedValue(b"\x01\x00\x00")
    assert bv.raw_status == 1
    assert isinstance(bv.error, SensorBridgeI2cNackError)
    with pytest.raises(SensorBridgeI2cNackError):
        _ = bv.data


def test_read_buffer_response():
    # 2 packets of length 3 (1 status + 2 data)
    raw = b"\x00\x11\x22\x00\x33\x44"
    resp = ReadBufferResponse(2, 5, 0, raw)
    assert resp.lost_bytes == 5
    assert resp.remaining_bytes == 0
    assert len(resp.values) == 2
    assert resp.values[0].data == b"\x11\x22"
    assert resp.values[1].data == b"\x33\x44"
