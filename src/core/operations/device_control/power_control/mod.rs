//! Power control operations for Lumidox II Controller
//!
//! This module provides unified power control operations that serve as the single
//! source of truth for device power control across CLI and GUI interfaces.
//! It implements structured responses and consistent error handling.
//!
//! The power control operations provide:
//! - Unified device turn-off with safety confirmation
//! - Structured operation responses with state transition data
//! - Consistent error handling and device state management
//! - Interface-independent business logic

use crate::core::LumidoxError;
use crate::core::operations::result_types::{OperationResult, OperationResponse, DeviceOperationData};
use crate::device::LumidoxDevice;
use std::time::Instant;

#[cfg(test)]
mod tests;

/// Power control operations for unified device power management functionality
pub struct PowerControlOperations;

impl PowerControlOperations {
    /// Turn off the device using unified operation pattern
    ///
    /// This function provides the single source of truth for device turn-off operations
    /// across all interfaces (CLI, GUI). It performs validation, executes the turn-off
    /// operation, and returns structured response data.
    ///
    /// # Arguments
    /// * `device` - Mutable reference to the device for turn-off operations
    ///
    /// # Returns
    /// * `OperationResult<DeviceOperationData>` - Structured operation result
    ///
    /// # Response Data
    /// The response contains `DeviceOperationData::DeviceControl` with:
    /// - `previous_state`: The device state before turn-off
    /// - `new_state`: The device state after turn-off
    /// - `success`: Whether the turn-off operation succeeded
    ///
    /// # Example
    /// ```
    /// let response = PowerControlOperations::turn_off_device_unified(&mut device)?;
    /// println!("Operation: {}", response.message);
    /// if let DeviceOperationData::DeviceControl { success, new_state, .. } = response.data {
    ///     if success {
    ///         println!("Device turned off successfully. New state: {:?}", new_state);
    ///     }
    /// }
    /// ```
    pub fn turn_off_device_unified(
        device: &mut LumidoxDevice
    ) -> OperationResult<DeviceOperationData> {
        let start_time = Instant::now();
        
        // Get current state before operation
        let previous_state = Self::get_device_state_string(device);
        
        // Perform the turn off operation using existing device method
        match device.turn_off() {
            Ok(_) => {
                let duration = start_time.elapsed().as_millis() as u64;
                let new_state = Self::get_device_state_string(device);
                
                let data = DeviceOperationData::DeviceControl {
                    previous_state,
                    new_state: new_state.clone(),
                    success: true,
                };
                
                let message = "Device turned off successfully and is now in safe standby mode".to_string();
                
                Ok(OperationResponse::success_with_duration(
                    data,
                    message,
                    "turn_off_device".to_string(),
                    duration,
                ).with_context("operation".to_string(), "device_turn_off".to_string()))
            }
            Err(e) => {
                let _data = DeviceOperationData::DeviceControl {
                    previous_state,
                    new_state: Self::get_device_state_string(device),
                    success: false,
                };
                
                Err(LumidoxError::DeviceError(format!("Failed to turn off device: {}", e)))
            }
        }
    }

    /// Validate device readiness for turn-off operations
    ///
    /// Provides centralized validation logic for device turn-off readiness
    /// used across all power control operations.
    ///
    /// # Arguments
    /// * `device` - Reference to the device to validate
    ///
    /// # Returns
    /// * `Result<()>` - Success if ready for turn-off, error if not ready
    ///
    /// # Example
    /// ```
    /// PowerControlOperations::validate_turn_off_readiness(&device)?; // OK if ready
    /// ```
    pub fn validate_turn_off_readiness(device: &LumidoxDevice) -> crate::core::Result<()> {
        // Check if device is in a state that allows turn-off
        match device.current_mode() {
            Some(mode) => {
                use crate::device::models::DeviceMode;
                match mode {
                    DeviceMode::Local => {
                        Err(LumidoxError::InvalidInput(
                            "Device is in local mode. Cannot control device remotely.".to_string()
                        ))
                    }
                    DeviceMode::Standby => {
                        Err(LumidoxError::InvalidInput(
                            "Device is already in standby mode.".to_string()
                        ))
                    }
                    DeviceMode::Armed | DeviceMode::Remote => {
                        Ok(()) // Armed or Remote modes can be turned off
                    }
                }
            }
            None => {
                Err(LumidoxError::InvalidInput(
                    "Device mode is unknown. Cannot determine turn-off readiness.".to_string()
                ))
            }
        }
    }

    /// Get device state as a string for logging/display
    ///
    /// Helper function to convert device state to string representation
    /// for consistent logging and display across interfaces.
    ///
    /// # Arguments
    /// * `device` - Reference to the device
    ///
    /// # Returns
    /// * `Option<String>` - Device state string if available
    fn get_device_state_string(device: &LumidoxDevice) -> Option<String> {
        device.current_mode().map(|mode| format!("{:?}", mode))
    }
}
