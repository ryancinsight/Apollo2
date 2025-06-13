//! Communication timeout error handling utilities
//!
//! This module provides specialized error handling for communication timeout-related
//! errors in the Lumidox II Controller. It handles various timeout scenarios
//! including operation timeouts, response timeouts, and connection timeouts.

use crate::core::error::types::LumidoxError;

/// Timeout error categories for better error classification
#[derive(Debug, Clone, PartialEq)]
pub enum TimeoutErrorCategory {
    /// Operation timeout errors
    Operation,
    /// Response timeout errors
    Response,
    /// Connection timeout errors
    Connection,
    /// Read timeout errors
    Read,
}

/// Timeout error utilities and helper functions
pub struct TimeoutErrorUtils;

impl TimeoutErrorUtils {
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
    pub fn operation_timeout_error(operation: &str, timeout_ms: u64) -> LumidoxError {
        LumidoxError::ProtocolError(format!(
            "Communication timeout: {} operation failed to complete within {}ms", 
            operation, timeout_ms
        ))
    }
    
    /// Create a response timeout error
    pub fn response_timeout_error(command: &str, timeout_ms: u64) -> LumidoxError {
        LumidoxError::ProtocolError(format!(
            "Response timeout: command {} failed to receive response within {}ms", 
            command, timeout_ms
        ))
    }
    
    /// Create a connection timeout error
    pub fn connection_timeout_error(target: &str, timeout_ms: u64) -> LumidoxError {
        LumidoxError::ProtocolError(format!(
            "Connection timeout: failed to connect to {} within {}ms", 
            target, timeout_ms
        ))
    }
    
    /// Create a read timeout error
    pub fn read_timeout_error(source: &str, timeout_ms: u64) -> LumidoxError {
        LumidoxError::ProtocolError(format!(
            "Read timeout: failed to read from {} within {}ms", 
            source, timeout_ms
        ))
    }
    
    /// Categorize a timeout error
    pub fn categorize_error(error_message: &str) -> TimeoutErrorCategory {
        let message_lower = error_message.to_lowercase();
        
        if message_lower.contains("operation") {
            TimeoutErrorCategory::Operation
        } else if message_lower.contains("response") {
            TimeoutErrorCategory::Response
        } else if message_lower.contains("connection") {
            TimeoutErrorCategory::Connection
        } else {
            TimeoutErrorCategory::Read
        }
    }
}
