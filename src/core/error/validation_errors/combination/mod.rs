//! Combination validation error handling utilities
//!
//! This module provides specialized error handling for parameter combination-related
//! errors in the Lumidox II Controller. It handles various combination validation scenarios
//! including parameter conflicts, dependencies, and mutual exclusions.

use crate::core::error::types::LumidoxError;

/// Combination error categories for better error classification
#[derive(Debug, Clone, PartialEq)]
pub enum CombinationErrorCategory {
    /// Parameter conflict errors
    Conflict,
    /// Parameter dependency errors
    Dependency,
    /// Mutual exclusion errors
    MutualExclusion,
    /// Conditional requirement errors
    Conditional,
}

/// Combination error utilities and helper functions
pub struct CombinationErrorUtils;

impl CombinationErrorUtils {
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
    pub fn combination_error(parameters: &str, reason: &str) -> LumidoxError {
        LumidoxError::InvalidInput(format!(
            "Invalid parameter combination for {}: {}", 
            parameters, reason
        ))
    }
    
    /// Create a parameter conflict error
    pub fn conflict_error(param1: &str, param2: &str, reason: &str) -> LumidoxError {
        LumidoxError::InvalidInput(format!(
            "Parameter conflict between '{}' and '{}': {}", 
            param1, param2, reason
        ))
    }
    
    /// Create a parameter dependency error
    pub fn dependency_error(dependent_param: &str, required_param: &str) -> LumidoxError {
        LumidoxError::InvalidInput(format!(
            "Parameter '{}' requires '{}' to be specified", 
            dependent_param, required_param
        ))
    }
    
    /// Create a mutual exclusion error
    pub fn mutual_exclusion_error(param1: &str, param2: &str) -> LumidoxError {
        LumidoxError::InvalidInput(format!(
            "Parameters '{}' and '{}' are mutually exclusive", 
            param1, param2
        ))
    }
    
    /// Create a conditional requirement error
    pub fn conditional_error(condition: &str, required_param: &str) -> LumidoxError {
        LumidoxError::InvalidInput(format!(
            "When {}, parameter '{}' is required", 
            condition, required_param
        ))
    }
    
    /// Categorize a combination error
    pub fn categorize_error(error_message: &str) -> CombinationErrorCategory {
        let message_lower = error_message.to_lowercase();
        
        if message_lower.contains("conflict") {
            CombinationErrorCategory::Conflict
        } else if message_lower.contains("dependency") || message_lower.contains("requires") {
            CombinationErrorCategory::Dependency
        } else if message_lower.contains("mutual") || message_lower.contains("exclusive") {
            CombinationErrorCategory::MutualExclusion
        } else {
            CombinationErrorCategory::Conditional
        }
    }
}
