//! Serial port error handling utilities
//!
//! This module provides specialized error handling for serial port-related
//! errors in the Lumidox II Controller. It handles various serial port scenarios
//! including configuration, connection, and parameter validation.

// Import specialized sub-modules (to be implemented)
// pub mod configuration;
// pub mod connection;

use crate::core::error::types::LumidoxError;

/// Serial port error categories for better error classification
#[derive(Debug, Clone, PartialEq)]
pub enum SerialPortErrorCategory {
    /// Port configuration errors
    Configuration,
    /// Port connection errors
    Connection,
    /// Port parameter errors
    Parameters,
    /// Port availability errors
    Availability,
}

/// Serial port error utilities and helper functions
pub struct SerialPortErrorUtils;

impl SerialPortErrorUtils {
    /// Create a serial port configuration error
    /// 
    /// Used when serial port cannot be configured properly.
    /// 
    /// # Arguments
    /// * `port_name` - The name of the serial port
    /// * `details` - Specific details about the configuration failure
    /// 
    /// # Returns
    /// * `LumidoxError::ProtocolError` - Formatted serial port error
    pub fn configuration_error(port_name: &str, details: &str) -> LumidoxError {
        LumidoxError::ProtocolError(format!("Serial port '{}' configuration error: {}", port_name, details))
    }
    
    /// Create a serial port connection error
    pub fn connection_error(port_name: &str, reason: &str) -> LumidoxError {
        LumidoxError::ProtocolError(format!("Serial port '{}' connection failed: {}", port_name, reason))
    }
    
    /// Create a serial port parameter error
    pub fn parameter_error(port_name: &str, parameter: &str, value: &str) -> LumidoxError {
        LumidoxError::ProtocolError(format!(
            "Serial port '{}' parameter error: {} = '{}'", 
            port_name, parameter, value
        ))
    }
    
    /// Create a serial port availability error
    pub fn availability_error(port_name: &str) -> LumidoxError {
        LumidoxError::ProtocolError(format!("Serial port '{}' is not available", port_name))
    }
    
    /// Categorize a serial port error
    pub fn categorize_error(error_message: &str) -> SerialPortErrorCategory {
        let message_lower = error_message.to_lowercase();
        
        if message_lower.contains("configuration") || message_lower.contains("config") {
            SerialPortErrorCategory::Configuration
        } else if message_lower.contains("connection") || message_lower.contains("connect") {
            SerialPortErrorCategory::Connection
        } else if message_lower.contains("parameter") || message_lower.contains("baud") {
            SerialPortErrorCategory::Parameters
        } else {
            SerialPortErrorCategory::Availability
        }
    }
}
