//! Data format error handling utilities
//!
//! This module provides specialized error handling for data format-related
//! errors in the Lumidox II Controller. It handles various data format scenarios
//! including parsing errors, validation failures, and format mismatches.

use crate::core::error::types::LumidoxError;

/// Data format error categories for better error classification
#[derive(Debug, Clone, PartialEq)]
pub enum DataFormatErrorCategory {
    /// Parsing errors
    Parsing,
    /// Validation errors
    Validation,
    /// Format mismatch errors
    Format,
    /// Encoding errors
    Encoding,
}

/// Data format error utilities and helper functions
pub struct DataFormatErrorUtils;

impl DataFormatErrorUtils {
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
    pub fn parsing_error(data_type: &str, raw_data: &str, reason: &str) -> LumidoxError {
        LumidoxError::ProtocolError(format!(
            "Data parsing error for {}: '{}' - {}", 
            data_type, raw_data, reason
        ))
    }
    
    /// Create a data validation error
    pub fn validation_error(data_type: &str, value: &str, constraint: &str) -> LumidoxError {
        LumidoxError::ProtocolError(format!(
            "Data validation error for {}: value '{}' violates constraint '{}'", 
            data_type, value, constraint
        ))
    }
    
    /// Create a format mismatch error
    pub fn format_error(expected_format: &str, actual_format: &str, data: &str) -> LumidoxError {
        LumidoxError::ProtocolError(format!(
            "Format mismatch: expected '{}', found '{}' in data '{}'", 
            expected_format, actual_format, data
        ))
    }
    
    /// Create an encoding error
    pub fn encoding_error(encoding_type: &str, data: &str, issue: &str) -> LumidoxError {
        LumidoxError::ProtocolError(format!(
            "Encoding error ({}): {} in data '{}'", 
            encoding_type, issue, data
        ))
    }
    
    /// Categorize a data format error
    pub fn categorize_error(error_message: &str) -> DataFormatErrorCategory {
        let message_lower = error_message.to_lowercase();
        
        if message_lower.contains("parsing") || message_lower.contains("parse") {
            DataFormatErrorCategory::Parsing
        } else if message_lower.contains("validation") || message_lower.contains("constraint") {
            DataFormatErrorCategory::Validation
        } else if message_lower.contains("format") || message_lower.contains("mismatch") {
            DataFormatErrorCategory::Format
        } else {
            DataFormatErrorCategory::Encoding
        }
    }
}
