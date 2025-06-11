//! Shutdown operations for Lumidox II Controller
//!
//! This module provides unified shutdown operations that serve as the single
//! source of truth for device shutdown across CLI and GUI interfaces.
//! It implements structured responses and consistent error handling.
//!
//! The shutdown operations provide:
//! - Unified device shutdown with state preservation
//! - Structured operation responses with state transition data
//! - Consistent error handling and device state management
//! - Interface-independent business logic

use crate::core::LumidoxError;
use crate::core::operations::result_types::{OperationResult, OperationResponse, DeviceOperationData};
use crate::device::LumidoxDevice;
use std::time::Instant;

#[cfg(test)]
mod tests;

/// Shutdown operations for unified device shutdown functionality
pub struct ShutdownOperations;

impl ShutdownOperations {
    /// Shutdown the device using unified operation pattern
    ///
    /// This function provides the single source of truth for device shutdown operations
    /// across all interfaces (CLI, GUI). It performs validation, executes the shutdown
    /// operation, and returns structured response data.
    ///
    /// # Arguments
    /// * `device` - Mutable reference to the device for shutdown operations
    ///
    /// # Returns
    /// * `OperationResult<DeviceOperationData>` - Structured operation result
    ///
    /// # Response Data
    /// The response contains `DeviceOperationData::DeviceControl` with:
    /// - `previous_state`: The device state before shutdown
    /// - `new_state`: The device state after shutdown
    /// - `success`: Whether the shutdown operation succeeded
    ///
    /// # Example
    /// ```
    /// let response = ShutdownOperations::shutdown_device_unified(&mut device)?;
    /// println!("Operation: {}", response.message);
    /// if let DeviceOperationData::DeviceControl { success, new_state, .. } = response.data {
    ///     if success {
    ///         println!("Device shutdown successfully. New state: {:?}", new_state);
    ///     }
    /// }
    /// ```
    pub fn shutdown_device_unified(
        device: &mut LumidoxDevice
    ) -> OperationResult<DeviceOperationData> {
        let start_time = Instant::now();
        
        // Get current state before operation
        let previous_state = Self::get_device_state_string(device);
        
        // Perform the shutdown operation using existing device method
        match device.shutdown() {
            Ok(_) => {
                let duration = start_time.elapsed().as_millis() as u64;
                let new_state = Self::get_device_state_string(device);
                
                let data = DeviceOperationData::DeviceControl {
                    previous_state,
                    new_state: new_state.clone(),
                    success: true,
                };
                
                let message = "Device shutdown successfully and returned to local mode".to_string();
                
                Ok(OperationResponse::success_with_duration(
                    data,
                    message,
                    "shutdown_device".to_string(),
                    duration,
                ).with_context("operation".to_string(), "device_shutdown".to_string()))
            }
            Err(e) => {
                let data = DeviceOperationData::DeviceControl {
                    previous_state,
                    new_state: Self::get_device_state_string(device),
                    success: false,
                };
                
                Err(LumidoxError::DeviceError(format!("Failed to shutdown device: {}", e)))
            }
        }
    }

    /// Validate device readiness for shutdown operations
    ///
    /// Provides centralized validation logic for device shutdown readiness
    /// used across all shutdown operations.
    ///
    /// # Arguments
    /// * `device` - Reference to the device to validate
    ///
    /// # Returns
    /// * `Result<()>` - Success if ready for shutdown, error if not ready
    ///
    /// # Example
    /// ```
    /// ShutdownOperations::validate_shutdown_readiness(&device)?; // OK if ready
    /// ```
    pub fn validate_shutdown_readiness(device: &LumidoxDevice) -> crate::core::Result<()> {
        // Check if device is in a state that allows shutdown
        match device.current_mode() {
            Some(mode) => {
                use crate::device::models::DeviceMode;
                match mode {
                    DeviceMode::Local => {
                        Err(LumidoxError::InvalidInput(
                            "Device is already in local mode.".to_string()
                        ))
                    }
                    DeviceMode::Standby | DeviceMode::Armed | DeviceMode::Remote => {
                        Ok(()) // All remote modes can be shutdown
                    }
                }
            }
            None => {
                Err(LumidoxError::InvalidInput(
                    "Device mode is unknown. Cannot determine shutdown readiness.".to_string()
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
