.. sensirion-uart-sensorbridge documentation master file

Sensirion SEK-SensorBridge Driver Documentation
===============================================

`sensirion-uart-sensorbridge` is a high-performance synchronous and asynchronous SEK-SensorBridge driver implemented in Rust with native PyO3 and Maturin bindings for Python.

.. toctree::
   :maxdepth: 2
   :caption: Contents:

   quickstart
   architecture
   api
   development

Features
--------

- **Sync and Async**: Native synchronous blocking API and Tokio/asyncio asynchronous APIs in both Rust and Python.
- **Layered Clean Architecture**: Strict separation of Transport, Connection, Protocol, and Application layers.
- **I²C and SPI**: Support for single-frame, chunked multi-frame, and repeated periodic measurements with local buffer management.
- **Type-safe Definitions**: Validated port constants, voltages (1.2V - 5.5V), I2C frequencies (10kHz - 2MHz), and SPI modes/frequencies.
- **Firmware Updates**: Full firmware update engine supporting Intel-Hex image parsing and bootloader flashing.
- **Built-in Mocks**: Decoupled transports allowing unit testing and CI without connected physical hardware.
