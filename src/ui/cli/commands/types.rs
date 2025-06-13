//! Command execution types for Lumidox II Controller CLI
//!
//! This module defines the core types used for command execution,
//! including execution contexts, results, and operation metadata.

use crate::core::Result;
use crate::device::LumidoxDevice;
use super::args::Commands;

/// Command execution context containing device and configuration
#[derive(Debug)]
pub struct CommandExecutionContext {
    /// The device controller instance
    pub device: LumidoxDevice,
    /// Whether transition optimizations are enabled
    pub optimize_transitions: bool,
    /// Port name used for connection
    pub port_name: String,
}

/// Result of command execution with optional continuation flag
#[derive(Debug, Clone)]
pub struct CommandExecutionResult {
    /// Whether the command executed successfully
    pub success: bool,
    /// Optional message describing the result
    pub message: Option<String>,
    /// Whether the application should continue running
    pub should_continue: bool,
    /// Optional data returned by the command
    pub data: Option<CommandResultData>,
}

/// Data returned by specific command types
#[derive(Debug, Clone)]
pub enum CommandResultData {
    /// Device information data
    DeviceInfo {
        firmware_version: String,
        model_number: String,
        serial_number: String,
        wavelength: String,
    },
    /// Device status data
    DeviceStatus {
        state_description: String,
        current_summary: String,
    },
    /// Remote mode state data
    RemoteMode {
        mode_description: String,
    },
    /// Current setting data
    CurrentSetting {
        current_ma: u16,
        setting_type: String,
    },
    /// Stage parameter data
    StageParameters {
        stage_number: u8,
        arm_current_ma: u16,
        fire_current_ma: u16,
        volt_limit_v: f32,
        volt_start_v: f32,
        power_total: f32,
        total_units: String,
        power_per_led: f32,
        per_led_units: String,
    },
    /// Port detection results
    PortDetection {
        candidates: Vec<PortCandidate>,
    },
    /// Baud rate test results
    BaudTest {
        results: Vec<BaudTestResult>,
    },
    /// Port diagnostics data
    PortDiagnostics {
        diagnostics: Vec<String>,
    },
}

/// Port candidate information for detection results
#[derive(Debug, Clone)]
pub struct PortCandidate {
    pub port_name: String,
    pub score_reason: String,
    pub compatibility_score: u32,
    pub firmware_version: Option<String>,
    pub model_number: Option<String>,
}

/// Baud rate test result information
#[derive(Debug, Clone)]
pub struct BaudTestResult {
    pub baud_rate: u32,
    pub success: bool,
    pub quality_score: u32,
    pub successful_responses: u32,
    pub total_attempts: u32,
    pub firmware_version: Option<String>,
}

/// Command execution configuration
#[derive(Debug, Clone)]
pub struct CommandExecutionConfig {
    /// Maximum number of retry attempts for operations
    pub max_retries: u8,
    /// Timeout for device operations in milliseconds
    pub operation_timeout_ms: u32,
    /// Whether to display verbose output
    pub verbose: bool,
    /// Whether to confirm destructive operations
    pub confirm_destructive: bool,
}

impl Default for CommandExecutionConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            operation_timeout_ms: 5000,
            verbose: false,
            confirm_destructive: true,
        }
    }
}

impl CommandExecutionResult {
    /// Create a successful result
    pub fn success() -> Self {
        Self {
            success: true,
            message: None,
            should_continue: true,
            data: None,
        }
    }

    /// Create a successful result with message
    pub fn success_with_message(message: String) -> Self {
        Self {
            success: true,
            message: Some(message),
            should_continue: true,
            data: None,
        }
    }

    /// Create a successful result with data
    pub fn success_with_data(data: CommandResultData) -> Self {
        Self {
            success: true,
            message: None,
            should_continue: true,
            data: Some(data),
        }
    }

    /// Create a failure result
    pub fn failure(message: String) -> Self {
        Self {
            success: false,
            message: Some(message),
            should_continue: true,
            data: None,
        }
    }

    /// Create a failure result that should terminate execution
    pub fn fatal_failure(message: String) -> Self {
        Self {
            success: false,
            message: Some(message),
            should_continue: false,
            data: None,
        }
    }
}

impl CommandExecutionContext {
    /// Create a new command execution context
    pub fn new(device: LumidoxDevice, optimize_transitions: bool, port_name: String) -> Self {
        Self {
            device,
            optimize_transitions,
            port_name,
        }
    }

    /// Get a mutable reference to the device
    pub fn device_mut(&mut self) -> &mut LumidoxDevice {
        &mut self.device
    }

    /// Get an immutable reference to the device
    pub fn device(&self) -> &LumidoxDevice {
        &self.device
    }

    /// Check if transition optimizations are enabled
    pub fn is_optimization_enabled(&self) -> bool {
        self.optimize_transitions
    }

    /// Get the port name used for connection
    pub fn port_name(&self) -> &str {
        &self.port_name
    }
}
