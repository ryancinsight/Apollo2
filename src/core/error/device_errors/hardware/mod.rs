//! Device hardware error handling utilities
//!
//! This module provides specialized error handling for device hardware-related
//! errors in the Lumidox II Controller. It handles various hardware scenarios
//! including malfunction detection, diagnostics, and component failures.

use crate::core::error::types::LumidoxError;

/// Hardware error categories for better error classification
#[derive(Debug, Clone, PartialEq)]
pub enum HardwareErrorCategory {
    /// Hardware malfunction errors
    Malfunction,
    /// Hardware diagnostic errors
    Diagnostics,
    /// Component failure errors
    ComponentFailure,
    /// Hardware communication errors
    Communication,
}

/// Hardware error utilities and helper functions
pub struct HardwareErrorUtils;

impl HardwareErrorUtils {
    /// Create a hardware malfunction error
    /// 
    /// Used when the device hardware is malfunctioning.
    /// 
    /// # Arguments
    /// * `component` - The hardware component that is malfunctioning
    /// * `symptoms` - Description of the malfunction symptoms
    /// 
    /// # Returns
    /// * `LumidoxError::DeviceError` - Formatted hardware malfunction error
    pub fn malfunction_error(component: &str, symptoms: &str) -> LumidoxError {
        LumidoxError::DeviceError(format!(
            "Hardware malfunction in {}: {}", 
            component, symptoms
        ))
    }
    
    /// Create a hardware diagnostic error
    pub fn diagnostic_error(test_name: &str, failure_details: &str) -> LumidoxError {
        LumidoxError::DeviceError(format!(
            "Hardware diagnostic '{}' failed: {}", 
            test_name, failure_details
        ))
    }
    
    /// Create a component failure error
    pub fn component_failure_error(component: &str, failure_mode: &str) -> LumidoxError {
        LumidoxError::DeviceError(format!(
            "Component failure in {}: {}", 
            component, failure_mode
        ))
    }
    
    /// Categorize a hardware error
    pub fn categorize_error(error_message: &str) -> HardwareErrorCategory {
        let message_lower = error_message.to_lowercase();
        
        if message_lower.contains("malfunction") {
            HardwareErrorCategory::Malfunction
        } else if message_lower.contains("diagnostic") {
            HardwareErrorCategory::Diagnostics
        } else if message_lower.contains("component") || message_lower.contains("failure") {
            HardwareErrorCategory::ComponentFailure
        } else {
            HardwareErrorCategory::Communication
        }
    }
}
