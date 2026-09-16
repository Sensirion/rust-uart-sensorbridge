pub mod async_device;
pub mod i2c_proxy;
pub mod sync_device;

pub use async_device::AsyncSensorBridgeDevice;
pub use i2c_proxy::{
    AsyncSensorBridgeI2cProxy, I2cProxyResult, SensorBridgeI2cProxy, I2C_STATUS_CHANNEL_DISABLED,
    I2C_STATUS_NACK, I2C_STATUS_OK, I2C_STATUS_TIMEOUT, I2C_STATUS_UNSPECIFIED_ERROR,
};
pub use sync_device::SensorBridgeDevice;
