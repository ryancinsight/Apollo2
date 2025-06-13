//! Status retrieval operations for device status
//!
//! This module provides specialized status retrieval operations for device status
//! in the Lumidox II Controller. It handles various status retrieval scenarios
//! including current readings, mode detection, and state analysis.

use crate::core::operations::result_types::{OperationResult, OperationResponse, DeviceOperationData};
use crate::device::LumidoxDevice;
use std::time::Instant;

/// Status retrieval categories for better classification
#[derive(Debug, Clone, PartialEq)]
pub enum StatusRetrievalCategory {
    /// Current readings retrieval
    CurrentReadings,
    /// Mode detection retrieval
    ModeDetection,
    /// State analysis retrieval
    StateAnalysis,
    /// Configuration retrieval
    Configuration,
}

/// Status retrieval operations for device status functionality
pub struct StatusRetrievalOperations;

impl StatusRetrievalOperations {
    /// Get device status using unified operation pattern
    ///
    /// This function provides the single source of truth for device status operations
    /// across all interfaces (CLI, GUI). It performs validation, executes the status
    /// retrieval, and returns structured response data.
    ///
    /// # Arguments
    /// * `device` - Mutable reference to the device for status operations
    ///
    /// # Returns
    /// * `OperationResult<DeviceOperationData>` - Structured operation result
    pub fn get_device_status_unified(
        device: &mut LumidoxDevice
    ) -> OperationResult<DeviceOperationData> {
        let start_time = Instant::now();
        
        // Read device status information using existing device methods
        let current_mode = Self::get_device_mode_string(device);
        let arm_current = device.read_arm_current().ok();
        let fire_current = device.read_fire_current().ok();
        let remote_mode_state = device.read_remote_mode().ok().map(|mode| mode as u16);
        
        // Import health assessment operations
        use super::health_assessment::HealthAssessmentOperations;
        
        // Assess connection health and operational readiness
        let connection_healthy = HealthAssessmentOperations::assess_connection_health(device);
        let ready_for_operations = HealthAssessmentOperations::assess_operational_readiness(device);
        
        let duration = start_time.elapsed().as_millis() as u64;
        
        let data = DeviceOperationData::DeviceStatus {
            current_mode,
            arm_current,
            fire_current,
            remote_mode_state,
            connection_healthy,
            ready_for_operations,
        };
        
        // Import formatting operations
        use super::formatting::FormattingOperations;
        let message = FormattingOperations::format_status_message(&data);
        
        Ok(OperationResponse::success_with_duration(
            data,
            message,
            "get_device_status".to_string(),
            duration,
        ).with_context("operation".to_string(), "device_status_retrieval".to_string()))
    }

    /// Read current values from device
    pub fn read_current_values(device: &mut LumidoxDevice) -> (Option<u16>, Option<u16>) {
        let arm_current = device.read_arm_current().ok();
        let fire_current = device.read_fire_current().ok();
        (arm_current, fire_current)
    }

    /// Detect device mode
    pub fn detect_device_mode(device: &LumidoxDevice) -> Option<String> {
        Self::get_device_mode_string(device)
    }

    /// Read remote mode state
    pub fn read_remote_mode_state(device: &mut LumidoxDevice) -> Option<u16> {
        device.read_remote_mode().ok().map(|mode| mode as u16)
    }

    /// Analyze device state comprehensively
    pub fn analyze_device_state(device: &mut LumidoxDevice) -> (Option<String>, bool, bool) {
        use super::health_assessment::HealthAssessmentOperations;
        
        let mode = Self::detect_device_mode(device);
        let connection_healthy = HealthAssessmentOperations::assess_connection_health(device);
        let ready_for_operations = HealthAssessmentOperations::assess_operational_readiness(device);
        
        (mode, connection_healthy, ready_for_operations)
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

    /// Categorize a status retrieval operation
    pub fn categorize_retrieval(operation_type: &str) -> StatusRetrievalCategory {
        let operation_lower = operation_type.to_lowercase();
        
        if operation_lower.contains("current") || operation_lower.contains("reading") {
            StatusRetrievalCategory::CurrentReadings
        } else if operation_lower.contains("mode") || operation_lower.contains("detection") {
            StatusRetrievalCategory::ModeDetection
        } else if operation_lower.contains("state") || operation_lower.contains("analysis") {
            StatusRetrievalCategory::StateAnalysis
        } else {
            StatusRetrievalCategory::Configuration
        }
    }
}
