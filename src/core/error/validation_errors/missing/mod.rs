//! Missing parameter validation error handling utilities
//!
//! This module provides specialized error handling for missing parameter-related
//! errors in the Lumidox II Controller. It handles various missing parameter scenarios
//! including required parameters, optional parameters, and field validation.

use crate::core::error::types::LumidoxError;

/// Missing error categories for better error classification
#[derive(Debug, Clone, PartialEq)]
pub enum MissingErrorCategory {
    /// Required parameter errors
    Required,
    /// Optional parameter errors
    Optional,
    /// Field validation errors
    Field,
    /// Configuration errors
    Configuration,
}

/// Missing error utilities and helper functions
pub struct MissingErrorUtils;

impl MissingErrorUtils {
    /// Create a required parameter missing error
    /// 
    /// Used when a required parameter is not provided.
    /// 
    /// # Arguments
    /// * `parameter_name` - The name of the missing parameter
    /// * `context` - The context where the parameter is required
    /// 
    /// # Returns
    /// * `LumidoxError::InvalidInput` - Formatted missing parameter error
    pub fn missing_parameter_error(parameter_name: &str, context: &str) -> LumidoxError {
        LumidoxError::InvalidInput(format!(
            "Required parameter '{}' is missing for {}", 
            parameter_name, context
        ))
    }
    
    /// Create a missing field error
    pub fn missing_field_error(field_name: &str, structure: &str) -> LumidoxError {
        LumidoxError::InvalidInput(format!(
            "Required field '{}' is missing from {}", 
            field_name, structure
        ))
    }
    
    /// Create a missing configuration error
    pub fn missing_configuration_error(config_name: &str, component: &str) -> LumidoxError {
        LumidoxError::InvalidInput(format!(
            "Required configuration '{}' is missing for {}", 
            config_name, component
        ))
    }
    
    /// Create a missing optional parameter warning
    pub fn missing_optional_warning(parameter_name: &str, default_value: &str) -> LumidoxError {
        LumidoxError::InvalidInput(format!(
            "Optional parameter '{}' not provided, using default: {}", 
            parameter_name, default_value
        ))
    }
    
    /// Categorize a missing error
    pub fn categorize_error(error_message: &str) -> MissingErrorCategory {
        let message_lower = error_message.to_lowercase();
        
        if message_lower.contains("required") {
            MissingErrorCategory::Required
        } else if message_lower.contains("optional") {
            MissingErrorCategory::Optional
        } else if message_lower.contains("field") {
            MissingErrorCategory::Field
        } else {
            MissingErrorCategory::Configuration
        }
    }
}
