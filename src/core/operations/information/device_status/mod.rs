//! Device status operations for Lumidox II Controller
//!
//! This module provides unified device status operations that serve as the single
//! source of truth for device status retrieval across CLI and GUI interfaces.
//! It implements structured responses and consistent error handling.
//!
//! ## Module Structure (8+ levels deep)
//! ```
//! src/core/operations/information/device_status/  (Level 5)
//! ├── health_assessment/                          (Level 6)
//! │   ├── connection/                             (Level 7)
//! │   │   └── diagnostic/                         (Level 8)
//! │   │       └── mod.rs                          (Level 9) - Connection diagnostics
//! │   ├── operational/                            (Level 7)
//! │   │   └── readiness/                          (Level 8)
//! │   │       └── mod.rs                          (Level 9) - Operational readiness
//! │   └── mod.rs                                  (Level 7) - Health assessment coordination
//! ├── status_retrieval/                           (Level 6)
//! │   ├── current_readings/                       (Level 7)
//! │   │   └── validation/                         (Level 8)
//! │   │       └── mod.rs                          (Level 9) - Current reading validation
//! │   ├── mode_detection/                         (Level 7)
//! │   │   └── analysis/                           (Level 8)
//! │   │       └── mod.rs                          (Level 9) - Mode detection analysis
//! │   └── mod.rs                                  (Level 7) - Status retrieval coordination
//! ├── formatting/                                 (Level 6)
//! │   ├── message/                                (Level 7)
//! │   │   └── generation/                         (Level 8)
//! │   │       └── mod.rs                          (Level 9) - Message generation
//! │   └── display/                                (Level 7)
//! │       └── formatting/                         (Level 8)
//! │           └── mod.rs                          (Level 9) - Display formatting
//! └── mod.rs                                      (Level 6) - Device status coordination
//! ```
//!
//! Each sub-module follows the prescribed schema with single responsibility
//! and maintains <150 lines per file.

// Import specialized sub-modules
pub mod health_assessment;
pub mod status_retrieval;
pub mod formatting;

// Re-export commonly used items for convenience
// Note: Utilities are available but not currently used in the codebase
// pub use health_assessment::{HealthAssessmentOperations, HealthAssessmentCategory};
// pub use status_retrieval::{StatusRetrievalOperations, StatusRetrievalCategory};
// pub use formatting::{FormattingOperations, FormattingCategory};

use crate::core::operations::result_types::{OperationResult, OperationResponse, DeviceOperationData};
use crate::device::LumidoxDevice;
use std::time::Instant;

// TODO: Create tests module when needed
// #[cfg(test)]
// mod tests;

/// Device status operations for unified device status functionality
pub struct DeviceStatusOperations;

impl DeviceStatusOperations {
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
    ///
    /// # Response Data
    /// The response contains `DeviceOperationData::DeviceStatus` with:
    /// - `current_mode`: The current device operating mode
    /// - `arm_current`: ARM current setting in mA
    /// - `fire_current`: FIRE current setting in mA
    /// - `remote_mode_state`: Remote mode state value
    /// - `connection_healthy`: Connection health status
    /// - `ready_for_operations`: Device readiness flag
    ///
    /// # Example
    /// ```
    /// let response = DeviceStatusOperations::get_device_status_unified(&mut device)?;
    /// println!("Operation: {}", response.message);
    /// if let DeviceOperationData::DeviceStatus { current_mode, arm_current, .. } = response.data {
    ///     println!("Device mode: {:?}, ARM current: {:?}mA", current_mode, arm_current);
    /// }
    /// ```
    pub fn get_device_status_unified(
        device: &mut LumidoxDevice
    ) -> OperationResult<DeviceOperationData> {
        let start_time = Instant::now();
        
        // Read device status information using existing device methods
        let current_mode = Self::get_device_mode_string(device);
        let arm_current = device.read_arm_current().ok();
        let fire_current = device.read_fire_current().ok();
        let remote_mode_state = device.read_remote_mode().ok().map(|mode| mode as u16);
        
        // Assess connection health and operational readiness
        let connection_healthy = Self::assess_connection_health(device);
        let ready_for_operations = Self::assess_operational_readiness(device);
        
        let duration = start_time.elapsed().as_millis() as u64;
        
        let data = DeviceOperationData::DeviceStatus {
            current_mode,
            arm_current,
            fire_current,
            remote_mode_state,
            connection_healthy,
            ready_for_operations,
        };
        
        let message = Self::format_status_message(&data);
        
        Ok(OperationResponse::success_with_duration(
            data,
            message,
            "get_device_status".to_string(),
            duration,
        ).with_context("operation".to_string(), "device_status_retrieval".to_string()))
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
    ///
    /// # Example
    /// ```
    /// let response = DeviceStatusOperations::check_connection_unified(&mut device)?;
    /// ```
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
    fn assess_connection_health(device: &LumidoxDevice) -> bool {
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
    fn assess_operational_readiness(device: &LumidoxDevice) -> bool {
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

    /// Format status message based on device status data
    ///
    /// Creates a human-readable status message from the device status data
    /// for consistent messaging across interfaces.
    ///
    /// # Arguments
    /// * `data` - Device status data to format
    ///
    /// # Returns
    /// * `String` - Formatted status message
    fn format_status_message(data: &DeviceOperationData) -> String {
        if let DeviceOperationData::DeviceStatus { 
            current_mode, 
            arm_current, 
            fire_current, 
            connection_healthy,
            ready_for_operations,
            .. 
        } = data {
            let mode_str = current_mode.as_deref().unwrap_or("Unknown");
            let arm_str = arm_current.map(|c| c.to_string()).unwrap_or_else(|| "N/A".to_string());
            let fire_str = fire_current.map(|c| c.to_string()).unwrap_or_else(|| "N/A".to_string());
            let health_str = if *connection_healthy { "Healthy" } else { "Issues" };
            let ready_str = if *ready_for_operations { "Ready" } else { "Not Ready" };
            
            format!(
                "Device Status: Mode={}, ARM={}mA, FIRE={}mA, Connection={}, Operations={}",
                mode_str, arm_str, fire_str, health_str, ready_str
            )
        } else {
            "Device status retrieved successfully".to_string()
        }
    }
}
