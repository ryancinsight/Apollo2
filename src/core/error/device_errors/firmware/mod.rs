//! Device firmware error handling utilities
//!
//! This module provides specialized error handling for device firmware-related
//! errors in the Lumidox II Controller. It handles various firmware scenarios
//! including compatibility checks, version validation, and update failures.

use crate::core::error::types::LumidoxError;

/// Firmware error categories for better error classification
#[derive(Debug, Clone, PartialEq)]
pub enum FirmwareErrorCategory {
    /// Firmware compatibility errors
    Compatibility,
    /// Firmware version errors
    Version,
    /// Firmware update errors
    Update,
    /// Firmware validation errors
    Validation,
}

/// Firmware error utilities and helper functions
pub struct FirmwareErrorUtils;

impl FirmwareErrorUtils {
    /// Create a firmware compatibility error
    /// 
    /// Used when the device firmware is incompatible with the controller.
    /// 
    /// # Arguments
    /// * `device_version` - The device firmware version
    /// * `required_version` - The required firmware version
    /// 
    /// # Returns
    /// * `LumidoxError::DeviceError` - Formatted firmware compatibility error
    pub fn compatibility_error(device_version: &str, required_version: &str) -> LumidoxError {
        LumidoxError::DeviceError(format!(
            "Firmware compatibility error: device version '{}' is incompatible with required version '{}'", 
            device_version, required_version
        ))
    }
    
    /// Create a firmware version error
    pub fn version_error(version_issue: &str) -> LumidoxError {
        LumidoxError::DeviceError(format!(
            "Firmware version error: {}", 
            version_issue
        ))
    }
    
    /// Create a firmware update error
    pub fn update_error(update_stage: &str, details: &str) -> LumidoxError {
        LumidoxError::DeviceError(format!(
            "Firmware update failed at {}: {}", 
            update_stage, details
        ))
    }
    
    /// Categorize a firmware error
    pub fn categorize_error(error_message: &str) -> FirmwareErrorCategory {
        let message_lower = error_message.to_lowercase();
        
        if message_lower.contains("compatibility") || message_lower.contains("incompatible") {
            FirmwareErrorCategory::Compatibility
        } else if message_lower.contains("version") {
            FirmwareErrorCategory::Version
        } else if message_lower.contains("update") {
            FirmwareErrorCategory::Update
        } else {
            FirmwareErrorCategory::Validation
        }
    }
}
