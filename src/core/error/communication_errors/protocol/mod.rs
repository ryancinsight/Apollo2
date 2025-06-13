//! Protocol error handling utilities
//!
//! This module provides specialized error handling for protocol-related
//! errors in the Lumidox II Controller. It handles various protocol scenarios
//! including command validation, execution, response parsing, and version compatibility.

use crate::core::error::types::LumidoxError;

/// Protocol error categories for better error classification
#[derive(Debug, Clone, PartialEq)]
pub enum ProtocolErrorCategory {
    /// Command validation errors
    Command,
    /// Response parsing errors
    Response,
    /// Version compatibility errors
    Version,
    /// Protocol format errors
    Format,
}

/// Protocol error utilities and helper functions
pub struct ProtocolErrorUtils;

impl ProtocolErrorUtils {
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
    pub fn command_error(command: &str, expected: &str, received: &str) -> LumidoxError {
        LumidoxError::ProtocolError(format!(
            "Protocol command {} failed: expected '{}', received '{}'", 
            command, expected, received
        ))
    }
    
    /// Create a protocol response error
    pub fn response_error(command: &str, response_issue: &str) -> LumidoxError {
        LumidoxError::ProtocolError(format!(
            "Protocol response error for command {}: {}", 
            command, response_issue
        ))
    }
    
    /// Create a protocol version error
    pub fn version_error(device_version: &str, supported_versions: &str) -> LumidoxError {
        LumidoxError::ProtocolError(format!(
            "Protocol version mismatch: device uses version '{}', software supports '{}'", 
            device_version, supported_versions
        ))
    }
    
    /// Create a protocol format error
    pub fn format_error(format_issue: &str, data: &str) -> LumidoxError {
        LumidoxError::ProtocolError(format!(
            "Protocol format error: {} in data '{}'", 
            format_issue, data
        ))
    }
    
    /// Categorize a protocol error
    pub fn categorize_error(error_message: &str) -> ProtocolErrorCategory {
        let message_lower = error_message.to_lowercase();
        
        if message_lower.contains("command") {
            ProtocolErrorCategory::Command
        } else if message_lower.contains("response") {
            ProtocolErrorCategory::Response
        } else if message_lower.contains("version") {
            ProtocolErrorCategory::Version
        } else {
            ProtocolErrorCategory::Format
        }
    }
}
