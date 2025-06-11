//! Device information model definitions
//!
//! This module contains types and structures related to device identification
//! and configuration information including firmware version, model number,
//! serial number, and wavelength specifications.

/// Device information structure
/// 
/// Contains comprehensive identification and configuration information
/// about the Lumidox II device. This information is typically read once
/// during device initialization and used for display and logging purposes.
/// 
/// All string fields are read from the device using specific protocol
/// commands and represent the actual hardware configuration.
#[derive(Debug, Clone)]
pub struct DeviceInfo {
    /// Firmware version string from the device
    /// 
    /// Read using command 0x02 (FIRMWARE_VERSION). This indicates
    /// the version of firmware running on the device.
    pub firmware_version: String,
    
    /// Device model number string
    /// 
    /// Read using commands 0x6c through 0x73 (MODEL_COMMANDS).
    /// This identifies the specific model of the device.
    pub model_number: String,
    
    /// Device serial number string
    /// 
    /// Read using commands 0x60 through 0x6b (SERIAL_COMMANDS).
    /// This provides unique identification for the device.
    pub serial_number: String,
    
    /// Device wavelength specification string
    /// 
    /// Read using commands 0x76, 0x81, 0x82, 0x89, 0x8a (WAVELENGTH_COMMANDS).
    /// This indicates the optical wavelength characteristics of the device.
    pub wavelength: String,
    
    /// Maximum current capability in milliamps
    /// 
    /// This represents the maximum safe operating current for the device.
    /// Used for validation and safety checks during operation.
    pub max_current_ma: u16,
}
