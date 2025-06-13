//! Device state error handling utilities
//!
//! This module provides specialized error handling for device state-related
//! errors in the Lumidox II Controller. It handles various state scenarios
//! including state transitions, validation, and consistency checks.

use crate::core::error::types::LumidoxError;

/// State error categories for better error classification
#[derive(Debug, Clone, PartialEq)]
pub enum StateErrorCategory {
    /// State transition errors
    Transition,
    /// State validation errors
    Validation,
    /// State consistency errors
    Consistency,
    /// Invalid state errors
    Invalid,
}

/// State error utilities and helper functions
pub struct StateErrorUtils;

impl StateErrorUtils {
    /// Create a device state error
    /// 
    /// Used when the device is in an invalid or unexpected state.
    /// 
    /// # Arguments
    /// * `current_state` - The current device state
    /// * `expected_state` - The expected device state
    /// 
    /// # Returns
    /// * `LumidoxError::DeviceError` - Formatted device state error
    pub fn state_error(current_state: &str, expected_state: &str) -> LumidoxError {
        LumidoxError::DeviceError(format!(
            "Device state error: expected '{}', found '{}'", 
            expected_state, current_state
        ))
    }
    
    /// Create a state transition error
    pub fn transition_error(from_state: &str, to_state: &str, reason: &str) -> LumidoxError {
        LumidoxError::DeviceError(format!(
            "State transition failed from '{}' to '{}': {}", 
            from_state, to_state, reason
        ))
    }
    
    /// Create a state validation error
    pub fn validation_error(state: &str, validation_issue: &str) -> LumidoxError {
        LumidoxError::DeviceError(format!(
            "State validation failed for '{}': {}", 
            state, validation_issue
        ))
    }
    
    /// Categorize a state error
    pub fn categorize_error(error_message: &str) -> StateErrorCategory {
        let message_lower = error_message.to_lowercase();
        
        if message_lower.contains("transition") {
            StateErrorCategory::Transition
        } else if message_lower.contains("validation") {
            StateErrorCategory::Validation
        } else if message_lower.contains("consistency") {
            StateErrorCategory::Consistency
        } else {
            StateErrorCategory::Invalid
        }
    }
}
