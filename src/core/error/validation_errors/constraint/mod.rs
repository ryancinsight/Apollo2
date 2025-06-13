//! Constraint validation error handling utilities
//!
//! This module provides specialized error handling for constraint validation-related
//! errors in the Lumidox II Controller. It handles various constraint validation scenarios
//! including business logic constraints, safety constraints, and policy violations.

use crate::core::error::types::LumidoxError;

/// Constraint error categories for better error classification
#[derive(Debug, Clone, PartialEq)]
pub enum ConstraintErrorCategory {
    /// Business logic constraint errors
    BusinessLogic,
    /// Safety constraint errors
    Safety,
    /// Policy violation errors
    Policy,
    /// Regulatory constraint errors
    Regulatory,
}

/// Constraint error utilities and helper functions
pub struct ConstraintErrorUtils;

impl ConstraintErrorUtils {
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
    pub fn constraint_error(parameter_name: &str, value: &str, constraint: &str) -> LumidoxError {
        LumidoxError::InvalidInput(format!(
            "Parameter '{}' value '{}' violates constraint: {}", 
            parameter_name, value, constraint
        ))
    }
    
    /// Create a business logic constraint error
    pub fn business_logic_error(operation: &str, constraint: &str) -> LumidoxError {
        LumidoxError::InvalidInput(format!(
            "Business logic constraint violated for {}: {}", 
            operation, constraint
        ))
    }
    
    /// Create a safety constraint error
    pub fn safety_constraint_error(parameter: &str, value: &str, safety_limit: &str) -> LumidoxError {
        LumidoxError::InvalidInput(format!(
            "Safety constraint violated: {} = '{}' exceeds safety limit of {}", 
            parameter, value, safety_limit
        ))
    }
    
    /// Create a policy violation error
    pub fn policy_violation_error(action: &str, policy: &str) -> LumidoxError {
        LumidoxError::InvalidInput(format!(
            "Policy violation: action '{}' violates policy '{}'", 
            action, policy
        ))
    }
    
    /// Create a regulatory constraint error
    pub fn regulatory_error(requirement: &str, violation: &str) -> LumidoxError {
        LumidoxError::InvalidInput(format!(
            "Regulatory constraint violated: {} - {}", 
            requirement, violation
        ))
    }
    
    /// Categorize a constraint error
    pub fn categorize_error(error_message: &str) -> ConstraintErrorCategory {
        let message_lower = error_message.to_lowercase();
        
        if message_lower.contains("business") || message_lower.contains("logic") {
            ConstraintErrorCategory::BusinessLogic
        } else if message_lower.contains("safety") {
            ConstraintErrorCategory::Safety
        } else if message_lower.contains("policy") {
            ConstraintErrorCategory::Policy
        } else {
            ConstraintErrorCategory::Regulatory
        }
    }
}
