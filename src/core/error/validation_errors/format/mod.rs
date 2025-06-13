//! Format validation error handling utilities
//!
//! This module provides specialized error handling for format validation-related
//! errors in the Lumidox II Controller. It handles various format validation scenarios
//! including type validation, structure validation, and format mismatches.

use crate::core::error::types::LumidoxError;

/// Format error categories for better error classification
#[derive(Debug, Clone, PartialEq)]
pub enum FormatErrorCategory {
    /// Type validation errors
    Type,
    /// Structure validation errors
    Structure,
    /// Format mismatch errors
    Mismatch,
    /// Encoding errors
    Encoding,
}

/// Format error utilities and helper functions
pub struct FormatErrorUtils;

impl FormatErrorUtils {
    /// Create a parameter format error
    /// 
    /// Used when a parameter is in an invalid format or type.
    /// 
    /// # Arguments
    /// * `parameter_name` - The name of the parameter
    /// * `value` - The invalid value provided
    /// * `expected_format` - Description of the expected format
    /// 
    /// # Returns
    /// * `LumidoxError::InvalidInput` - Formatted parameter format error
    pub fn format_error(parameter_name: &str, value: &str, expected_format: &str) -> LumidoxError {
        LumidoxError::InvalidInput(format!(
            "Parameter '{}' value '{}' has invalid format. Expected: {}", 
            parameter_name, value, expected_format
        ))
    }
    
    /// Create a type validation error
    pub fn type_error(parameter_name: &str, value: &str, expected_type: &str) -> LumidoxError {
        LumidoxError::InvalidInput(format!(
            "Parameter '{}' value '{}' has invalid type. Expected: {}", 
            parameter_name, value, expected_type
        ))
    }
    
    /// Create a structure validation error
    pub fn structure_error(data_name: &str, structure_issue: &str) -> LumidoxError {
        LumidoxError::InvalidInput(format!(
            "Structure validation failed for '{}': {}", 
            data_name, structure_issue
        ))
    }
    
    /// Create a format mismatch error
    pub fn mismatch_error(expected: &str, actual: &str, context: &str) -> LumidoxError {
        LumidoxError::InvalidInput(format!(
            "Format mismatch in {}: expected '{}', found '{}'", 
            context, expected, actual
        ))
    }
    
    /// Create an encoding error
    pub fn encoding_error(encoding_type: &str, value: &str, issue: &str) -> LumidoxError {
        LumidoxError::InvalidInput(format!(
            "Encoding error ({}): {} in value '{}'", 
            encoding_type, issue, value
        ))
    }
    
    /// Categorize a format error
    pub fn categorize_error(error_message: &str) -> FormatErrorCategory {
        let message_lower = error_message.to_lowercase();
        
        if message_lower.contains("type") {
            FormatErrorCategory::Type
        } else if message_lower.contains("structure") {
            FormatErrorCategory::Structure
        } else if message_lower.contains("mismatch") {
            FormatErrorCategory::Mismatch
        } else {
            FormatErrorCategory::Encoding
        }
    }
}
