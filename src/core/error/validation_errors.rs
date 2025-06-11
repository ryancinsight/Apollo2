//! Input validation error handling utilities
//!
//! This module provides specialized error handling utilities and documentation
//! for input validation and parameter constraint errors in the Lumidox II Controller.
//! 
//! Validation errors typically occur during:
//! - User input parameter validation
//! - Configuration value range checking
//! - Stage number and current value validation
//! - Command line argument parsing
//! - Interactive menu input processing

use super::types::LumidoxError;

/// Validation error categories for better error classification
/// 
/// This enum helps categorize different types of validation errors
/// for more specific error handling and user feedback.
#[derive(Debug, Clone, PartialEq)]
pub enum ValidationErrorCategory {
    /// Parameter range or bounds errors
    Range,
    /// Required parameter missing errors
    Missing,
    /// Parameter format or type errors
    Format,
    /// Parameter combination or dependency errors
    Combination,
    /// Parameter constraint violation errors
    Constraint,
}

/// Validation error utilities and helper functions
pub struct ValidationErrorUtils;

impl ValidationErrorUtils {
    /// Create a parameter range error
    /// 
    /// Used when a parameter value is outside the acceptable range.
    /// 
    /// # Arguments
    /// * `parameter_name` - The name of the parameter
    /// * `value` - The invalid value provided
    /// * `min` - The minimum acceptable value
    /// * `max` - The maximum acceptable value
    /// 
    /// # Returns
    /// * `LumidoxError::InvalidInput` - Formatted parameter range error
    /// 
    /// # Example
    /// ```
    /// let error = ValidationErrorUtils::range_error("stage_number", "6", "1", "5");
    /// ```
    pub fn range_error(parameter_name: &str, value: &str, min: &str, max: &str) -> LumidoxError {
        LumidoxError::InvalidInput(format!(
            "Parameter '{}' value '{}' is out of range. Must be between {} and {}", 
            parameter_name, value, min, max
        ))
    }
    
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
    /// 
    /// # Example
    /// ```
    /// let error = ValidationErrorUtils::missing_parameter_error("current_value", "fire command");
    /// ```
    pub fn missing_parameter_error(parameter_name: &str, context: &str) -> LumidoxError {
        LumidoxError::InvalidInput(format!(
            "Required parameter '{}' is missing for {}", 
            parameter_name, context
        ))
    }
    
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
    /// 
    /// # Example
    /// ```
    /// let error = ValidationErrorUtils::format_error("current_value", "abc", "positive integer");
    /// ```
    pub fn format_error(parameter_name: &str, value: &str, expected_format: &str) -> LumidoxError {
        LumidoxError::InvalidInput(format!(
            "Parameter '{}' value '{}' has invalid format. Expected: {}", 
            parameter_name, value, expected_format
        ))
    }
    
    /// Create a parameter combination error
    /// 
    /// Used when parameter combinations are invalid or conflicting.
    /// 
    /// # Arguments
    /// * `parameters` - Description of the conflicting parameters
    /// * `reason` - Explanation of why the combination is invalid
    /// 
    /// # Returns
    /// * `LumidoxError::InvalidInput` - Formatted parameter combination error
    /// 
    /// # Example
    /// ```
    /// let error = ValidationErrorUtils::combination_error("stage and current", "cannot specify both stage number and custom current");
    /// ```
    pub fn combination_error(parameters: &str, reason: &str) -> LumidoxError {
        LumidoxError::InvalidInput(format!(
            "Invalid parameter combination for {}: {}", 
            parameters, reason
        ))
    }
    
    /// Create a parameter constraint violation error
    /// 
    /// Used when parameters violate business logic or safety constraints.
    /// 
    /// # Arguments
    /// * `parameter_name` - The name of the parameter
    /// * `value` - The value that violates constraints
    /// * `constraint` - Description of the violated constraint
    /// 
    /// # Returns
    /// * `LumidoxError::InvalidInput` - Formatted constraint violation error
    /// 
    /// # Example
    /// ```
    /// let error = ValidationErrorUtils::constraint_error("current_value", "5000", "exceeds device maximum safe current");
    /// ```
    pub fn constraint_error(parameter_name: &str, value: &str, constraint: &str) -> LumidoxError {
        LumidoxError::InvalidInput(format!(
            "Parameter '{}' value '{}' violates constraint: {}", 
            parameter_name, value, constraint
        ))
    }
    
    /// Create a stage number validation error
    /// 
    /// Specialized error for invalid stage numbers (common validation case).
    /// 
    /// # Arguments
    /// * `stage_number` - The invalid stage number provided
    /// 
    /// # Returns
    /// * `LumidoxError::InvalidInput` - Formatted stage number error
    /// 
    /// # Example
    /// ```
    /// let error = ValidationErrorUtils::invalid_stage_error(0);
    /// ```
    pub fn invalid_stage_error(stage_number: u8) -> LumidoxError {
        LumidoxError::InvalidInput(format!(
            "Invalid stage number: {}. Must be between 1 and 5", 
            stage_number
        ))
    }
    
    /// Create a current value validation error
    /// 
    /// Specialized error for invalid current values (common validation case).
    /// 
    /// # Arguments
    /// * `current_ma` - The invalid current value in milliamps
    /// * `max_current_ma` - The maximum allowed current in milliamps
    /// 
    /// # Returns
    /// * `LumidoxError::InvalidInput` - Formatted current value error
    /// 
    /// # Example
    /// ```
    /// let error = ValidationErrorUtils::invalid_current_error(6000, 5000);
    /// ```
    pub fn invalid_current_error(current_ma: u16, max_current_ma: u16) -> LumidoxError {
        LumidoxError::InvalidInput(format!(
            "Invalid current value: {}mA. Must be between 1 and {}mA", 
            current_ma, max_current_ma
        ))
    }
    
    /// Categorize a validation error for better handling
    /// 
    /// Analyzes a validation error message to determine its category.
    /// This can be used for implementing category-specific error handling.
    /// 
    /// # Arguments
    /// * `error_message` - The validation error message to categorize
    /// 
    /// # Returns
    /// * `ValidationErrorCategory` - The determined error category
    /// 
    /// # Example
    /// ```
    /// let category = ValidationErrorUtils::categorize_error("Parameter 'stage_number' value '6' is out of range");
    /// assert_eq!(category, ValidationErrorCategory::Range);
    /// ```
    pub fn categorize_error(error_message: &str) -> ValidationErrorCategory {
        let message_lower = error_message.to_lowercase();
        
        if message_lower.contains("range") || message_lower.contains("between") || message_lower.contains("out of") {
            ValidationErrorCategory::Range
        } else if message_lower.contains("missing") || message_lower.contains("required") {
            ValidationErrorCategory::Missing
        } else if message_lower.contains("format") || message_lower.contains("invalid format") || message_lower.contains("expected") {
            ValidationErrorCategory::Format
        } else if message_lower.contains("combination") || message_lower.contains("conflict") {
            ValidationErrorCategory::Combination
        } else {
            ValidationErrorCategory::Constraint
        }
    }
}
