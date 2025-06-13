//! Stage information operations for Lumidox II Controller
//!
//! This module provides unified stage information operations that serve as the single
//! source of truth for stage data retrieval across CLI and GUI interfaces.
//! It implements structured responses and consistent error handling.
//!
//! The stage information operations provide:
//! - Unified stage data retrieval with current/voltage readings
//! - Structured operation responses with stage-specific information
//! - Consistent error handling and stage validation
//! - Interface-independent business logic

use crate::core::LumidoxError;
use crate::core::operations::result_types::{OperationResult, OperationResponse, DeviceOperationData};
use crate::device::LumidoxDevice;
use std::time::Instant;

// TODO: Create tests module when needed
// #[cfg(test)]
// mod tests;

/// Stage information operations for unified stage data functionality
pub struct StageInfoOperations;

impl StageInfoOperations {
    /// Get stage data using unified operation pattern
    ///
    /// This function provides the single source of truth for stage data operations
    /// across all interfaces (CLI, GUI). It performs validation, executes the stage
    /// data retrieval, and returns structured response data.
    ///
    /// # Arguments
    /// * `device` - Mutable reference to the device for stage operations
    /// * `stage` - Stage number to query (1-5)
    ///
    /// # Returns
    /// * `OperationResult<DeviceOperationData>` - Structured operation result
    ///
    /// # Response Data
    /// The response contains `DeviceOperationData::StageInfo` with:
    /// - `stage_number`: The stage number queried
    /// - `current_ma`: Stage current in mA (if available)
    /// - `voltage_v`: Stage voltage in V (if available)
    /// - `power_info`: Power information string
    /// - `ready_for_firing`: Stage readiness flag
    ///
    /// # Example
    /// ```
    /// let response = StageInfoOperations::get_stage_data_unified(&mut device, 2)?;
    /// println!("Operation: {}", response.message);
    /// if let DeviceOperationData::StageInfo { stage_number, current_ma, .. } = response.data {
    ///     println!("Stage {}: Current = {:?}mA", stage_number, current_ma);
    /// }
    /// ```
    pub fn get_stage_data_unified(
        device: &mut LumidoxDevice,
        stage: u8
    ) -> OperationResult<DeviceOperationData> {
        let start_time = Instant::now();
        
        // Validate stage number first
        Self::validate_stage_number(stage)?;
        
        // Read stage-specific information
        let current_ma = Self::read_stage_current(device, stage).ok();
        let voltage_v = Self::read_stage_voltage(device, stage).ok();
        let power_info = Self::get_stage_power_info(device, stage).ok();
        let ready_for_firing = Self::assess_stage_readiness(device, stage);
        
        let duration = start_time.elapsed().as_millis() as u64;
        
        let data = DeviceOperationData::StageInfo {
            stage_number: stage,
            current_ma,
            voltage_v,
            power_info,
            ready_for_firing,
        };
        
        let message = Self::format_stage_message(stage, &data);
        
        Ok(OperationResponse::success_with_duration(
            data,
            message,
            "get_stage_data".to_string(),
            duration,
        ).with_context("operation".to_string(), "stage_data_retrieval".to_string())
         .with_context("stage".to_string(), stage.to_string()))
    }

    /// Read stage parameters using unified operation pattern
    ///
    /// This function provides centralized stage parameter reading
    /// used across all stage information operations.
    ///
    /// # Arguments
    /// * `device` - Mutable reference to the device for parameter reading
    /// * `stage` - Stage number to query (1-5)
    ///
    /// # Returns
    /// * `OperationResult<DeviceOperationData>` - Structured operation result
    ///
    /// # Example
    /// ```
    /// let response = StageInfoOperations::read_stage_parameters_unified(&mut device, 3)?;
    /// ```
    pub fn read_stage_parameters_unified(
        device: &mut LumidoxDevice,
        stage: u8
    ) -> OperationResult<DeviceOperationData> {
        let start_time = Instant::now();
        
        // Validate stage number first
        Self::validate_stage_number(stage)?;
        
        // Read stage parameters
        let current_ma = Self::read_stage_current(device, stage).ok();
        let ready_for_firing = Self::assess_stage_readiness(device, stage);
        
        let duration = start_time.elapsed().as_millis() as u64;
        
        let data = DeviceOperationData::StageInfo {
            stage_number: stage,
            current_ma,
            voltage_v: None,
            power_info: Some(format!("Stage {} parameters", stage)),
            ready_for_firing,
        };
        
        let message = format!("Stage {} parameters: Current = {:?}mA", 
            stage, 
            current_ma.unwrap_or(0)
        );
        
        Ok(OperationResponse::success_with_duration(
            data,
            message,
            "read_stage_parameters".to_string(),
            duration,
        ).with_context("operation".to_string(), "stage_parameter_reading".to_string())
         .with_context("stage".to_string(), stage.to_string()))
    }

    /// Get firing readiness using unified operation pattern
    ///
    /// This function provides centralized firing readiness assessment
    /// used across all stage operations.
    ///
    /// # Arguments
    /// * `device` - Mutable reference to the device for readiness assessment
    /// * `stage` - Stage number to assess (1-5)
    ///
    /// # Returns
    /// * `OperationResult<DeviceOperationData>` - Structured operation result
    ///
    /// # Example
    /// ```
    /// let response = StageInfoOperations::get_firing_readiness_unified(&mut device, 1)?;
    /// ```
    pub fn get_firing_readiness_unified(
        device: &mut LumidoxDevice,
        stage: u8
    ) -> OperationResult<DeviceOperationData> {
        let start_time = Instant::now();
        
        // Validate stage number first
        Self::validate_stage_number(stage)?;
        
        let ready_for_firing = Self::assess_stage_readiness(device, stage);
        let current_ma = Self::read_stage_current(device, stage).ok();
        
        let duration = start_time.elapsed().as_millis() as u64;
        
        let data = DeviceOperationData::StageInfo {
            stage_number: stage,
            current_ma,
            voltage_v: None,
            power_info: Some(format!("Firing readiness assessment")),
            ready_for_firing,
        };
        
        let message = format!("Stage {} firing readiness: {}", 
            stage, 
            if ready_for_firing { "Ready" } else { "Not Ready" }
        );
        
        Ok(OperationResponse::success_with_duration(
            data,
            message,
            "get_firing_readiness".to_string(),
            duration,
        ).with_context("operation".to_string(), "firing_readiness_assessment".to_string())
         .with_context("stage".to_string(), stage.to_string()))
    }

    /// Validate stage number
    ///
    /// Provides centralized validation logic for stage numbers
    /// used across all stage operations.
    ///
    /// # Arguments
    /// * `stage` - Stage number to validate
    ///
    /// # Returns
    /// * `Result<()>` - Success if valid, error if invalid
    pub fn validate_stage_number(stage: u8) -> crate::core::Result<()> {
        if !(1..=5).contains(&stage) {
            return Err(LumidoxError::InvalidInput(
                format!("Invalid stage number: {}. Must be 1-5", stage)
            ));
        }
        Ok(())
    }

    /// Read stage current (placeholder implementation)
    ///
    /// Reads the current setting for a specific stage.
    ///
    /// # Arguments
    /// * `device` - Device reference
    /// * `stage` - Stage number
    ///
    /// # Returns
    /// * `Result<u16>` - Stage current in mA
    fn read_stage_current(_device: &mut LumidoxDevice, _stage: u8) -> crate::core::Result<u16> {
        // Placeholder implementation - would use actual device protocol
        Ok(1000) // Default 1A current
    }

    /// Read stage voltage (placeholder implementation)
    ///
    /// Reads the voltage for a specific stage.
    ///
    /// # Arguments
    /// * `device` - Device reference
    /// * `stage` - Stage number
    ///
    /// # Returns
    /// * `Result<f32>` - Stage voltage in V
    fn read_stage_voltage(_device: &mut LumidoxDevice, _stage: u8) -> crate::core::Result<f32> {
        // Placeholder implementation - would use actual device protocol
        Ok(12.0) // Default 12V
    }

    /// Get stage power information (placeholder implementation)
    ///
    /// Gets power information for a specific stage.
    ///
    /// # Arguments
    /// * `device` - Device reference
    /// * `stage` - Stage number
    ///
    /// # Returns
    /// * `Result<String>` - Power information string
    fn get_stage_power_info(_device: &mut LumidoxDevice, stage: u8) -> crate::core::Result<String> {
        // Placeholder implementation - would use actual device protocol
        Ok(format!("Stage {} power: 12W", stage))
    }

    /// Assess stage readiness for firing
    ///
    /// Determines if a stage is ready for firing operations.
    ///
    /// # Arguments
    /// * `device` - Device reference
    /// * `stage` - Stage number
    ///
    /// # Returns
    /// * `bool` - True if ready for firing, false otherwise
    fn assess_stage_readiness(device: &LumidoxDevice, _stage: u8) -> bool {
        // Check if device is in a state that allows firing
        match device.current_mode() {
            Some(mode) => {
                use crate::device::models::DeviceMode;
                matches!(mode, DeviceMode::Armed | DeviceMode::Remote)
            }
            None => false,
        }
    }

    /// Format stage message based on stage data
    ///
    /// Creates a human-readable stage message from the stage data
    /// for consistent messaging across interfaces.
    ///
    /// # Arguments
    /// * `stage` - Stage number
    /// * `data` - Stage data to format
    ///
    /// # Returns
    /// * `String` - Formatted stage message
    fn format_stage_message(stage: u8, data: &DeviceOperationData) -> String {
        if let DeviceOperationData::StageInfo { 
            current_ma, 
            voltage_v, 
            ready_for_firing,
            .. 
        } = data {
            let current_str = current_ma.map(|c| c.to_string()).unwrap_or_else(|| "N/A".to_string());
            let voltage_str = voltage_v.map(|v| format!("{:.1}", v)).unwrap_or_else(|| "N/A".to_string());
            let ready_str = if *ready_for_firing { "Ready" } else { "Not Ready" };
            
            format!(
                "Stage {} Info: Current={}mA, Voltage={}V, Status={}",
                stage, current_str, voltage_str, ready_str
            )
        } else {
            format!("Stage {} information retrieved successfully", stage)
        }
    }
}
