# -*- coding: utf-8 -*-
import pytest
from sensirion_uart_sensorbridge import (
    I2C_FREQUENCIES,
    SPI_FREQUENCIES,
    VOLTAGES,
    SensorBridgePort,
    i2c_frequency_to_byte,
    port_to_byte,
    spi_frequency_to_byte,
    voltage_to_byte,
)


def test_sensorbridge_port_enum():
    assert int(SensorBridgePort.ONE) == 0x00
    assert int(SensorBridgePort.TWO) == 0x01
    assert int(SensorBridgePort.ALL) == 0xFF


def test_port_to_byte():
    assert port_to_byte(SensorBridgePort.ONE) == 0x00
    assert port_to_byte(SensorBridgePort.TWO) == 0x01
    assert port_to_byte(0x00) == 0x00
    assert port_to_byte(0x01) == 0x01

    with pytest.raises(ValueError):
        port_to_byte(SensorBridgePort.ALL, accept_all=False)

    assert port_to_byte(SensorBridgePort.ALL, accept_all=True) == 0xFF
    assert port_to_byte(0xFF, accept_all=True) == 0xFF

    with pytest.raises(ValueError):
        port_to_byte(42)


def test_voltage_to_byte():
    for v, b in VOLTAGES.items():
        assert voltage_to_byte(v) == b

    with pytest.raises(ValueError):
        voltage_to_byte(1.3)


def test_i2c_frequency_to_byte():
    for f, b in I2C_FREQUENCIES.items():
        assert i2c_frequency_to_byte(f) == b

    with pytest.raises(ValueError):
        i2c_frequency_to_byte(200000)


def test_spi_frequency_to_byte():
    for f, b in SPI_FREQUENCIES.items():
        assert spi_frequency_to_byte(f) == b

    with pytest.raises(ValueError):
        spi_frequency_to_byte(500000)
