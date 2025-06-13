//! Status reading operations for Lumidox II Controller
//!
//! This module handles device status reading operations with
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

/// Status reading operations handler
pub struct StatusReadingOperations;

impl StatusReadingOperations {
    /// Create a new status reading operations handler
    pub fn new() -> Self {
        Self
    }

    /// Validate status reading operation
    fn validate_status_reading(&self, device: &LumidoxDevice) -> Result<()> {
        // Validate device is ready for status operations
        InformationValidator::validate_device_initialized(device)?;
        
        // Additional validation for status requests
        InformationValidator::validate_status_request()?;
        
        Ok(())
    }

    /// Execute status reading operation
    fn execute_status_reading(
        &self,
        device: &mut LumidoxDevice,
    ) -> Result<CommandExecutionResult> {
        println!("Reading device status...");
        
        // Read device state
        let state_description = match device.read_device_state() {
            Ok(state_desc) => {
                println!("Device State: {}", state_desc);
                state_desc
            }
            Err(e) => {
                let error_msg = format!("Error reading device state: {}", e);
                println!("{}", error_msg);
                return Ok(CommandExecutionResult::failure(error_msg));
            }
        };

        // Read current settings
        let current_summary = match device.read_current_settings() {
            Ok(current_summary) => {
                println!("Current Settings: {}", current_summary);
                current_summary
            }
            Err(e) => {
                let error_msg = format!("Error reading current settings: {}", e);
                println!("{}", error_msg);
                return Ok(CommandExecutionResult::failure(error_msg));
            }
        };

        // Create status data
        let status_data = CommandResultData::DeviceStatus {
            state_description: state_description.clone(),
            current_summary: current_summary.clone(),
        };

        let message = format!("Status read successfully - State: {}, Settings: {}", 
                            state_description, current_summary);
        Ok(CommandExecutionResult::success_with_data(status_data))
    }

    /// Get comprehensive status information
    fn get_comprehensive_status(&self, device: &mut LumidoxDevice) -> Result<StatusReport> {
        let mut report = StatusReport::new();

        // Device state
        match device.read_device_state() {
            Ok(state) => report.device_state = Some(state),
            Err(e) => report.errors.push(format!("Device state error: {}", e)),
        }

        // Current settings
        match device.read_current_settings() {
            Ok(settings) => report.current_settings = Some(settings),
            Err(e) => report.errors.push(format!("Current settings error: {}", e)),
        }

        // Device info
        if let Some(info) = device.info() {
            report.device_info = Some(DeviceInfoSummary {
                model: info.model_number.clone(),
                firmware: info.firmware_version.clone(),
                serial: info.serial_number.clone(),
            });
        }

        // Connection status
        report.connection_status = self.get_connection_status(device);

        // Operational status
        report.operational_status = self.get_operational_status(device);

        Ok(report)
    }

    /// Get connection status
    fn get_connection_status(&self, device: &LumidoxDevice) -> ConnectionStatus {
        if device.info().is_some() {
            ConnectionStatus::Connected
        } else {
            ConnectionStatus::Disconnected
        }
    }

    /// Get operational status
    fn get_operational_status(&self, device: &mut LumidoxDevice) -> OperationalStatus {
        // Try to read device state to determine operational status
        match device.read_device_state() {
            Ok(state) => {
                if state.contains("Ready") || state.contains("Armed") {
                    OperationalStatus::Ready
                } else if state.contains("Error") || state.contains("Fault") {
                    OperationalStatus::Error
                } else if state.contains("Busy") || state.contains("Processing") {
                    OperationalStatus::Busy
                } else {
                    OperationalStatus::Unknown
                }
            }
            Err(_) => OperationalStatus::Error,
        }
    }

    /// Format status for display
    fn format_status_display(&self, report: &StatusReport) -> Vec<String> {
        let mut lines = Vec::new();

        lines.push("=== Device Status Report ===".to_string());

        // Connection status
        lines.push(format!("Connection: {:?}", report.connection_status));

        // Operational status
        lines.push(format!("Operational Status: {:?}", report.operational_status));

        // Device info
        if let Some(info) = &report.device_info {
            lines.push(format!("Device: {} ({})", info.model, info.serial));
            lines.push(format!("Firmware: {}", info.firmware));
        }

        // Device state
        if let Some(state) = &report.device_state {
            lines.push(format!("State: {}", state));
        }

        // Current settings
        if let Some(settings) = &report.current_settings {
            lines.push(format!("Settings: {}", settings));
        }

        // Errors
        if !report.errors.is_empty() {
            lines.push("Errors:".to_string());
            for error in &report.errors {
                lines.push(format!("  - {}", error));
            }
        }

        lines
    }

    /// Check if status indicates device is healthy
    fn is_device_healthy(&self, report: &StatusReport) -> bool {
        matches!(report.connection_status, ConnectionStatus::Connected) &&
        matches!(report.operational_status, OperationalStatus::Ready) &&
        report.errors.is_empty()
    }

    /// Get status summary for quick display
    fn get_status_summary(&self, report: &StatusReport) -> String {
        if self.is_device_healthy(report) {
            "Device is healthy and ready".to_string()
        } else if matches!(report.connection_status, ConnectionStatus::Disconnected) {
            "Device is disconnected".to_string()
        } else if !report.errors.is_empty() {
            format!("Device has {} error(s)", report.errors.len())
        } else {
            format!("Device status: {:?}", report.operational_status)
        }
    }
}

impl Default for StatusReadingOperations {
    fn default() -> Self {
        Self::new()
    }
}

impl CommandExecutor for StatusReadingOperations {
    fn execute(
        &self,
        command: &Commands,
        context: &mut CommandExecutionContext,
        _config: &CommandExecutionConfig,
    ) -> Result<CommandExecutionResult> {
        // Validate this is a status command
        if !matches!(command, Commands::Status) {
            return Err(CommandError::invalid_parameters(
                command,
                "Not a status reading command"
            ).into());
        }

        // Validate operation
        self.validate_status_reading(context.device())?;
        
        // Execute the status reading operation
        self.execute_status_reading(context.device_mut())
    }

    fn can_handle(&self, command: &Commands) -> bool {
        matches!(command, Commands::Status)
    }
}

impl super::super::OperationValidator for StatusReadingOperations {
    fn validate_operation(&self, command: &Commands, device: &LumidoxDevice) -> Result<()> {
        if !matches!(command, Commands::Status) {
            return Err(CommandError::invalid_parameters(
                command,
                "Not a status reading command"
            ).into());
        }
        self.validate_status_reading(device)
    }

    fn requires_device_connection(&self, _command: &Commands) -> bool {
        true
    }

    fn is_safe_operation(&self, _command: &Commands) -> bool {
        true // Status reading is always safe
    }
}

impl super::super::OperationExecutor for StatusReadingOperations {
    fn execute_operation(
        &self,
        command: &Commands,
        device: &mut LumidoxDevice,
        _config: &CommandExecutionConfig,
    ) -> Result<CommandExecutionResult> {
        if !matches!(command, Commands::Status) {
            return Err(CommandError::invalid_parameters(
                command,
                "Not a status reading command"
            ).into());
        }
        self.execute_status_reading(device)
    }

    fn operation_name(&self) -> &'static str {
        "Status Reading"
    }

    fn operation_description(&self) -> &'static str {
        "Read device status including state and current settings"
    }
}

/// Comprehensive status report
#[derive(Debug, Clone)]
pub struct StatusReport {
    pub device_state: Option<String>,
    pub current_settings: Option<String>,
    pub device_info: Option<DeviceInfoSummary>,
    pub connection_status: ConnectionStatus,
    pub operational_status: OperationalStatus,
    pub errors: Vec<String>,
}

impl StatusReport {
    pub fn new() -> Self {
        Self {
            device_state: None,
            current_settings: None,
            device_info: None,
            connection_status: ConnectionStatus::Unknown,
            operational_status: OperationalStatus::Unknown,
            errors: Vec::new(),
        }
    }
}

/// Device information summary for status reports
#[derive(Debug, Clone)]
pub struct DeviceInfoSummary {
    pub model: String,
    pub firmware: String,
    pub serial: String,
}

/// Connection status enumeration
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConnectionStatus {
    Connected,
    Disconnected,
    Unknown,
}

/// Operational status enumeration
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OperationalStatus {
    Ready,
    Busy,
    Error,
    Unknown,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_handle() {
        let ops = StatusReadingOperations::new();
        
        assert!(ops.can_handle(&Commands::Status));
        
        assert!(!ops.can_handle(&Commands::Info));
        assert!(!ops.can_handle(&Commands::ReadState));
        assert!(!ops.can_handle(&Commands::Stage1));
        assert!(!ops.can_handle(&Commands::Arm));
    }

    #[test]
    fn test_requires_device_connection() {
        let ops = StatusReadingOperations::new();
        assert!(ops.requires_device_connection(&Commands::Status));
    }

    #[test]
    fn test_is_safe_operation() {
        let ops = StatusReadingOperations::new();
        assert!(ops.is_safe_operation(&Commands::Status));
    }

    #[test]
    fn test_operation_metadata() {
        let ops = StatusReadingOperations::new();
        assert_eq!(ops.operation_name(), "Status Reading");
        assert!(!ops.operation_description().is_empty());
        assert!(ops.operation_description().contains("status"));
        assert!(ops.operation_description().contains("state"));
        assert!(ops.operation_description().contains("settings"));
    }

    #[test]
    fn test_status_report_creation() {
        let report = StatusReport::new();
        assert!(report.device_state.is_none());
        assert!(report.current_settings.is_none());
        assert!(report.device_info.is_none());
        assert_eq!(report.connection_status, ConnectionStatus::Unknown);
        assert_eq!(report.operational_status, OperationalStatus::Unknown);
        assert!(report.errors.is_empty());
    }

    #[test]
    fn test_connection_status() {
        assert_eq!(ConnectionStatus::Connected, ConnectionStatus::Connected);
        assert_ne!(ConnectionStatus::Connected, ConnectionStatus::Disconnected);
    }

    #[test]
    fn test_operational_status() {
        assert_eq!(OperationalStatus::Ready, OperationalStatus::Ready);
        assert_ne!(OperationalStatus::Ready, OperationalStatus::Error);
    }

    #[test]
    fn test_device_info_summary() {
        let info = DeviceInfoSummary {
            model: "Lumidox II".to_string(),
            firmware: "3.1.0".to_string(),
            serial: "LX001".to_string(),
        };
        
        assert_eq!(info.model, "Lumidox II");
        assert_eq!(info.firmware, "3.1.0");
        assert_eq!(info.serial, "LX001");
    }

    #[test]
    fn test_default_implementation() {
        let ops1 = StatusReadingOperations::new();
        let ops2 = StatusReadingOperations::default();
        
        // Both should have same behavior
        assert_eq!(ops1.operation_name(), ops2.operation_name());
        assert_eq!(ops1.operation_description(), ops2.operation_description());
        assert_eq!(ops1.can_handle(&Commands::Status), ops2.can_handle(&Commands::Status));
    }
}
