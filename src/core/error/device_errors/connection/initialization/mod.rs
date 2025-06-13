//! Device initialization error handling utilities
//!
//! This module provides specialized error handling for device initialization-related
//! errors in the Lumidox II Controller. It handles various initialization scenarios
//! including port setup, device handshake, and configuration validation.

use crate::core::error::types::LumidoxError;

/// Initialization error categories for better error classification
#[derive(Debug, Clone, PartialEq)]
pub enum InitializationErrorCategory {
    /// Port setup errors
    PortSetup,
    /// Device handshake errors
    Handshake,
    /// Configuration validation errors
    Configuration,
    /// Firmware compatibility errors
    Firmware,
}

/// Initialization error utilities and helper functions
pub struct InitializationErrorUtils;

impl InitializationErrorUtils {
    /// Create a device initialization error
    /// 
    /// Used when the device fails to initialize properly.
    /// 
    /// # Arguments
    /// * `stage` - The initialization stage that failed
    /// * `details` - Specific details about the failure
    /// 
    /// # Returns
    /// * `LumidoxError::DeviceError` - Formatted device initialization error
    pub fn initialization_error(stage: &str, details: &str) -> LumidoxError {
        LumidoxError::DeviceError(format!(
            "Device initialization failed at {}: {}", 
            stage, details
        ))
    }
    
    /// Create a port setup error
    pub fn port_setup_error(port: &str, issue: &str) -> LumidoxError {
        LumidoxError::DeviceError(format!(
            "Port setup failed for {}: {}", 
            port, issue
        ))
    }
    
    /// Create a handshake error
    pub fn handshake_error(details: &str) -> LumidoxError {
        LumidoxError::DeviceError(format!(
            "Device handshake failed: {}", 
            details
        ))
    }
    
    /// Categorize an initialization error
    pub fn categorize_error(error_message: &str) -> InitializationErrorCategory {
        let message_lower = error_message.to_lowercase();
        
        if message_lower.contains("port") || message_lower.contains("setup") {
            InitializationErrorCategory::PortSetup
        } else if message_lower.contains("handshake") {
            InitializationErrorCategory::Handshake
        } else if message_lower.contains("firmware") {
            InitializationErrorCategory::Firmware
        } else {
            InitializationErrorCategory::Configuration
        }
    }
}
