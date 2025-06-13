//! State reading operations for Lumidox II Controller
//!
//! This module handles device state reading operations with
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

/// State reading operations handler
pub struct StateReadingOperations;

impl StateReadingOperations {
    /// Create a new state reading operations handler
    pub fn new() -> Self {
        Self
    }

    /// Validate state reading operation
    fn validate_state_reading(&self, device: &LumidoxDevice) -> Result<()> {
        // Validate device is ready for remote mode operations
        InformationValidator::validate_remote_mode(device)?;
        
        // Additional validation for state requests
        InformationValidator::validate_state_request()?;
        
        Ok(())
    }

    /// Execute state reading operation
    fn execute_state_reading(
        &self,
        device: &mut LumidoxDevice,
    ) -> Result<CommandExecutionResult> {
        println!("Reading remote mode state...");
        
        // Read remote mode state
        match device.read_remote_mode() {
            Ok(mode) => {
                let mode_description = format!("{:?}", mode);
                println!("Remote Mode State: {}", mode_description);

                // Create state data
                let state_data = CommandResultData::RemoteMode {
                    mode_description: mode_description.clone(),
                };

                let message = format!("Remote mode state read successfully: {}", mode_description);
                Ok(CommandExecutionResult::success_with_data(state_data))
            }
            Err(e) => {
                let error_msg = format!("Error reading remote mode state: {}", e);
                println!("{}", error_msg);
                Ok(CommandExecutionResult::failure(error_msg))
            }
        }
    }

    /// Get detailed state information
    fn get_detailed_state_info(&self, device: &mut LumidoxDevice) -> Result<StateReport> {
        let mut report = StateReport::new();

        // Remote mode state
        match device.read_remote_mode() {
            Ok(mode) => {
                report.remote_mode = Some(format!("{:?}", mode));
                report.remote_mode_raw = Some(mode);
            }
            Err(e) => report.errors.push(format!("Remote mode error: {}", e)),
        }

        // Device operational state
        match device.read_device_state() {
            Ok(state) => report.device_state = Some(state),
            Err(e) => report.errors.push(format!("Device state error: {}", e)),
        }

        // Connection state
        report.connection_state = self.get_connection_state(device);

        // Communication state
        report.communication_state = self.get_communication_state(device);

        Ok(report)
    }

    /// Get connection state
    fn get_connection_state(&self, device: &LumidoxDevice) -> ConnectionState {
        if device.info().is_some() {
            ConnectionState::Connected
        } else {
            ConnectionState::Disconnected
        }
    }

    /// Get communication state
    fn get_communication_state(&self, device: &mut LumidoxDevice) -> CommunicationState {
        // Try a simple read operation to test communication
        match device.read_device_state() {
            Ok(_) => CommunicationState::Active,
            Err(_) => CommunicationState::Failed,
        }
    }

    /// Parse remote mode state for detailed analysis
    fn parse_remote_mode_state(&self, mode_str: &str) -> RemoteModeDetails {
        let mut details = RemoteModeDetails::new();

        // Parse the mode string to extract details
        if mode_str.contains("Off") {
            details.power_state = PowerState::Off;
        } else if mode_str.contains("On") {
            details.power_state = PowerState::On;
        } else {
            details.power_state = PowerState::Unknown;
        }

        if mode_str.contains("Armed") || mode_str.contains("Arm") {
            details.arm_state = ArmState::Armed;
        } else if mode_str.contains("Disarmed") || mode_str.contains("Off") {
            details.arm_state = ArmState::Disarmed;
        } else {
            details.arm_state = ArmState::Unknown;
        }

        if mode_str.contains("Fire") || mode_str.contains("Firing") {
            details.firing_state = FiringState::Firing;
        } else if mode_str.contains("Ready") {
            details.firing_state = FiringState::Ready;
        } else {
            details.firing_state = FiringState::Idle;
        }

        details
    }

    /// Check if device is in safe state
    fn is_device_in_safe_state(&self, report: &StateReport) -> bool {
        matches!(report.connection_state, ConnectionState::Connected) &&
        matches!(report.communication_state, CommunicationState::Active) &&
        report.errors.is_empty()
    }

    /// Get state summary for quick display
    fn get_state_summary(&self, report: &StateReport) -> String {
        if self.is_device_in_safe_state(report) {
            if let Some(mode) = &report.remote_mode {
                format!("Device connected - Remote mode: {}", mode)
            } else {
                "Device connected - Remote mode unknown".to_string()
            }
        } else if matches!(report.connection_state, ConnectionState::Disconnected) {
            "Device disconnected".to_string()
        } else if !report.errors.is_empty() {
            format!("Device has {} error(s)", report.errors.len())
        } else {
            "Device state unknown".to_string()
        }
    }

    /// Format state report for display
    fn format_state_display(&self, report: &StateReport) -> Vec<String> {
        let mut lines = Vec::new();

        lines.push("=== Device State Report ===".to_string());

        // Connection state
        lines.push(format!("Connection: {:?}", report.connection_state));

        // Communication state
        lines.push(format!("Communication: {:?}", report.communication_state));

        // Remote mode
        if let Some(mode) = &report.remote_mode {
            lines.push(format!("Remote Mode: {}", mode));
        }

        // Device state
        if let Some(state) = &report.device_state {
            lines.push(format!("Device State: {}", state));
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

    /// Get recommended actions based on state
    fn get_recommended_actions(&self, report: &StateReport) -> Vec<&'static str> {
        let mut actions = Vec::new();

        if matches!(report.connection_state, ConnectionState::Disconnected) {
            actions.push("Check device connection");
            actions.push("Verify COM port settings");
        }

        if matches!(report.communication_state, CommunicationState::Failed) {
            actions.push("Check communication settings");
            actions.push("Try reconnecting to device");
        }

        if !report.errors.is_empty() {
            actions.push("Review error messages");
            actions.push("Check device status");
        }

        if actions.is_empty() {
            actions.push("Device state is normal");
        }

        actions
    }
}

impl Default for StateReadingOperations {
    fn default() -> Self {
        Self::new()
    }
}

impl CommandExecutor for StateReadingOperations {
    fn execute(
        &self,
        command: &Commands,
        context: &mut CommandExecutionContext,
        _config: &CommandExecutionConfig,
    ) -> Result<CommandExecutionResult> {
        // Validate this is a state reading command
        if !matches!(command, Commands::ReadState) {
            return Err(CommandError::invalid_parameters(
                command,
                "Not a state reading command"
            ).into());
        }

        // Validate operation
        self.validate_state_reading(context.device())?;
        
        // Execute the state reading operation
        self.execute_state_reading(context.device_mut())
    }

    fn can_handle(&self, command: &Commands) -> bool {
        matches!(command, Commands::ReadState)
    }
}

impl super::super::OperationValidator for StateReadingOperations {
    fn validate_operation(&self, command: &Commands, device: &LumidoxDevice) -> Result<()> {
        if !matches!(command, Commands::ReadState) {
            return Err(CommandError::invalid_parameters(
                command,
                "Not a state reading command"
            ).into());
        }
        self.validate_state_reading(device)
    }

    fn requires_device_connection(&self, _command: &Commands) -> bool {
        true
    }

    fn is_safe_operation(&self, _command: &Commands) -> bool {
        true // State reading is always safe
    }
}

impl super::super::OperationExecutor for StateReadingOperations {
    fn execute_operation(
        &self,
        command: &Commands,
        device: &mut LumidoxDevice,
        _config: &CommandExecutionConfig,
    ) -> Result<CommandExecutionResult> {
        if !matches!(command, Commands::ReadState) {
            return Err(CommandError::invalid_parameters(
                command,
                "Not a state reading command"
            ).into());
        }
        self.execute_state_reading(device)
    }

    fn operation_name(&self) -> &'static str {
        "State Reading"
    }

    fn operation_description(&self) -> &'static str {
        "Read remote mode state and device operational status"
    }
}

/// Comprehensive state report
#[derive(Debug, Clone)]
pub struct StateReport {
    pub remote_mode: Option<String>,
    pub remote_mode_raw: Option<crate::device::models::DeviceMode>,
    pub device_state: Option<String>,
    pub connection_state: ConnectionState,
    pub communication_state: CommunicationState,
    pub errors: Vec<String>,
}

impl StateReport {
    pub fn new() -> Self {
        Self {
            remote_mode: None,
            remote_mode_raw: None,
            device_state: None,
            connection_state: ConnectionState::Unknown,
            communication_state: CommunicationState::Unknown,
            errors: Vec::new(),
        }
    }
}

/// Connection state enumeration
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConnectionState {
    Connected,
    Disconnected,
    Unknown,
}

/// Communication state enumeration
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommunicationState {
    Active,
    Failed,
    Unknown,
}

/// Remote mode details for detailed analysis
#[derive(Debug, Clone)]
pub struct RemoteModeDetails {
    pub power_state: PowerState,
    pub arm_state: ArmState,
    pub firing_state: FiringState,
}

impl RemoteModeDetails {
    pub fn new() -> Self {
        Self {
            power_state: PowerState::Unknown,
            arm_state: ArmState::Unknown,
            firing_state: FiringState::Idle,
        }
    }
}

/// Power state enumeration
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PowerState {
    On,
    Off,
    Unknown,
}

/// Arm state enumeration
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArmState {
    Armed,
    Disarmed,
    Unknown,
}

/// Firing state enumeration
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FiringState {
    Idle,
    Ready,
    Firing,
    Unknown,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_handle() {
        let ops = StateReadingOperations::new();
        
        assert!(ops.can_handle(&Commands::ReadState));
        
        assert!(!ops.can_handle(&Commands::Info));
        assert!(!ops.can_handle(&Commands::Status));
        assert!(!ops.can_handle(&Commands::Stage1));
        assert!(!ops.can_handle(&Commands::Arm));
    }

    #[test]
    fn test_requires_device_connection() {
        let ops = StateReadingOperations::new();
        assert!(ops.requires_device_connection(&Commands::ReadState));
    }

    #[test]
    fn test_is_safe_operation() {
        let ops = StateReadingOperations::new();
        assert!(ops.is_safe_operation(&Commands::ReadState));
    }

    #[test]
    fn test_operation_metadata() {
        let ops = StateReadingOperations::new();
        assert_eq!(ops.operation_name(), "State Reading");
        assert!(!ops.operation_description().is_empty());
        assert!(ops.operation_description().contains("remote mode"));
        assert!(ops.operation_description().contains("state"));
    }

    #[test]
    fn test_state_report_creation() {
        let report = StateReport::new();
        assert!(report.remote_mode.is_none());
        assert!(report.device_state.is_none());
        assert_eq!(report.connection_state, ConnectionState::Unknown);
        assert_eq!(report.communication_state, CommunicationState::Unknown);
        assert!(report.errors.is_empty());
    }

    #[test]
    fn test_connection_state() {
        assert_eq!(ConnectionState::Connected, ConnectionState::Connected);
        assert_ne!(ConnectionState::Connected, ConnectionState::Disconnected);
    }

    #[test]
    fn test_communication_state() {
        assert_eq!(CommunicationState::Active, CommunicationState::Active);
        assert_ne!(CommunicationState::Active, CommunicationState::Failed);
    }

    #[test]
    fn test_remote_mode_details() {
        let details = RemoteModeDetails::new();
        assert_eq!(details.power_state, PowerState::Unknown);
        assert_eq!(details.arm_state, ArmState::Unknown);
        assert_eq!(details.firing_state, FiringState::Idle);
    }

    #[test]
    fn test_power_state() {
        assert_eq!(PowerState::On, PowerState::On);
        assert_ne!(PowerState::On, PowerState::Off);
    }

    #[test]
    fn test_arm_state() {
        assert_eq!(ArmState::Armed, ArmState::Armed);
        assert_ne!(ArmState::Armed, ArmState::Disarmed);
    }

    #[test]
    fn test_firing_state() {
        assert_eq!(FiringState::Ready, FiringState::Ready);
        assert_ne!(FiringState::Ready, FiringState::Firing);
    }

    #[test]
    fn test_default_implementation() {
        let ops1 = StateReadingOperations::new();
        let ops2 = StateReadingOperations::default();
        
        // Both should have same behavior
        assert_eq!(ops1.operation_name(), ops2.operation_name());
        assert_eq!(ops1.operation_description(), ops2.operation_description());
        assert_eq!(ops1.can_handle(&Commands::ReadState), ops2.can_handle(&Commands::ReadState));
    }
}
