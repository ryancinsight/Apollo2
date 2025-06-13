//! Operations sub-module for command execution
//!
//! This module organizes command operations into specialized sub-modules
//! following the deep hierarchical structure principle:
//!
//! ## Module Structure (6+ levels deep)
//! ```
//! src/ui/cli/commands/operations/          (Level 5)
//! ├── device_control/                      (Level 6)
//! │   ├── stage_firing.rs                  (Level 7) - Stage firing operations
//! │   ├── current_control.rs               (Level 7) - Custom current control
//! │   └── power_control.rs                 (Level 7) - Power state control
//! ├── information/                         (Level 6)
//! │   ├── device_info.rs                   (Level 7) - Device information retrieval
//! │   ├── status_reading.rs                (Level 7) - Status reading operations
//! │   └── state_reading.rs                 (Level 7) - State reading operations
//! ├── parameters/                          (Level 6)
//! │   ├── current_settings.rs              (Level 7) - Current setting operations
//! │   └── stage_parameters.rs              (Level 7) - Stage parameter operations
//! └── port_management/                     (Level 6)
//!     ├── detection.rs                     (Level 7) - Port detection operations
//!     ├── testing.rs                       (Level 7) - Baud rate testing
//!     └── diagnostics.rs                   (Level 7) - Port diagnostics
//! ```
//!
//! Each sub-module follows the prescribed schema with single responsibility
//! and maintains <150 lines per file.

pub mod device_control;
pub mod information;
pub mod parameters;
pub mod port_management;

// Re-export commonly used items for convenience
pub use device_control::{
    StageFiringOperations, CurrentControlOperations, PowerControlOperations
};
pub use information::{
    DeviceInfoOperations, StatusReadingOperations, StateReadingOperations
};
pub use parameters::{
    CurrentSettingsOperations, StageParametersOperations
};
pub use port_management::{
    PortDetectionOperations, BaudTestingOperations, PortDiagnosticsOperations
};

use crate::core::Result;
use crate::device::LumidoxDevice;
use super::{
    args::Commands,
    types::{CommandExecutionContext, CommandExecutionResult, CommandExecutionConfig},
    enums::CommandCategory,
    traits::CommandExecutor,
};

/// Main operations coordinator that delegates to specialized sub-modules
pub struct OperationsCoordinator;

impl OperationsCoordinator {
    /// Create a new operations coordinator
    pub fn new() -> Self {
        Self
    }

    /// Execute a command by delegating to the appropriate sub-module
    pub fn execute_command(
        &self,
        command: &Commands,
        context: &mut CommandExecutionContext,
        config: &CommandExecutionConfig,
    ) -> Result<CommandExecutionResult> {
        match CommandCategory::from_command(command) {
            CommandCategory::DeviceControl => {
                self.execute_device_control_command(command, context, config)
            }
            CommandCategory::Information => {
                self.execute_information_command(command, context, config)
            }
            CommandCategory::Parameters => {
                self.execute_parameters_command(command, context, config)
            }
            CommandCategory::PortManagement => {
                self.execute_port_management_command(command, context, config)
            }
        }
    }

    /// Execute device control commands
    fn execute_device_control_command(
        &self,
        command: &Commands,
        context: &mut CommandExecutionContext,
        config: &CommandExecutionConfig,
    ) -> Result<CommandExecutionResult> {
        match command {
            Commands::Stage1 | Commands::Stage2 | Commands::Stage3 | 
            Commands::Stage4 | Commands::Stage5 => {
                let executor = StageFiringOperations::new();
                executor.execute(command, context, config)
            }
            Commands::Current { .. } => {
                let executor = CurrentControlOperations::new();
                executor.execute(command, context, config)
            }
            Commands::Arm | Commands::Off => {
                let executor = PowerControlOperations::new();
                executor.execute(command, context, config)
            }
            _ => unreachable!("Non-device-control command passed to device control handler"),
        }
    }

    /// Execute information retrieval commands
    fn execute_information_command(
        &self,
        command: &Commands,
        context: &mut CommandExecutionContext,
        config: &CommandExecutionConfig,
    ) -> Result<CommandExecutionResult> {
        match command {
            Commands::Info => {
                let executor = DeviceInfoOperations::new();
                executor.execute(command, context, config)
            }
            Commands::Status => {
                let executor = StatusReadingOperations::new();
                executor.execute(command, context, config)
            }
            Commands::ReadState => {
                let executor = StateReadingOperations::new();
                executor.execute(command, context, config)
            }
            _ => unreachable!("Non-information command passed to information handler"),
        }
    }

    /// Execute parameter management commands
    fn execute_parameters_command(
        &self,
        command: &Commands,
        context: &mut CommandExecutionContext,
        config: &CommandExecutionConfig,
    ) -> Result<CommandExecutionResult> {
        match command {
            Commands::ReadArmCurrent | Commands::ReadFireCurrent | 
            Commands::SetArmCurrent { .. } => {
                let executor = CurrentSettingsOperations::new();
                executor.execute(command, context, config)
            }
            Commands::StageInfo { .. } | Commands::StageArm { .. } | 
            Commands::StageVoltages { .. } => {
                let executor = StageParametersOperations::new();
                executor.execute(command, context, config)
            }
            _ => unreachable!("Non-parameter command passed to parameter handler"),
        }
    }

    /// Execute port management commands
    fn execute_port_management_command(
        &self,
        command: &Commands,
        context: &mut CommandExecutionContext,
        config: &CommandExecutionConfig,
    ) -> Result<CommandExecutionResult> {
        match command {
            Commands::DetectPorts => {
                let executor = PortDetectionOperations::new();
                executor.execute(command, context, config)
            }
            Commands::TestBaud { .. } => {
                let executor = BaudTestingOperations::new();
                executor.execute(command, context, config)
            }
            Commands::PortDiagnostics => {
                let executor = PortDiagnosticsOperations::new();
                executor.execute(command, context, config)
            }
            Commands::ListPorts => {
                // ListPorts is handled elsewhere, but included for completeness
                unreachable!("ListPorts command should be handled before reaching operations")
            }
            _ => unreachable!("Non-port-management command passed to port management handler"),
        }
    }

    /// Check if a command can be handled by this coordinator
    pub fn can_handle(&self, command: &Commands) -> bool {
        matches!(
            CommandCategory::from_command(command),
            CommandCategory::DeviceControl | 
            CommandCategory::Information | 
            CommandCategory::Parameters | 
            CommandCategory::PortManagement
        )
    }

    /// Get the appropriate sub-module for a command category
    pub fn get_handler_name(&self, command: &Commands) -> &'static str {
        match CommandCategory::from_command(command) {
            CommandCategory::DeviceControl => "Device Control Operations",
            CommandCategory::Information => "Information Operations",
            CommandCategory::Parameters => "Parameter Operations",
            CommandCategory::PortManagement => "Port Management Operations",
        }
    }
}

impl Default for OperationsCoordinator {
    fn default() -> Self {
        Self::new()
    }
}

/// Trait for operation validation
pub trait OperationValidator {
    /// Validate that an operation can be performed
    fn validate_operation(&self, command: &Commands, device: &LumidoxDevice) -> Result<()>;
    
    /// Check if the operation requires device connection
    fn requires_device_connection(&self, command: &Commands) -> bool;
    
    /// Check if the operation is safe to perform
    fn is_safe_operation(&self, command: &Commands) -> bool;
}

/// Trait for operation execution
pub trait OperationExecutor {
    /// Execute the operation
    fn execute_operation(
        &self,
        command: &Commands,
        device: &mut LumidoxDevice,
        config: &CommandExecutionConfig,
    ) -> Result<CommandExecutionResult>;
    
    /// Get the operation name
    fn operation_name(&self) -> &'static str;
    
    /// Get the operation description
    fn operation_description(&self) -> &'static str;
}
