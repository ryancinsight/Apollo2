//! Stage firing operations for Lumidox II Controller
//!
//! This module provides unified stage firing operations that serve as the single
//! source of truth for stage-based firing across CLI and GUI interfaces.
//! It implements structured responses and consistent error handling.
//!
//! The stage operations provide:
//! - Unified stage firing with validation (stages 1-5)
//! - Structured operation responses with firing data
//! - Consistent error handling and device state management
//! - Interface-independent business logic

use crate::core::LumidoxError;
use crate::core::operations::result_types::{OperationResult, OperationResponse, DeviceOperationData};
use crate::device::LumidoxDevice;
use std::time::Instant;

#[cfg(test)]
mod tests;

/// Stage operations for unified firing functionality
pub struct StageOperations;

impl StageOperations {
    /// Fire a specific stage using unified operation pattern
    ///
    /// This function provides the single source of truth for stage firing operations
    /// across all interfaces (CLI, GUI). It performs validation, executes the firing
    /// operation, and returns structured response data.
    ///
    /// # Arguments
    /// * `device` - Mutable reference to the device for firing operations
    /// * `stage` - Stage number to fire (1-5)
    ///
    /// # Returns
    /// * `OperationResult<DeviceOperationData>` - Structured operation result
    ///
    /// # Response Data
    /// The response contains `DeviceOperationData::StageFiring` with:
    /// - `stage`: The stage number that was fired
    /// - `success`: Whether the firing operation succeeded
    /// - `current_ma`: The current value used for firing (if available)
    ///
    /// # Example
    /// ```
    /// let response = StageOperations::fire_stage_unified(&mut device, 3)?;
    /// println!("Operation: {}", response.message);
    /// if let DeviceOperationData::StageFiring { stage, success, .. } = response.data {
    ///     println!("Stage {} firing: {}", stage, if success { "Success" } else { "Failed" });
    /// }
    /// ```
    pub fn fire_stage_unified(
        device: &mut LumidoxDevice,
        stage: u8
    ) -> OperationResult<DeviceOperationData> {
        // Validate stage number
        if !(1..=5).contains(&stage) {
            return Err(LumidoxError::InvalidInput(
                format!("Invalid stage number: {}. Must be 1-5.", stage)
            ));
        }

        let start_time = Instant::now();

        // Attempt to get the current for this stage before firing (for response data)
        let current_ma = device.get_stage_arm_current(stage).ok();

        // Execute the firing operation using existing device method
        match device.fire_stage(stage) {
            Ok(_) => {
                let duration = start_time.elapsed().as_millis() as u64;

                let data = DeviceOperationData::StageFiring {
                    stage,
                    current_ma,
                    success: true,
                };

                let message = format!("Stage {} fired successfully", stage);

                Ok(OperationResponse::success_with_duration(
                    data,
                    message,
                    "fire_stage".to_string(),
                    duration,
                ).with_context("stage".to_string(), stage.to_string()))
            }
            Err(e) => {
                Err(LumidoxError::DeviceError(format!("Failed to fire stage {}: {}", stage, e)))
            }
        }
    }

    /// Validate stage number for firing operations
    ///
    /// Provides centralized validation logic for stage numbers used across
    /// all firing operations.
    ///
    /// # Arguments
    /// * `stage` - Stage number to validate
    ///
    /// # Returns
    /// * `Result<()>` - Success if valid, error if invalid
    ///
    /// # Example
    /// ```
    /// StageOperations::validate_stage_number(3)?; // OK
    /// StageOperations::validate_stage_number(6)?; // Error
    /// ```
    pub fn validate_stage_number(stage: u8) -> crate::core::Result<()> {
        if !(1..=5).contains(&stage) {
            Err(LumidoxError::InvalidInput(
                format!("Invalid stage number: {}. Must be 1-5.", stage)
            ))
        } else {
            Ok(())
        }
    }
}
