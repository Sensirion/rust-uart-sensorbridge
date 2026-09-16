API Reference
=============

Device Classes
--------------

.. autoclass:: sensirion_uart_sensorbridge.SensorBridgeShdlcDevice
   :members:
   :undoc-members:
   :show-inheritance:

.. autoclass:: sensirion_uart_sensorbridge.AsyncSensorBridgeShdlcDevice
   :members:
   :undoc-members:
   :show-inheritance:

I2C Proxy Classes
-----------------

.. autoclass:: sensirion_uart_sensorbridge.SensorBridgeI2cProxy
   :members:
   :undoc-members:
   :show-inheritance:

.. autoclass:: sensirion_uart_sensorbridge.AsyncSensorBridgeI2cProxy
   :members:
   :undoc-members:
   :show-inheritance:

Definitions & Constants
-----------------------

.. autoclass:: sensirion_uart_sensorbridge.SensorBridgePort
   :members:
   :undoc-members:

.. py:data:: sensirion_uart_sensorbridge.VOLTAGES
   :type: dict

   Mapping of supported voltage values in Volts to SHDLC byte representation.

.. py:data:: sensirion_uart_sensorbridge.I2C_FREQUENCIES
   :type: dict

   Mapping of supported I2C frequencies in Hz to SHDLC byte representation.

.. py:data:: sensirion_uart_sensorbridge.SPI_FREQUENCIES
   :type: dict

   Mapping of supported SPI frequencies in Hz to SHDLC byte representation.

Types
-----

.. autoclass:: sensirion_uart_sensorbridge.RepeatedTransceiveHandle
   :members:
   :undoc-members:

.. autoclass:: sensirion_uart_sensorbridge.BufferedValue
   :members:
   :undoc-members:

.. autoclass:: sensirion_uart_sensorbridge.ReadBufferResponse
   :members:
   :undoc-members:

Firmware Image
--------------

.. autoclass:: sensirion_uart_sensorbridge.SensorBridgeFirmwareImage
   :members:
   :undoc-members:
