use rust_shdlc_driver::firmware::ShdlcFirmwareImage;
use rust_shdlc_driver::protocol::errors::ShdlcError;
use std::path::Path;

/// SEK-SensorBridge firmware image parser.
#[derive(Debug, Clone)]
pub struct SensorBridgeFirmwareImage {
    pub inner: ShdlcFirmwareImage,
}

impl SensorBridgeFirmwareImage {
    pub const BL_START_ADDR: u32 = 0x8000000;
    pub const APP_START_ADDR: u32 = 0x8004000;

    /// Loads and verifies a SensorBridge firmware image from Intel-Hex string.
    pub fn from_hex_str(hex_content: &str) -> Result<Self, ShdlcError> {
        let inner = ShdlcFirmwareImage::new(
            hex_content,
            Self::BL_START_ADDR,
            Self::APP_START_ADDR,
            None,
            None,
        )?;
        Ok(Self { inner })
    }

    /// Loads and verifies a SensorBridge firmware image from a hex file on disk.
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, ShdlcError> {
        let content = std::fs::read_to_string(path).map_err(|e| ShdlcError::Io(e.to_string()))?;
        Self::from_hex_str(&content)
    }

    pub fn product_type(&self) -> u32 {
        self.inner.product_type()
    }

    pub fn bootloader_version(&self) -> rust_shdlc_driver::protocol::types::FirmwareVersion {
        self.inner.bootloader_version()
    }

    pub fn application_version(&self) -> rust_shdlc_driver::protocol::types::FirmwareVersion {
        self.inner.application_version()
    }

    pub fn checksum(&self) -> u8 {
        self.inner.checksum()
    }

    pub fn size(&self) -> usize {
        self.inner.size()
    }
}
