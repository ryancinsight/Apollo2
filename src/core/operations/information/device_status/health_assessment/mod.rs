//! Health assessment operations for device status
//!
//! This module provides specialized health assessment operations for device status
//! in the Lumidox II Controller. It handles various health assessment scenarios
//! including connection diagnostics, operational readiness, and system health checks.

// Import specialized sub-modules
pub mod connection;

// Re-export commonly used items for convenience
// Note: Utilities are available but not currently used in the codebase
// pub use connection::{ConnectionHealthOperations, ConnectionHealthCategory};

use crate::core::operations::result_types::{OperationResult, OperationResponse, DeviceOperationData};
use crate::device::LumidoxDevice;
use std::time::Instant;

/// Health assessment categories for better classification
#[derive(Debug, Clone, PartialEq)]
pub enum HealthAssessmentCategory {
    /// Connection health assessment
    Connection,
    /// Operational readiness assessment
    Operational,
    /// System health assessment
    System,
    /// Communication health assessment
    Communication,
}

/// Health assessment operations for device status functionality
pub struct HealthAssessmentOperations;

impl HealthAssessmentOperations {
    /// Assess connection health
    ///
    /// Provides centralized logic for determining device connection health
    /// based on communication responsiveness and state consistency.
    ///
    /// # Arguments
    /// * `device` - Reference to the device to assess
    ///
    /// # Returns
    /// * `bool` - True if connection is healthy, false otherwise
    pub fn assess_connection_health(device: &LumidoxDevice) -> bool {
        // Check if device mode is available (indicates communication)
        device.current_mode().is_some()
    }

    /// Assess operational readiness
    ///
    /// Provides centralized logic for determining device operational readiness
    /// based on current state and configuration.
    ///
    /// # Arguments
    /// * `device` - Reference to the device to assess
    ///
    /// # Returns
    /// * `bool` - True if ready for operations, false otherwise
    pub fn assess_operational_readiness(device: &LumidoxDevice) -> bool {
        match device.current_mode() {
            Some(mode) => {
                use crate::device::models::DeviceMode;
                match mode {
                    DeviceMode::Local => false, // Not ready in local mode
                    DeviceMode::Standby | DeviceMode::Armed | DeviceMode::Remote => true,
                }
            }
            None => false, // Not ready if mode is unknown
        }
    }

    /// Check connection health using unified operation pattern
    ///
    /// This function provides centralized connection health checking
    /// used across all status operations.
    ///
    /// # Arguments
    /// * `device` - Mutable reference to the device for health checking
    ///
    /// # Returns
    /// * `OperationResult<DeviceOperationData>` - Structured operation result
    pub fn check_connection_unified(
        device: &mut LumidoxDevice
    ) -> OperationResult<DeviceOperationData> {
        let start_time = Instant::now();
        
        let connection_healthy = Self::assess_connection_health(device);
        let current_mode = Self::get_device_mode_string(device);
        
        let duration = start_time.elapsed().as_millis() as u64;
        
        let data = DeviceOperationData::DeviceStatus {
            current_mode,
            arm_current: None,
            fire_current: None,
            remote_mode_state: None,
            connection_healthy,
            ready_for_operations: connection_healthy,
        };
        
        let message = if connection_healthy {
            "Device connection is healthy and responsive".to_string()
        } else {
            "Device connection issues detected".to_string()
        };
        
        Ok(OperationResponse::success_with_duration(
            data,
            message,
            "check_connection".to_string(),
            duration,
        ).with_context("operation".to_string(), "connection_health_check".to_string()))
    }

    /// Assess system health comprehensively
    pub fn assess_system_health(device: &LumidoxDevice) -> bool {
        let connection_healthy = Self::assess_connection_health(device);
        let operational_ready = Self::assess_operational_readiness(device);
        
        connection_healthy && operational_ready
    }

    /// Assess communication health
    pub fn assess_communication_health(device: &LumidoxDevice) -> bool {
        // Check if we can read basic device information
        device.current_mode().is_some()
    }

    /// Get device mode as a string for logging/display
    ///
    /// Helper function to convert device mode to string representation
    /// for consistent logging and display across interfaces.
    ///
    /// # Arguments
    /// * `device` - Reference to the device
    ///
    /// # Returns
    /// * `Option<String>` - Device mode string if available
    fn get_device_mode_string(device: &LumidoxDevice) -> Option<String> {
        device.current_mode().map(|mode| format!("{:?}", mode))
    }

    /// Categorize a health assessment
    pub fn categorize_assessment(assessment_type: &str) -> HealthAssessmentCategory {
        let assessment_lower = assessment_type.to_lowercase();
        
        if assessment_lower.contains("connection") {
            HealthAssessmentCategory::Connection
        } else if assessment_lower.contains("operational") || assessment_lower.contains("readiness") {
            HealthAssessmentCategory::Operational
        } else if assessment_lower.contains("communication") {
            HealthAssessmentCategory::Communication
        } else {
            HealthAssessmentCategory::System
        }
    }
}
