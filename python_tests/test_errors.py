# -*- coding: utf-8 -*-
from sensirion_uart_sensorbridge import (
    SensorBridgeDeviceI2cNackError,
    SensorBridgeDeviceI2cTimeoutError,
    SensorBridgeFatalError,
    SensorBridgeFrameChecksumError,
    SensorBridgeFrameSizeInfoMismatchError,
    SensorBridgeFunctionalityNotImplementedError,
    SensorBridgeI2cError,
    SensorBridgeI2cNackError,
    SensorBridgeI2cTimeoutError,
    SensorBridgeI2cTimingError,
    SensorBridgeNoMorePeriodicMeasurementsPossibleError,
    i2c_error_from_code,
)


def test_i2c_error_from_code():
    assert i2c_error_from_code(0) is None

    err_nack = i2c_error_from_code(1)
    assert isinstance(err_nack, SensorBridgeI2cNackError)
    assert err_nack.error_code == 1

    err_timeout = i2c_error_from_code(2)
    assert isinstance(err_timeout, SensorBridgeI2cTimeoutError)
    assert err_timeout.error_code == 2

    err_timing = i2c_error_from_code(3)
    assert isinstance(err_timing, SensorBridgeI2cTimingError)
    assert err_timing.error_code == 3

    err_unknown = i2c_error_from_code(99)
    assert isinstance(err_unknown, SensorBridgeI2cError)
    assert err_unknown.error_code == 99


def test_device_errors_inheritance():
    assert issubclass(SensorBridgeFunctionalityNotImplementedError, Exception)
    assert issubclass(SensorBridgeNoMorePeriodicMeasurementsPossibleError, Exception)
    assert issubclass(SensorBridgeFrameChecksumError, Exception)
    assert issubclass(SensorBridgeFrameSizeInfoMismatchError, Exception)
    assert issubclass(SensorBridgeDeviceI2cNackError, Exception)
    assert issubclass(SensorBridgeDeviceI2cTimeoutError, Exception)
    assert issubclass(SensorBridgeFatalError, Exception)
