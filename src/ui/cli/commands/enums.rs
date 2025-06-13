//! Command category enumerations for Lumidox II Controller CLI
//!
//! This module defines enumerations that categorize commands by their
//! operational domain and execution characteristics.

use super::args::Commands;

/// Categories of command operations based on their primary function
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommandCategory {
    /// Device control operations (firing, arming, power control)
    DeviceControl,
    /// Information retrieval operations (status, device info, parameters)
    Information,
    /// Parameter management operations (reading/setting currents, stage parameters)
    Parameters,
    /// Port management operations (detection, testing, diagnostics)
    PortManagement,
}

/// Sub-categories for device control operations
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeviceControlCategory {
    /// Stage firing operations (Stage1-5)
    StageFiring,
    /// Custom current control operations
    CurrentControl,
    /// Power state control operations (Arm, Off)
    PowerControl,
}

/// Sub-categories for information retrieval operations
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InformationCategory {
    /// Device information operations (Info)
    DeviceInfo,
    /// Status reading operations (Status)
    StatusReading,
    /// State reading operations (ReadState)
    StateReading,
}

/// Sub-categories for parameter management operations
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParameterCategory {
    /// Current setting operations (ReadArmCurrent, ReadFireCurrent, SetArmCurrent)
    CurrentSettings,
    /// Stage parameter operations (StageInfo, StageArm, StageVoltages)
    StageParameters,
}

/// Sub-categories for port management operations
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PortManagementCategory {
    /// Port detection operations (DetectPorts)
    Detection,
    /// Baud rate testing operations (TestBaud)
    Testing,
    /// Port diagnostics operations (PortDiagnostics)
    Diagnostics,
}

/// Command execution priority levels
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum CommandPriority {
    /// Low priority commands (information retrieval)
    Low,
    /// Normal priority commands (parameter operations)
    Normal,
    /// High priority commands (device control)
    High,
    /// Critical priority commands (safety operations)
    Critical,
}

/// Command safety levels for risk assessment
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommandSafetyLevel {
    /// Safe operations with no device state changes
    Safe,
    /// Low risk operations with minimal device impact
    LowRisk,
    /// Medium risk operations with device state changes
    MediumRisk,
    /// High risk operations that could affect device operation
    HighRisk,
}

/// Command execution requirements
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommandRequirement {
    /// Requires active device connection
    DeviceConnection,
    /// Requires device to be in specific state
    DeviceState,
    /// Requires user confirmation
    UserConfirmation,
    /// Requires elevated privileges
    ElevatedPrivileges,
}

impl CommandCategory {
    /// Determine the category of a command
    pub fn from_command(command: &Commands) -> Self {
        match command {
            Commands::Stage1 | Commands::Stage2 | Commands::Stage3 | 
            Commands::Stage4 | Commands::Stage5 | Commands::Current { .. } |
            Commands::Arm | Commands::Off => Self::DeviceControl,
            
            Commands::Info | Commands::Status | Commands::ReadState => Self::Information,
            
            Commands::ReadArmCurrent | Commands::ReadFireCurrent | 
            Commands::SetArmCurrent { .. } | Commands::StageInfo { .. } |
            Commands::StageArm { .. } | Commands::StageVoltages { .. } => Self::Parameters,
            
            Commands::ListPorts | Commands::DetectPorts | 
            Commands::TestBaud { .. } | Commands::PortDiagnostics => Self::PortManagement,
        }
    }

    /// Get the display name for the category
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::DeviceControl => "Device Control",
            Self::Information => "Information Retrieval",
            Self::Parameters => "Parameter Management",
            Self::PortManagement => "Port Management",
        }
    }

    /// Get the description for the category
    pub fn description(&self) -> &'static str {
        match self {
            Self::DeviceControl => "Operations that control device behavior and state",
            Self::Information => "Operations that retrieve device information and status",
            Self::Parameters => "Operations that manage device parameters and settings",
            Self::PortManagement => "Operations that manage serial port connections",
        }
    }
}

impl DeviceControlCategory {
    /// Determine the device control sub-category of a command
    pub fn from_command(command: &Commands) -> Option<Self> {
        match command {
            Commands::Stage1 | Commands::Stage2 | Commands::Stage3 | 
            Commands::Stage4 | Commands::Stage5 => Some(Self::StageFiring),
            
            Commands::Current { .. } => Some(Self::CurrentControl),
            
            Commands::Arm | Commands::Off => Some(Self::PowerControl),
            
            _ => None,
        }
    }
}

impl InformationCategory {
    /// Determine the information sub-category of a command
    pub fn from_command(command: &Commands) -> Option<Self> {
        match command {
            Commands::Info => Some(Self::DeviceInfo),
            Commands::Status => Some(Self::StatusReading),
            Commands::ReadState => Some(Self::StateReading),
            _ => None,
        }
    }
}

impl ParameterCategory {
    /// Determine the parameter sub-category of a command
    pub fn from_command(command: &Commands) -> Option<Self> {
        match command {
            Commands::ReadArmCurrent | Commands::ReadFireCurrent | 
            Commands::SetArmCurrent { .. } => Some(Self::CurrentSettings),
            
            Commands::StageInfo { .. } | Commands::StageArm { .. } | 
            Commands::StageVoltages { .. } => Some(Self::StageParameters),
            
            _ => None,
        }
    }
}

impl PortManagementCategory {
    /// Determine the port management sub-category of a command
    pub fn from_command(command: &Commands) -> Option<Self> {
        match command {
            Commands::DetectPorts => Some(Self::Detection),
            Commands::TestBaud { .. } => Some(Self::Testing),
            Commands::PortDiagnostics => Some(Self::Diagnostics),
            Commands::ListPorts => Some(Self::Detection), // ListPorts is handled elsewhere
            _ => None,
        }
    }
}

impl CommandPriority {
    /// Determine the priority level of a command
    pub fn from_command(command: &Commands) -> Self {
        match command {
            // Critical safety operations
            Commands::Off => Self::Critical,
            
            // High priority device control
            Commands::Stage1 | Commands::Stage2 | Commands::Stage3 | 
            Commands::Stage4 | Commands::Stage5 | Commands::Current { .. } |
            Commands::Arm => Self::High,
            
            // Normal priority parameter operations
            Commands::SetArmCurrent { .. } => Self::Normal,
            
            // Low priority information and diagnostics
            Commands::Info | Commands::Status | Commands::ReadState |
            Commands::ReadArmCurrent | Commands::ReadFireCurrent |
            Commands::StageInfo { .. } | Commands::StageArm { .. } |
            Commands::StageVoltages { .. } | Commands::ListPorts |
            Commands::DetectPorts | Commands::TestBaud { .. } |
            Commands::PortDiagnostics => Self::Low,
        }
    }
}

impl CommandSafetyLevel {
    /// Determine the safety level of a command
    pub fn from_command(command: &Commands) -> Self {
        match command {
            // High risk operations that change device state significantly
            Commands::Stage1 | Commands::Stage2 | Commands::Stage3 | 
            Commands::Stage4 | Commands::Stage5 | Commands::Current { .. } => Self::HighRisk,
            
            // Medium risk operations that change device state
            Commands::Arm | Commands::Off | Commands::SetArmCurrent { .. } => Self::MediumRisk,
            
            // Low risk operations with minimal impact
            Commands::ReadArmCurrent | Commands::ReadFireCurrent => Self::LowRisk,
            
            // Safe operations with no device state changes
            Commands::Info | Commands::Status | Commands::ReadState |
            Commands::StageInfo { .. } | Commands::StageArm { .. } |
            Commands::StageVoltages { .. } | Commands::ListPorts |
            Commands::DetectPorts | Commands::TestBaud { .. } |
            Commands::PortDiagnostics => Self::Safe,
        }
    }

    /// Check if the command requires user confirmation
    pub fn requires_confirmation(&self) -> bool {
        matches!(self, Self::HighRisk | Self::MediumRisk)
    }
}

impl CommandRequirement {
    /// Get all requirements for a command
    pub fn from_command(command: &Commands) -> Vec<Self> {
        let mut requirements = Vec::new();

        // Most commands require device connection except port management
        match command {
            Commands::ListPorts | Commands::DetectPorts | 
            Commands::TestBaud { .. } | Commands::PortDiagnostics => {
                // Port management commands don't require device connection
            }
            _ => {
                requirements.push(Self::DeviceConnection);
            }
        }

        // High-risk commands require confirmation
        if CommandSafetyLevel::from_command(command).requires_confirmation() {
            requirements.push(Self::UserConfirmation);
        }

        requirements
    }
}
