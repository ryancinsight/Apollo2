//! Connection timeout error handling utilities
//!
//! This module provides specialized error handling for connection timeout-related
//! errors in the Lumidox II Controller. It handles various timeout scenarios
//! including network timeouts, serial timeouts, and operation timeouts.

// Import specialized sub-modules (to be implemented)
// pub mod network;
// pub mod serial;

use crate::core::error::types::LumidoxError;

/// Timeout error categories for better error classification
#[derive(Debug, Clone, PartialEq)]
pub enum TimeoutErrorCategory {
    /// Network connection timeout
    Network,
    /// Serial port timeout
    Serial,
    /// Operation timeout
    Operation,
    /// Response timeout
    Response,
}

/// Timeout error utilities and helper functions
pub struct TimeoutErrorUtils;

impl TimeoutErrorUtils {
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
    pub fn timeout_error(operation: &str, timeout_ms: u64) -> LumidoxError {
        LumidoxError::DeviceError(format!(
            "Device timeout: {} operation failed to complete within {}ms", 
            operation, timeout_ms
        ))
    }
    
    /// Create a network timeout error
    pub fn network_timeout_error(address: &str, timeout_ms: u64) -> LumidoxError {
        LumidoxError::DeviceError(format!(
            "Network timeout: connection to {} failed within {}ms", 
            address, timeout_ms
        ))
    }
    
    /// Create a serial timeout error
    pub fn serial_timeout_error(port: &str, timeout_ms: u64) -> LumidoxError {
        LumidoxError::DeviceError(format!(
            "Serial timeout: port {} failed to respond within {}ms", 
            port, timeout_ms
        ))
    }
    
    /// Categorize a timeout error
    pub fn categorize_error(error_message: &str) -> TimeoutErrorCategory {
        let message_lower = error_message.to_lowercase();
        
        if message_lower.contains("network") {
            TimeoutErrorCategory::Network
        } else if message_lower.contains("serial") {
            TimeoutErrorCategory::Serial
        } else if message_lower.contains("response") {
            TimeoutErrorCategory::Response
        } else {
            TimeoutErrorCategory::Operation
        }
    }
}
