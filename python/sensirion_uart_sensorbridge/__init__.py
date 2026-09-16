# -*- coding: utf-8 -*-
"""
SEK-SensorBridge Driver - High-performance sync and async driver with PyO3 bindings.
"""

import logging

from ._sensirion_uart_sensorbridge import *  # noqa: F401, F403
from ._sensirion_uart_sensorbridge import (
    I2C_FREQUENCIES,
    SPI_FREQUENCIES,
    VOLTAGES,
    BufferedValue,
    ReadBufferResponse,
    # Types
    RepeatedTransceiveHandle,
    SensorBridgeCmdAnalogMeasurement,
    SensorBridgeCmdBlink,
    SensorBridgeCmdFirstTransceive,
    SensorBridgeCmdI2cRepeatedTransceive,
    SensorBridgeCmdI2cScan,
    SensorBridgeCmdPortVoltageOnOff,
    SensorBridgeCmdReadBuffer,
    SensorBridgeCmdSetI2cFrequency,
    # Commands
    SensorBridgeCmdSetPortVoltage,
    SensorBridgeCmdSpiConfig,
    SensorBridgeCmdSpiTransceive,
    SensorBridgeCmdStopRepeatedTransceive,
    SensorBridgeCmdSubsequentTransceive,
    # Errors
    SensorBridgeDeviceError,
    SensorBridgeDeviceI2cNackError,
    SensorBridgeDeviceI2cTimeoutError,
    SensorBridgeFatalError,
    # Firmware
    SensorBridgeFirmwareImage,
    SensorBridgeFrameChecksumError,
    SensorBridgeFrameSizeInfoMismatchError,
    SensorBridgeFunctionalityNotImplementedError,
    SensorBridgeI2cError,
    SensorBridgeI2cNackError,
    # I2C Proxy
    SensorBridgeI2cProxy,
    SensorBridgeI2cTimeoutError,
    SensorBridgeI2cTimingError,
    SensorBridgeNoMorePeriodicMeasurementsPossibleError,
    # Definitions & Enums
    SensorBridgePort,
    # Devices
    SensorBridgeShdlcDevice,
    __version__,
    i2c_error_from_code,
    i2c_frequency_to_byte,
    port_to_byte,
    spi_frequency_to_byte,
    voltage_to_byte,
)

log = logging.getLogger(__name__)


class AsyncSensorBridgeShdlcDevice:
    """
    SEK-SensorBridge asynchronous device driver.
    """

    def __init__(self, connection, slave_address=0):
        self._connection = connection
        self._slave_address = slave_address
        self._last_error_flag = False

    @property
    def connection(self):
        return self._connection

    @property
    def slave_address(self):
        return self._slave_address

    @property
    def last_error_flag(self):
        return self._last_error_flag

    async def execute(self, command):
        res, err_flag = await self._connection.execute(self._slave_address, command, True)
        self._last_error_flag = err_flag
        return res

    async def blink_led(self, port):
        raw_port = port_to_byte(port, accept_all=True)
        return await self.execute(SensorBridgeCmdBlink(raw_port))

    async def measure_voltage(self, port):
        raw_port = port_to_byte(port, accept_all=False)
        return await self.execute(SensorBridgeCmdAnalogMeasurement(raw_port))

    async def set_supply_voltage(self, port, voltage):
        raw_port = port_to_byte(port, accept_all=True)
        raw_voltage = voltage_to_byte(voltage)
        return await self.execute(SensorBridgeCmdSetPortVoltage(raw_port, raw_voltage))

    async def switch_supply_on(self, port):
        raw_port = port_to_byte(port, accept_all=True)
        return await self.execute(SensorBridgeCmdPortVoltageOnOff(raw_port, 1))

    async def switch_supply_off(self, port):
        raw_port = port_to_byte(port, accept_all=True)
        return await self.execute(SensorBridgeCmdPortVoltageOnOff(raw_port, 0))

    async def set_i2c_frequency(self, port, frequency):
        raw_port = port_to_byte(port, accept_all=True)
        raw_freq = i2c_frequency_to_byte(frequency)
        return await self.execute(SensorBridgeCmdSetI2cFrequency(raw_port, raw_freq))

    async def scan_i2c(self, port, first_address=1, last_address=127):
        raw_port = port_to_byte(port, accept_all=False)
        res = await self.execute(SensorBridgeCmdI2cScan(raw_port, first_address, last_address))
        return list(bytearray(res))

    async def transceive_i2c(self, port, address, tx_data, rx_length, timeout_us):
        raw_port = port_to_byte(port, accept_all=False)
        max_tx_len_per_frame = 255 - 15  # 240 bytes
        tx_bytes = bytes(bytearray(tx_data)) if tx_data else b""
        tx_len = len(tx_bytes)

        tx_len_sent = min(tx_len, max_tx_len_per_frame)
        rx_data = await self.execute(
            SensorBridgeCmdFirstTransceive(
                raw_port, address, tx_len, rx_length, int(timeout_us), tx_bytes[0:tx_len_sent]
            )
        )

        while tx_len_sent < tx_len:
            chunk = min(tx_len - tx_len_sent, max_tx_len_per_frame)
            sub_rx = await self.execute(
                SensorBridgeCmdSubsequentTransceive(
                    raw_port, tx_bytes[tx_len_sent : tx_len_sent + chunk]
                )
            )
            rx_data += sub_rx
            tx_len_sent += chunk

        while len(rx_data) < rx_length:
            sub_rx = await self.execute(SensorBridgeCmdSubsequentTransceive(raw_port, b""))
            if not sub_rx:
                break
            rx_data += sub_rx

        return rx_data

    async def start_repeated_i2c_transceive(
        self, port, interval_us, address, tx_data, rx_length, timeout_us, read_delay_us=0
    ):
        raw_port = port_to_byte(port, accept_all=True)
        tx_bytes = bytes(bytearray(tx_data)) if tx_data else b""
        raw_handles = await self.execute(
            SensorBridgeCmdI2cRepeatedTransceive(
                int(interval_us),
                raw_port,
                address,
                len(tx_bytes),
                rx_length,
                int(timeout_us),
                int(read_delay_us),
                tx_bytes,
            )
        )
        handles = [RepeatedTransceiveHandle(h, rx_length) for h in bytearray(raw_handles)]
        return handles[0] if len(handles) == 1 else tuple(handles)

    async def stop_repeated_i2c_transceive(self, handle=None):
        raw_handle = 0xFF if (handle is None) else handle.raw_handle
        return await self.execute(SensorBridgeCmdStopRepeatedTransceive(raw_handle))

    async def read_buffer(self, handle, max_reads=100):
        total_lost_bytes = 0
        total_rx_data = bytes()
        remaining_bytes = 0
        for _ in range(max_reads):
            lost, remaining, rx_data = await self.execute(
                SensorBridgeCmdReadBuffer(handle.raw_handle)
            )
            total_lost_bytes += lost
            remaining_bytes = remaining
            total_rx_data += rx_data
            if remaining_bytes == 0:
                break
        return ReadBufferResponse(
            handle.rx_length, total_lost_bytes, remaining_bytes, total_rx_data
        )

    async def set_spi_config(self, port, mode, frequency):
        raw_port = port_to_byte(port, accept_all=True)
        raw_freq = spi_frequency_to_byte(frequency)
        return await self.execute(SensorBridgeCmdSpiConfig(raw_port, int(mode), raw_freq))

    async def transceive_spi(self, port, tx_data):
        raw_port = port_to_byte(port, accept_all=False)
        tx_bytes = bytes(bytearray(tx_data))
        return await self.execute(SensorBridgeCmdSpiTransceive(raw_port, tx_bytes))


class AsyncSensorBridgeI2cProxy:
    """
    Asynchronous I²C Proxy for SEK-SensorBridge.
    """

    API_VERSION = 1
    STATUS_OK = 0
    STATUS_CHANNEL_DISABLED = 1
    STATUS_NACK = 2
    STATUS_TIMEOUT = 3
    STATUS_UNSPECIFIED_ERROR = 4

    def __init__(self, device, port):
        self._device = device
        self._port = port

    @property
    def description(self):
        return "SensorBridge"

    @property
    def channel_count(self):
        return None

    async def transceive(self, slave_address, tx_data, rx_length, read_delay, timeout):
        total_timeout_us = max(read_delay, timeout) * 1e6
        try:
            rx_data = await self._device.transceive_i2c(
                self._port, slave_address, tx_data or b"", rx_length or 0, total_timeout_us
            )
            return self.STATUS_OK, None, rx_data
        except (SensorBridgeI2cNackError, SensorBridgeDeviceI2cNackError) as e:
            return self.STATUS_NACK, e, b""
        except (SensorBridgeI2cTimeoutError, SensorBridgeDeviceI2cTimeoutError) as e:
            return self.STATUS_TIMEOUT, e, b""
        except Exception as e:
            return self.STATUS_UNSPECIFIED_ERROR, e, b""


__all__ = [
    "SensorBridgePort",
    "VOLTAGES",
    "I2C_FREQUENCIES",
    "SPI_FREQUENCIES",
    "port_to_byte",
    "voltage_to_byte",
    "i2c_frequency_to_byte",
    "spi_frequency_to_byte",
    "SensorBridgeDeviceError",
    "SensorBridgeFunctionalityNotImplementedError",
    "SensorBridgeNoMorePeriodicMeasurementsPossibleError",
    "SensorBridgeFrameChecksumError",
    "SensorBridgeFrameSizeInfoMismatchError",
    "SensorBridgeDeviceI2cNackError",
    "SensorBridgeDeviceI2cTimeoutError",
    "SensorBridgeFatalError",
    "SensorBridgeI2cError",
    "SensorBridgeI2cNackError",
    "SensorBridgeI2cTimeoutError",
    "SensorBridgeI2cTimingError",
    "i2c_error_from_code",
    "RepeatedTransceiveHandle",
    "BufferedValue",
    "ReadBufferResponse",
    "SensorBridgeFirmwareImage",
    "SensorBridgeShdlcDevice",
    "AsyncSensorBridgeShdlcDevice",
    "SensorBridgeI2cProxy",
    "AsyncSensorBridgeI2cProxy",
    "SensorBridgeCmdSetPortVoltage",
    "SensorBridgeCmdPortVoltageOnOff",
    "SensorBridgeCmdSetI2cFrequency",
    "SensorBridgeCmdI2cScan",
    "SensorBridgeCmdFirstTransceive",
    "SensorBridgeCmdSubsequentTransceive",
    "SensorBridgeCmdI2cRepeatedTransceive",
    "SensorBridgeCmdReadBuffer",
    "SensorBridgeCmdStopRepeatedTransceive",
    "SensorBridgeCmdSpiConfig",
    "SensorBridgeCmdSpiTransceive",
    "SensorBridgeCmdBlink",
    "SensorBridgeCmdAnalogMeasurement",
    "__version__",
]
