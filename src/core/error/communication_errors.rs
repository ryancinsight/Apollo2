//! Communication and protocol error handling utilities
//!
//! This module provides specialized error handling utilities and documentation
//! for communication, protocol, and serial port related errors in the Lumidox II Controller.
//!
//! ## Module Structure (8+ levels deep)
//! ```
//! src/core/error/communication_errors/      (Level 4)
//! ├── serial_port/                          (Level 5)
//! │   ├── configuration/                    (Level 6)
//! │   │   ├── baud_rate/                    (Level 7)
//! │   │   │   └── mod.rs                    (Level 8) - Baud rate config errors
//! │   │   └── parameters/                   (Level 7)
//! │   │       └── mod.rs                    (Level 8) - Port parameter errors
//! │   ├── connection/                       (Level 6)
//! │   │   └── mod.rs                        (Level 7) - Port connection errors
//! │   └── mod.rs                            (Level 6) - Serial port coordination
//! ├── protocol/                             (Level 5)
//! │   ├── command/                          (Level 6)
//! │   │   ├── validation/                   (Level 7)
//! │   │   │   └── mod.rs                    (Level 8) - Command validation
//! │   │   ├── execution/                    (Level 7)
//! │   │   │   └── mod.rs                    (Level 8) - Command execution
//! │   │   └── mod.rs                        (Level 7) - Command coordination
//! │   ├── response/                         (Level 6)
//! │   │   ├── parsing/                      (Level 7)
//! │   │   │   └── mod.rs                    (Level 8) - Response parsing
//! │   │   └── validation/                   (Level 7)
//! │   │       └── mod.rs                    (Level 8) - Response validation
//! │   └── version/                          (Level 6)
//! │       └── compatibility/                (Level 7)
//! │           └── mod.rs                    (Level 8) - Version compatibility
//! ├── data_format/                          (Level 5)
//! │   ├── parsing/                          (Level 6)
//! │   │   └── numeric/                      (Level 7)
//! │   │       └── mod.rs                    (Level 8) - Numeric parsing errors
//! │   └── validation/                       (Level 6)
//! │       └── format/                       (Level 7)
//! │           └── mod.rs                    (Level 8) - Format validation
//! ├── timeout/                              (Level 5)
//! │   ├── operation/                        (Level 6)
//! │   │   └── mod.rs                        (Level 7) - Operation timeouts
//! │   └── response/                         (Level 6)
//! │       └── mod.rs                        (Level 7) - Response timeouts
//! └── integrity/                            (Level 5)
//!     ├── checksum/                         (Level 6)
//!     │   └── validation/                   (Level 7)
//!     │       └── mod.rs                    (Level 8) - Checksum validation
//!     └── corruption/                       (Level 6)
//!         └── detection/                    (Level 7)
//!             └── mod.rs                    (Level 8) - Data corruption detection
//! ```
//!
//! Each sub-module follows the prescribed schema with single responsibility
//! and maintains <150 lines per file.

// Import specialized sub-modules
pub mod serial_port;
pub mod protocol;
pub mod data_format;
pub mod timeout;
pub mod integrity;

// Re-export commonly used items for convenience
// Note: Utilities are available but not currently used in the codebase
// pub use serial_port::{SerialPortErrorUtils, SerialPortErrorCategory};
// pub use protocol::{ProtocolErrorUtils, ProtocolErrorCategory};
// pub use data_format::{DataFormatErrorUtils, DataFormatErrorCategory};
// pub use timeout::{TimeoutErrorUtils, TimeoutErrorCategory};
// pub use integrity::{IntegrityErrorUtils, IntegrityErrorCategory};

use super::types::LumidoxError;

/// Communication error categories for better error classification
/// 
/// This enum helps categorize different types of communication errors
/// for more specific error handling and user feedback.
#[derive(Debug, Clone, PartialEq)]
pub enum CommunicationErrorCategory {
    /// Serial port connection or configuration errors
    SerialPort,
    /// Protocol command or response errors
    Protocol,
    /// Data parsing or format errors
    DataFormat,
    /// Communication timeout errors
    Timeout,
    /// Checksum or data integrity errors
    DataIntegrity,
}

/// Communication error utilities and helper functions
pub struct CommunicationErrorUtils;

impl CommunicationErrorUtils {
    /// Create a serial port configuration error
    /// 
    /// Used when serial port cannot be configured or opened.
    /// 
    /// # Arguments
    /// * `port_name` - The name of the serial port
    /// * `details` - Specific details about the configuration failure
    /// 
    /// # Returns
    /// * `LumidoxError::ProtocolError` - Formatted serial port error
    /// 
    /// # Example
    /// ```
    /// let error = CommunicationErrorUtils::serial_port_error("COM3", "Port not found");
    /// ```
    pub fn serial_port_error(port_name: &str, details: &str) -> LumidoxError {
        LumidoxError::ProtocolError(format!("Serial port '{}' error: {}", port_name, details))
    }
    
    /// Create a protocol command error
    /// 
    /// Used when a protocol command fails or returns unexpected results.
    /// 
    /// # Arguments
    /// * `command` - The protocol command that failed (as hex string)
    /// * `expected` - What was expected from the command
    /// * `received` - What was actually received
    /// 
    /// # Returns
    /// * `LumidoxError::ProtocolError` - Formatted protocol command error
    /// 
    /// # Example
    /// ```
    /// let error = CommunicationErrorUtils::protocol_command_error("0x02", "firmware version", "timeout");
    /// ```
    pub fn protocol_command_error(command: &str, expected: &str, received: &str) -> LumidoxError {
        LumidoxError::ProtocolError(format!(
            "Protocol command {} failed: expected '{}', received '{}'", 
            command, expected, received
        ))
    }
    
    /// Create a data format error
    /// 
    /// Used when received data cannot be parsed or is in an unexpected format.
    /// 
    /// # Arguments
    /// * `data_type` - The type of data that failed to parse
    /// * `raw_data` - The raw data that couldn't be parsed
    /// * `reason` - The reason parsing failed
    /// 
    /// # Returns
    /// * `LumidoxError::ProtocolError` - Formatted data format error
    /// 
    /// # Example
    /// ```
    /// let error = CommunicationErrorUtils::data_format_error("current value", "0xFF", "invalid numeric format");
    /// ```
    pub fn data_format_error(data_type: &str, raw_data: &str, reason: &str) -> LumidoxError {
        LumidoxError::ProtocolError(format!(
            "Data format error for {}: '{}' - {}", 
            data_type, raw_data, reason
        ))
    }
    
    /// Create a communication timeout error
    /// 
    /// Used when communication operations exceed expected timeframes.
    /// 
    /// # Arguments
    /// * `operation` - The communication operation that timed out
    /// * `timeout_ms` - The timeout duration in milliseconds
    /// 
    /// # Returns
    /// * `LumidoxError::ProtocolError` - Formatted communication timeout error
    /// 
    /// # Example
    /// ```
    /// let error = CommunicationErrorUtils::communication_timeout_error("command response", 1000);
    /// ```
    pub fn communication_timeout_error(operation: &str, timeout_ms: u64) -> LumidoxError {
        LumidoxError::ProtocolError(format!(
            "Communication timeout: {} operation failed to complete within {}ms", 
            operation, timeout_ms
        ))
    }
    
    /// Create a data integrity error
    /// 
    /// Used when data corruption or checksum failures are detected.
    /// 
    /// # Arguments
    /// * `data_description` - Description of the corrupted data
    /// * `integrity_check` - The type of integrity check that failed
    /// 
    /// # Returns
    /// * `LumidoxError::ProtocolError` - Formatted data integrity error
    /// 
    /// # Example
    /// ```
    /// let error = CommunicationErrorUtils::data_integrity_error("device response", "checksum validation");
    /// ```
    pub fn data_integrity_error(data_description: &str, integrity_check: &str) -> LumidoxError {
        LumidoxError::ProtocolError(format!(
            "Data integrity error in {}: {} failed", 
            data_description, integrity_check
        ))
    }
    
    /// Create a protocol version mismatch error
    /// 
    /// Used when the device protocol version is incompatible.
    /// 
    /// # Arguments
    /// * `device_version` - The protocol version reported by the device
    /// * `supported_versions` - The protocol versions supported by the software
    /// 
    /// # Returns
    /// * `LumidoxError::ProtocolError` - Formatted protocol version error
    /// 
    /// # Example
    /// ```
    /// let error = CommunicationErrorUtils::protocol_version_error("1.0", "2.0+");
    /// ```
    pub fn protocol_version_error(device_version: &str, supported_versions: &str) -> LumidoxError {
        LumidoxError::ProtocolError(format!(
            "Protocol version mismatch: device uses version '{}', software supports '{}'", 
            device_version, supported_versions
        ))
    }
    
    /// Categorize a communication error for better handling
    /// 
    /// Analyzes a communication error message to determine its category.
    /// This can be used for implementing category-specific error handling.
    /// 
    /// # Arguments
    /// * `error_message` - The communication error message to categorize
    /// 
    /// # Returns
    /// * `CommunicationErrorCategory` - The determined error category
    /// 
    /// # Example
    /// ```
    /// let category = CommunicationErrorUtils::categorize_error("Serial port 'COM3' error: Port not found");
    /// assert_eq!(category, CommunicationErrorCategory::SerialPort);
    /// ```
    pub fn categorize_error(error_message: &str) -> CommunicationErrorCategory {
        let message_lower = error_message.to_lowercase();
        
        if message_lower.contains("serial") || message_lower.contains("port") || message_lower.contains("com") {
            CommunicationErrorCategory::SerialPort
        } else if message_lower.contains("protocol") || message_lower.contains("command") {
            CommunicationErrorCategory::Protocol
        } else if message_lower.contains("format") || message_lower.contains("parse") || message_lower.contains("invalid") {
            CommunicationErrorCategory::DataFormat
        } else if message_lower.contains("timeout") || message_lower.contains("timed out") {
            CommunicationErrorCategory::Timeout
        } else {
            CommunicationErrorCategory::DataIntegrity
        }
    }
}
