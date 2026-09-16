Architecture
============

The driver is designed following a clean, decoupled 4-layer architecture:

.. code-block:: text

   +---------------------------------------------------------------+
   |                      Application Layer                        |
   |   SensorBridgeDevice / AsyncSensorBridgeDevice                |
   |   SensorBridgeI2cProxy / AsyncSensorBridgeI2cProxy            |
   |   SensorBridgeFirmwareImage                                   |
   +---------------------------------------------------------------+
                                  |
   +---------------------------------------------------------------+
   |                       Protocol Layer                          |
   |   SensorBridge Commands (PortVoltage, I2c, Spi, Blink, etc.)  |
   |   Definitions & Enums (Port, Voltage, Frequency, SpiMode)     |
   |   Types (RepeatedTransceiveHandle, BufferedValue, etc.)       |
   |   Errors & Exception Mapping                                  |
   +---------------------------------------------------------------+
                                  |
   +---------------------------------------------------------------+
   |                      Connection Layer                         |
   |   ShdlcConnection / AsyncShdlcConnection                      |
   +---------------------------------------------------------------+
                                  |
   +---------------------------------------------------------------+
   |                      Transport Layer                          |
   |   ShdlcSerialPort (Serial / RS485)                            |
   |   ShdlcTcpPort (TCP / Ethernet)                               |
   |   ShdlcMockPort (Mock in-memory for testing & CI)             |
   +---------------------------------------------------------------+

Key Components
--------------

- **Application Layer**: Provides complete device operations including multi-frame chunked I2C transceives, repeated background measurements, buffer readout, and I2C proxy interfaces.
- **Protocol Layer**: Encapsulates command framing, payload serialization/deserialization, and validation.
- **Connection Layer**: Manages request-response transactions, sequence matching, and post-processing delay timing.
- **Transport Layer**: Abstracts physical I/O over async streams, supporting mock loopbacks for CI testing without physical hardware.
