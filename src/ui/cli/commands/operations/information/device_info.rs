//! Device information operations for Lumidox II Controller
//!
//! This module handles device information retrieval operations with
//! proper validation, execution, and result handling.

use crate::core::Result;
use crate::device::LumidoxDevice;
use super::super::super::{
    args::Commands,
    types::{CommandExecutionContext, CommandExecutionResult, CommandResultData, CommandExecutionConfig},
    traits::CommandExecutor,
    errors::CommandError,
};
use super::{InformationValidator, InformationResultType};

/// Device information operations handler
pub struct DeviceInfoOperations;

impl DeviceInfoOperations {
    /// Create a new device info operations handler
    pub fn new() -> Self {
        Self
    }

    /// Validate device info operation
    fn validate_device_info(&self, device: &LumidoxDevice) -> Result<()> {
        // Validate device is ready for info operations
        InformationValidator::validate_device_ready(device)?;
        
        // Additional validation for info requests
        InformationValidator::validate_info_request()?;
        
        Ok(())
    }

    /// Execute device info retrieval operation
    fn execute_device_info(
        &self,
        device: &LumidoxDevice,
    ) -> Result<CommandExecutionResult> {
        // Retrieve device information
        if let Some(info) = device.info() {
            // Create device info data
            let device_info_data = CommandResultData::DeviceInfo {
                firmware_version: info.firmware_version.clone(),
                model_number: info.model_number.clone(),
                serial_number: info.serial_number.clone(),
                wavelength: info.wavelength.clone(),
            };

            // Display device information
            self.display_device_info(&info.firmware_version, &info.model_number, 
                                   &info.serial_number, &info.wavelength)?;

            Ok(CommandExecutionResult::success_with_data(device_info_data))
        } else {
            let message = "Device information not available".to_string();
            Ok(CommandExecutionResult::failure(message))
        }
    }

    /// Display device information to console
    fn display_device_info(
        &self,
        firmware_version: &str,
        model_number: &str,
        serial_number: &str,
        wavelength: &str,
    ) -> Result<()> {
        println!("Controller Firmware Version: {}", firmware_version);
        println!("Device Model Number: {}", model_number);
        println!("Device Serial Number: {}", serial_number);
        println!("Device Wavelength: {}", wavelength);
        Ok(())
    }

    /// Get device info summary for logging
    fn get_device_info_summary(
        &self,
        firmware_version: &str,
        model_number: &str,
        serial_number: &str,
        wavelength: &str,
    ) -> String {
        format!(
            "Device: {} (S/N: {}) - Firmware: {} - Wavelength: {}",
            model_number, serial_number, firmware_version, wavelength
        )
    }

    /// Validate device info fields
    fn validate_device_info_fields(
        &self,
        firmware_version: &str,
        model_number: &str,
        serial_number: &str,
        wavelength: &str,
    ) -> Result<()> {
        if firmware_version.is_empty() {
            return Err(CommandError::execution_failed(
                &Commands::Info,
                "Firmware version is empty"
            ).into());
        }

        if model_number.is_empty() {
            return Err(CommandError::execution_failed(
                &Commands::Info,
                "Model number is empty"
            ).into());
        }

        if serial_number.is_empty() {
            return Err(CommandError::execution_failed(
                &Commands::Info,
                "Serial number is empty"
            ).into());
        }

        if wavelength.is_empty() {
            return Err(CommandError::execution_failed(
                &Commands::Info,
                "Wavelength is empty"
            ).into());
        }

        Ok(())
    }

    /// Check if device info is complete
    fn is_device_info_complete(&self, device: &LumidoxDevice) -> bool {
        if let Some(info) = device.info() {
            !info.firmware_version.is_empty() &&
            !info.model_number.is_empty() &&
            !info.serial_number.is_empty() &&
            !info.wavelength.is_empty()
        } else {
            false
        }
    }

    /// Get device compatibility information
    fn get_device_compatibility_info(&self, device: &LumidoxDevice) -> Option<String> {
        if let Some(info) = device.info() {
            // Check if this is a known compatible model
            if info.model_number.contains("Lumidox II") {
                Some("Fully compatible Lumidox II Controller".to_string())
            } else if info.model_number.contains("Lumidox") {
                Some("Compatible Lumidox device (legacy model)".to_string())
            } else {
                Some("Unknown device model - compatibility not verified".to_string())
            }
        } else {
            None
        }
    }

    /// Get device age estimation based on firmware version
    fn estimate_device_age(&self, firmware_version: &str) -> Option<&'static str> {
        // Simple firmware version-based age estimation
        if firmware_version.starts_with("3.") {
            Some("Current generation (2023+)")
        } else if firmware_version.starts_with("2.") {
            Some("Previous generation (2020-2022)")
        } else if firmware_version.starts_with("1.") {
            Some("Legacy generation (pre-2020)")
        } else {
            Some("Unknown generation")
        }
    }

    /// Get recommended firmware version
    fn get_recommended_firmware(&self) -> &'static str {
        "3.2.0 or later"
    }

    /// Check if firmware update is recommended
    fn is_firmware_update_recommended(&self, firmware_version: &str) -> bool {
        // Simple version comparison - recommend update for versions < 3.0.0
        !firmware_version.starts_with("3.")
    }
}

impl Default for DeviceInfoOperations {
    fn default() -> Self {
        Self::new()
    }
}

impl CommandExecutor for DeviceInfoOperations {
    fn execute(
        &self,
        command: &Commands,
        context: &mut CommandExecutionContext,
        _config: &CommandExecutionConfig,
    ) -> Result<CommandExecutionResult> {
        // Validate this is an info command
        if !matches!(command, Commands::Info) {
            return Err(CommandError::invalid_parameters(
                command,
                "Not a device info command"
            ).into());
        }

        // Validate operation
        self.validate_device_info(context.device())?;
        
        // Execute the info retrieval operation
        self.execute_device_info(context.device())
    }

    fn can_handle(&self, command: &Commands) -> bool {
        matches!(command, Commands::Info)
    }
}

impl super::super::OperationValidator for DeviceInfoOperations {
    fn validate_operation(&self, command: &Commands, device: &LumidoxDevice) -> Result<()> {
        if !matches!(command, Commands::Info) {
            return Err(CommandError::invalid_parameters(
                command,
                "Not a device info command"
            ).into());
        }
        self.validate_device_info(device)
    }

    fn requires_device_connection(&self, _command: &Commands) -> bool {
        true
    }

    fn is_safe_operation(&self, _command: &Commands) -> bool {
        true // Device info retrieval is always safe
    }
}

impl super::super::OperationExecutor for DeviceInfoOperations {
    fn execute_operation(
        &self,
        command: &Commands,
        device: &mut LumidoxDevice,
        _config: &CommandExecutionConfig,
    ) -> Result<CommandExecutionResult> {
        if !matches!(command, Commands::Info) {
            return Err(CommandError::invalid_parameters(
                command,
                "Not a device info command"
            ).into());
        }
        self.execute_device_info(device)
    }

    fn operation_name(&self) -> &'static str {
        "Device Information"
    }

    fn operation_description(&self) -> &'static str {
        "Retrieve device information including firmware, model, and serial number"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_handle() {
        let ops = DeviceInfoOperations::new();
        
        assert!(ops.can_handle(&Commands::Info));
        
        assert!(!ops.can_handle(&Commands::Status));
        assert!(!ops.can_handle(&Commands::ReadState));
        assert!(!ops.can_handle(&Commands::Stage1));
        assert!(!ops.can_handle(&Commands::Arm));
    }

    #[test]
    fn test_requires_device_connection() {
        let ops = DeviceInfoOperations::new();
        assert!(ops.requires_device_connection(&Commands::Info));
    }

    #[test]
    fn test_is_safe_operation() {
        let ops = DeviceInfoOperations::new();
        assert!(ops.is_safe_operation(&Commands::Info));
    }

    #[test]
    fn test_operation_metadata() {
        let ops = DeviceInfoOperations::new();
        assert_eq!(ops.operation_name(), "Device Information");
        assert!(!ops.operation_description().is_empty());
        assert!(ops.operation_description().contains("firmware"));
        assert!(ops.operation_description().contains("model"));
        assert!(ops.operation_description().contains("serial"));
    }

    #[test]
    fn test_get_device_info_summary() {
        let ops = DeviceInfoOperations::new();
        
        let summary = ops.get_device_info_summary(
            "3.1.0",
            "Lumidox II Pro",
            "LX2023001",
            "810nm"
        );
        
        assert!(summary.contains("Lumidox II Pro"));
        assert!(summary.contains("LX2023001"));
        assert!(summary.contains("3.1.0"));
        assert!(summary.contains("810nm"));
    }

    #[test]
    fn test_validate_device_info_fields() {
        let ops = DeviceInfoOperations::new();
        
        // Valid fields should pass
        assert!(ops.validate_device_info_fields(
            "3.1.0",
            "Lumidox II",
            "LX001",
            "810nm"
        ).is_ok());
        
        // Empty firmware should fail
        assert!(ops.validate_device_info_fields(
            "",
            "Lumidox II",
            "LX001",
            "810nm"
        ).is_err());
        
        // Empty model should fail
        assert!(ops.validate_device_info_fields(
            "3.1.0",
            "",
            "LX001",
            "810nm"
        ).is_err());
        
        // Empty serial should fail
        assert!(ops.validate_device_info_fields(
            "3.1.0",
            "Lumidox II",
            "",
            "810nm"
        ).is_err());
        
        // Empty wavelength should fail
        assert!(ops.validate_device_info_fields(
            "3.1.0",
            "Lumidox II",
            "LX001",
            ""
        ).is_err());
    }

    #[test]
    fn test_estimate_device_age() {
        let ops = DeviceInfoOperations::new();
        
        assert_eq!(ops.estimate_device_age("3.1.0"), Some("Current generation (2023+)"));
        assert_eq!(ops.estimate_device_age("2.5.0"), Some("Previous generation (2020-2022)"));
        assert_eq!(ops.estimate_device_age("1.9.0"), Some("Legacy generation (pre-2020)"));
        assert_eq!(ops.estimate_device_age("0.5.0"), Some("Unknown generation"));
    }

    #[test]
    fn test_is_firmware_update_recommended() {
        let ops = DeviceInfoOperations::new();
        
        assert!(!ops.is_firmware_update_recommended("3.1.0"));
        assert!(!ops.is_firmware_update_recommended("3.0.0"));
        assert!(ops.is_firmware_update_recommended("2.9.0"));
        assert!(ops.is_firmware_update_recommended("1.5.0"));
    }

    #[test]
    fn test_get_recommended_firmware() {
        let ops = DeviceInfoOperations::new();
        let recommended = ops.get_recommended_firmware();
        assert!(recommended.contains("3."));
        assert!(recommended.contains("later"));
    }

    #[test]
    fn test_default_implementation() {
        let ops1 = DeviceInfoOperations::new();
        let ops2 = DeviceInfoOperations::default();
        
        // Both should have same behavior
        assert_eq!(ops1.operation_name(), ops2.operation_name());
        assert_eq!(ops1.operation_description(), ops2.operation_description());
        assert_eq!(ops1.can_handle(&Commands::Info), ops2.can_handle(&Commands::Info));
    }
}
