//! Device-specific error handling utilities
//!
//! This module provides specialized error handling utilities and documentation
//! for device operation and hardware-related errors in the Lumidox II Controller.
//!
//! ## Module Structure (8+ levels deep)
//! ```
//! src/core/error/device_errors/             (Level 4)
//! ├── connection/                           (Level 5)
//! │   ├── timeout/                          (Level 6)
//! │   │   ├── network/                      (Level 7)
//! │   │   │   └── mod.rs                    (Level 8) - Network timeout errors
//! │   │   └── serial/                       (Level 7)
//! │   │       └── mod.rs                    (Level 8) - Serial timeout errors
//! │   ├── initialization/                   (Level 6)
//! │   │   └── mod.rs                        (Level 7) - Device init errors
//! │   └── mod.rs                            (Level 6) - Connection error coordination
//! ├── state/                                (Level 5)
//! │   ├── transitions/                      (Level 6)
//! │   │   └── mod.rs                        (Level 7) - State transition errors
//! │   └── validation/                       (Level 6)
//! │       └── mod.rs                        (Level 7) - State validation errors
//! ├── firmware/                             (Level 5)
//! │   ├── compatibility/                    (Level 6)
//! │   │   └── mod.rs                        (Level 7) - Version compatibility
//! │   └── validation/                       (Level 6)
//! │       └── mod.rs                        (Level 7) - Firmware validation
//! └── hardware/                             (Level 5)
//!     ├── malfunction/                      (Level 6)
//!     │   └── mod.rs                        (Level 7) - Hardware malfunction
//!     └── diagnostics/                      (Level 6)
//!         └── mod.rs                        (Level 7) - Hardware diagnostics
//! ```
//!
//! Each sub-module follows the prescribed schema with single responsibility
//! and maintains <150 lines per file.

// Import specialized sub-modules
pub mod connection;
pub mod state;
pub mod firmware;
pub mod hardware;

// Re-export commonly used items for convenience
// Note: Utilities are available but not currently used in the codebase
// pub use connection::{ConnectionErrorUtils, ConnectionErrorCategory};
// pub use state::{StateErrorUtils, StateErrorCategory};
// pub use firmware::{FirmwareErrorUtils, FirmwareErrorCategory};
// pub use hardware::{HardwareErrorUtils, HardwareErrorCategory};

use super::types::LumidoxError;

/// Device error categories for better error classification
/// 
/// This enum helps categorize different types of device errors
/// for more specific error handling and user feedback.
#[derive(Debug, Clone, PartialEq)]
pub enum DeviceErrorCategory {
    /// Device connection or initialization errors
    Connection,
    /// Device communication timeout errors
    Timeout,
    /// Device state or mode errors
    State,
    /// Device firmware or compatibility errors
    Firmware,
    /// Device hardware malfunction errors
    Hardware,
}

/// Device error utilities and helper functions
pub struct DeviceErrorUtils;

impl DeviceErrorUtils {
    /// Create a device connection error
    /// 
    /// Used when the device cannot be connected to or initialized.
    /// 
    /// # Arguments
    /// * `details` - Specific details about the connection failure
    /// 
    /// # Returns
    /// * `LumidoxError::DeviceError` - Formatted device connection error
    /// 
    /// # Example
    /// ```
    /// let error = DeviceErrorUtils::connection_error("Failed to open serial port COM3");
    /// ```
    pub fn connection_error(details: &str) -> LumidoxError {
        LumidoxError::DeviceError(format!("Device connection failed: {}", details))
    }
    
    /// Create a device timeout error
    /// 
    /// Used when the device fails to respond within expected timeframes.
    /// 
    /// # Arguments
    /// * `operation` - The operation that timed out
    /// * `timeout_ms` - The timeout duration in milliseconds
    /// 
    /// # Returns
    /// * `LumidoxError::DeviceError` - Formatted device timeout error
    /// 
    /// # Example
    /// ```
    /// let error = DeviceErrorUtils::timeout_error("firmware version read", 5000);
    /// ```
    pub fn timeout_error(operation: &str, timeout_ms: u64) -> LumidoxError {
        LumidoxError::DeviceError(format!(
            "Device timeout: {} operation failed to complete within {}ms", 
            operation, timeout_ms
        ))
    }
    
    /// Create a device state error
    /// 
    /// Used when the device is in an unexpected or invalid state.
    /// 
    /// # Arguments
    /// * `expected_state` - The expected device state
    /// * `actual_state` - The actual device state encountered
    /// 
    /// # Returns
    /// * `LumidoxError::DeviceError` - Formatted device state error
    /// 
    /// # Example
    /// ```
    /// let error = DeviceErrorUtils::state_error("Armed", "Local");
    /// ```
    pub fn state_error(expected_state: &str, actual_state: &str) -> LumidoxError {
        LumidoxError::DeviceError(format!(
            "Device state error: expected '{}', found '{}'", 
            expected_state, actual_state
        ))
    }
    
    /// Create a device firmware compatibility error
    /// 
    /// Used when the device firmware is incompatible or unsupported.
    /// 
    /// # Arguments
    /// * `firmware_version` - The detected firmware version
    /// * `required_version` - The required or supported firmware version
    /// 
    /// # Returns
    /// * `LumidoxError::DeviceError` - Formatted firmware compatibility error
    /// 
    /// # Example
    /// ```
    /// let error = DeviceErrorUtils::firmware_error("1.0.0", ">=1.2.0");
    /// ```
    pub fn firmware_error(firmware_version: &str, required_version: &str) -> LumidoxError {
        LumidoxError::DeviceError(format!(
            "Firmware compatibility error: device has version '{}', requires '{}'", 
            firmware_version, required_version
        ))
    }
    
    /// Create a device hardware malfunction error
    /// 
    /// Used when the device hardware appears to be malfunctioning.
    /// 
    /// # Arguments
    /// * `component` - The hardware component that is malfunctioning
    /// * `symptoms` - Description of the malfunction symptoms
    /// 
    /// # Returns
    /// * `LumidoxError::DeviceError` - Formatted hardware malfunction error
    /// 
    /// # Example
    /// ```
    /// let error = DeviceErrorUtils::hardware_error("LED driver", "inconsistent current readings");
    /// ```
    pub fn hardware_error(component: &str, symptoms: &str) -> LumidoxError {
        LumidoxError::DeviceError(format!(
            "Hardware malfunction in {}: {}", 
            component, symptoms
        ))
    }
    
    /// Categorize a device error for better handling
    /// 
    /// Analyzes a device error message to determine its category.
    /// This can be used for implementing category-specific error handling.
    /// 
    /// # Arguments
    /// * `error_message` - The device error message to categorize
    /// 
    /// # Returns
    /// * `DeviceErrorCategory` - The determined error category
    /// 
    /// # Example
    /// ```
    /// let category = DeviceErrorUtils::categorize_error("Device connection failed: port not found");
    /// assert_eq!(category, DeviceErrorCategory::Connection);
    /// ```
    pub fn categorize_error(error_message: &str) -> DeviceErrorCategory {
        let message_lower = error_message.to_lowercase();
        
        if message_lower.contains("connection") || message_lower.contains("port") || message_lower.contains("connect") {
            DeviceErrorCategory::Connection
        } else if message_lower.contains("timeout") || message_lower.contains("timed out") {
            DeviceErrorCategory::Timeout
        } else if message_lower.contains("state") || message_lower.contains("mode") {
            DeviceErrorCategory::State
        } else if message_lower.contains("firmware") || message_lower.contains("version") {
            DeviceErrorCategory::Firmware
        } else {
            DeviceErrorCategory::Hardware
        }
    }
}
