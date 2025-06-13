//! Range validation error handling utilities
//!
//! This module provides specialized error handling for range validation-related
//! errors in the Lumidox II Controller. It handles various range validation scenarios
//! including numeric bounds, stage ranges, and current value ranges.

use crate::core::error::types::LumidoxError;

/// Range error categories for better error classification
#[derive(Debug, Clone, PartialEq)]
pub enum RangeErrorCategory {
    /// Numeric bounds errors
    Numeric,
    /// Stage range errors
    Stage,
    /// Current range errors
    Current,
    /// General bounds errors
    Bounds,
}

/// Range error utilities and helper functions
pub struct RangeErrorUtils;

impl RangeErrorUtils {
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
    pub fn range_error(parameter_name: &str, value: &str, min: &str, max: &str) -> LumidoxError {
        LumidoxError::InvalidInput(format!(
            "Parameter '{}' value '{}' is out of range. Must be between {} and {}", 
            parameter_name, value, min, max
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
    pub fn invalid_current_error(current_ma: u16, max_current_ma: u16) -> LumidoxError {
        LumidoxError::InvalidInput(format!(
            "Invalid current value: {}mA. Must be between 1 and {}mA", 
            current_ma, max_current_ma
        ))
    }
    
    /// Create a numeric bounds error
    pub fn numeric_bounds_error(value: f64, min: f64, max: f64, unit: &str) -> LumidoxError {
        LumidoxError::InvalidInput(format!(
            "Numeric value {}{} is out of bounds. Must be between {}{} and {}{}", 
            value, unit, min, unit, max, unit
        ))
    }
    
    /// Categorize a range error
    pub fn categorize_error(error_message: &str) -> RangeErrorCategory {
        let message_lower = error_message.to_lowercase();
        
        if message_lower.contains("stage") {
            RangeErrorCategory::Stage
        } else if message_lower.contains("current") {
            RangeErrorCategory::Current
        } else if message_lower.contains("numeric") {
            RangeErrorCategory::Numeric
        } else {
            RangeErrorCategory::Bounds
        }
    }
}
