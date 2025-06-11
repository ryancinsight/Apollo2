//! Device operation messages for Lumidox II Controller GUI
//!
//! This module provides device operation message types for the Iced message-driven
//! architecture including connection management, stage firing, parameter setting,
//! and device status operations. All messages are designed to integrate with
//! async Command patterns and existing CLI device operations.
//!
//! The device messages system provides:
//! - Device connection and disconnection messages with auto-detection support
//! - Stage firing operations with validation and safety checks
//! - Parameter management (ARM current, FIRE current) with proper validation
//! - Device status and information retrieval operations
//! - Proper async Command integration for non-blocking GUI operations
//! - Complete error handling and result propagation

use crate::device::{LumidoxDevice, models::{DeviceMode, DeviceInfo}};
use crate::core::{LumidoxError, Result};

/// Device operation messages
/// 
/// Defines all device-related operations that can be performed through
/// the GUI with proper async Command integration and error handling.
#[derive(Debug, Clone)]
pub enum DeviceMessage {
    /// Connect to device with specified configuration
    /// 
    /// # Arguments
    /// * `port_name` - Optional specific port name, None for auto-detection
    /// * `auto_detect` - Whether to use automatic port detection
    /// * `verbose` - Enable verbose output during connection
    /// * `optimize_transitions` - Enable optimized stage transitions
    Connect {
        port_name: Option<String>,
        auto_detect: bool,
        verbose: bool,
        optimize_transitions: bool,
    },
    
    /// Disconnect from current device
    Disconnect,
    
    /// Fire specific stage (1-5)
    /// 
    /// # Arguments
    /// * `stage` - Stage number to fire (1-5)
    FireStage { stage: u8 },
    
    /// Fire with custom current value
    /// 
    /// # Arguments
    /// * `current` - Current value in milliamps
    FireCustom { current: u16 },
    
    /// Set ARM current value
    /// 
    /// # Arguments
    /// * `current` - ARM current value in milliamps
    SetArmCurrent { current: u16 },
    
    /// Arm the device for firing operations
    ArmDevice,
    
    /// Turn off the device
    TurnOffDevice,
    
    /// Shutdown device and quit application
    ShutdownDevice,
    
    /// Read current device status and parameters
    ReadDeviceStatus,
    
    /// Read device information (model, serial, firmware)
    ReadDeviceInfo,
    
    /// Read current ARM current setting
    ReadArmCurrent,
    
    /// Read current FIRE current setting
    ReadFireCurrent,
    
    /// Read device remote mode state
    ReadRemoteMode,
    
    /// Read stage-specific parameters
    /// 
    /// # Arguments
    /// * `stage` - Stage number (1-5)
    ReadStageParameters { stage: u8 },
    
    /// Refresh all cached parameters
    RefreshParameters,
}

impl DeviceMessage {
    /// Check if message represents a potentially destructive operation
    /// 
    /// Used for safety checks and confirmation dialogs in the GUI.
    /// 
    /// # Returns
    /// * `bool` - True if operation is potentially destructive
    /// 
    /// # Example
    /// ```
    /// let fire_msg = DeviceMessage::FireStage { stage: 1 };
    /// assert!(fire_msg.is_destructive());
    /// 
    /// let read_msg = DeviceMessage::ReadDeviceStatus;
    /// assert!(!read_msg.is_destructive());
    /// ```
    pub fn is_destructive(&self) -> bool {
        match self {
            DeviceMessage::FireStage { .. } |
            DeviceMessage::FireCustom { .. } |
            DeviceMessage::ArmDevice |
            DeviceMessage::TurnOffDevice |
            DeviceMessage::ShutdownDevice => true,
            _ => false,
        }
    }
    
    /// Check if message requires device connection
    /// 
    /// Used to validate that device operations are only performed
    /// when a device is connected.
    /// 
    /// # Returns
    /// * `bool` - True if operation requires device connection
    /// 
    /// # Example
    /// ```
    /// let connect_msg = DeviceMessage::Connect { 
    ///     port_name: None, auto_detect: true, verbose: false, optimize_transitions: true 
    /// };
    /// assert!(!connect_msg.requires_connection());
    /// 
    /// let fire_msg = DeviceMessage::FireStage { stage: 1 };
    /// assert!(fire_msg.requires_connection());
    /// ```
    pub fn requires_connection(&self) -> bool {
        match self {
            DeviceMessage::Connect { .. } |
            DeviceMessage::Disconnect => false,
            _ => true,
        }
    }
    
    /// Get operation description for user feedback
    /// 
    /// Provides human-readable description of the operation for
    /// status displays and notifications.
    /// 
    /// # Returns
    /// * `String` - Human-readable operation description
    /// 
    /// # Example
    /// ```
    /// let fire_msg = DeviceMessage::FireStage { stage: 3 };
    /// assert_eq!(fire_msg.get_description(), "Fire Stage 3");
    /// ```
    pub fn get_description(&self) -> String {
        match self {
            DeviceMessage::Connect { port_name, auto_detect, .. } => {
                if *auto_detect {
                    "Connect (Auto-detect)".to_string()
                } else if let Some(port) = port_name {
                    format!("Connect to {}", port)
                } else {
                    "Connect".to_string()
                }
            }
            DeviceMessage::Disconnect => "Disconnect".to_string(),
            DeviceMessage::FireStage { stage } => format!("Fire Stage {}", stage),
            DeviceMessage::FireCustom { current } => format!("Fire with {}mA", current),
            DeviceMessage::SetArmCurrent { current } => format!("Set ARM Current to {}mA", current),
            DeviceMessage::ArmDevice => "Arm Device".to_string(),
            DeviceMessage::TurnOffDevice => "Turn Off Device".to_string(),
            DeviceMessage::ShutdownDevice => "Shutdown Device".to_string(),
            DeviceMessage::ReadDeviceStatus => "Read Device Status".to_string(),
            DeviceMessage::ReadDeviceInfo => "Read Device Information".to_string(),
            DeviceMessage::ReadArmCurrent => "Read ARM Current".to_string(),
            DeviceMessage::ReadFireCurrent => "Read FIRE Current".to_string(),
            DeviceMessage::ReadRemoteMode => "Read Remote Mode".to_string(),
            DeviceMessage::ReadStageParameters { stage } => format!("Read Stage {} Parameters", stage),
            DeviceMessage::RefreshParameters => "Refresh Parameters".to_string(),
        }
    }
    
    /// Get safety level for operation
    /// 
    /// Categorizes operations by safety level for appropriate
    /// user interface feedback and confirmation requirements.
    /// 
    /// # Returns
    /// * `SafetyLevel` - Safety level of the operation
    /// 
    /// # Example
    /// ```
    /// let fire_msg = DeviceMessage::FireStage { stage: 1 };
    /// assert_eq!(fire_msg.get_safety_level(), SafetyLevel::High);
    /// ```
    pub fn get_safety_level(&self) -> SafetyLevel {
        match self {
            DeviceMessage::FireStage { .. } |
            DeviceMessage::FireCustom { .. } => SafetyLevel::High,
            DeviceMessage::ArmDevice |
            DeviceMessage::TurnOffDevice |
            DeviceMessage::ShutdownDevice => SafetyLevel::Medium,
            DeviceMessage::SetArmCurrent { .. } => SafetyLevel::Low,
            DeviceMessage::Connect { .. } |
            DeviceMessage::Disconnect => SafetyLevel::Low,
            _ => SafetyLevel::None,
        }
    }
    
    /// Validate message parameters
    /// 
    /// Performs validation on message parameters before execution
    /// to ensure they are within acceptable ranges and formats.
    /// 
    /// # Returns
    /// * `Result<()>` - Ok if parameters are valid, Err with validation message
    /// 
    /// # Example
    /// ```
    /// let valid_msg = DeviceMessage::FireStage { stage: 3 };
    /// assert!(valid_msg.validate().is_ok());
    /// 
    /// let invalid_msg = DeviceMessage::FireStage { stage: 10 };
    /// assert!(invalid_msg.validate().is_err());
    /// ```
    pub fn validate(&self) -> Result<()> {
        match self {
            DeviceMessage::FireStage { stage } => {
                if !(1..=5).contains(stage) {
                    return Err(LumidoxError::InvalidInput(
                        format!("Invalid stage number: {}. Must be 1-5.", stage)
                    ));
                }
            }
            DeviceMessage::FireCustom { current } => {
                if *current == 0 {
                    return Err(LumidoxError::InvalidInput(
                        "Fire current must be greater than 0".to_string()
                    ));
                }
                if *current > 5000 {
                    return Err(LumidoxError::InvalidInput(
                        "Fire current must not exceed 5000mA".to_string()
                    ));
                }
            }
            DeviceMessage::SetArmCurrent { current } => {
                if *current == 0 {
                    return Err(LumidoxError::InvalidInput(
                        "ARM current must be greater than 0".to_string()
                    ));
                }
                if *current > 5000 {
                    return Err(LumidoxError::InvalidInput(
                        "ARM current must not exceed 5000mA".to_string()
                    ));
                }
            }
            DeviceMessage::ReadStageParameters { stage } => {
                if !(1..=5).contains(stage) {
                    return Err(LumidoxError::InvalidInput(
                        format!("Invalid stage number: {}. Must be 1-5.", stage)
                    ));
                }
            }
            _ => {} // Other messages don't require parameter validation
        }
        
        Ok(())
    }
}

/// Safety levels for device operations
/// 
/// Categorizes operations by their potential impact and risk level
/// for appropriate user interface handling and confirmation requirements.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SafetyLevel {
    /// No safety concerns (read operations)
    None,
    /// Low safety impact (configuration changes)
    Low,
    /// Medium safety impact (device state changes)
    Medium,
    /// High safety impact (firing operations)
    High,
}

/// Device operation results
/// 
/// Represents the results of device operations for proper
/// state management and user feedback.
#[derive(Debug, Clone)]
pub enum DeviceOperationResult {
    /// Connection operation result
    ConnectionResult {
        success: bool,
        port_name: Option<String>,
        device_info: Option<DeviceInfo>,
        error: Option<String>,
    },
    
    /// Firing operation result
    FiringResult {
        success: bool,
        operation: String,
        error: Option<String>,
    },
    
    /// Parameter operation result
    ParameterResult {
        success: bool,
        parameter_name: String,
        value: Option<String>,
        error: Option<String>,
    },
    
    /// Status operation result
    StatusResult {
        success: bool,
        device_mode: Option<DeviceMode>,
        arm_current: Option<u16>,
        fire_current: Option<u16>,
        error: Option<String>,
    },
    
    /// General operation result
    GeneralResult {
        success: bool,
        operation: String,
        message: Option<String>,
        error: Option<String>,
    },
}

impl DeviceOperationResult {
    /// Check if operation was successful
    pub fn is_success(&self) -> bool {
        match self {
            DeviceOperationResult::ConnectionResult { success, .. } |
            DeviceOperationResult::FiringResult { success, .. } |
            DeviceOperationResult::ParameterResult { success, .. } |
            DeviceOperationResult::StatusResult { success, .. } |
            DeviceOperationResult::GeneralResult { success, .. } => *success,
        }
    }
    
    /// Get error message if operation failed
    pub fn get_error(&self) -> Option<&str> {
        match self {
            DeviceOperationResult::ConnectionResult { error, .. } |
            DeviceOperationResult::FiringResult { error, .. } |
            DeviceOperationResult::ParameterResult { error, .. } |
            DeviceOperationResult::StatusResult { error, .. } |
            DeviceOperationResult::GeneralResult { error, .. } => error.as_deref(),
        }
    }
    
    /// Get success message for user feedback
    pub fn get_success_message(&self) -> Option<String> {
        match self {
            DeviceOperationResult::ConnectionResult { success: true, port_name, .. } => {
                Some(format!("Connected to device on {}", 
                    port_name.as_deref().unwrap_or("auto-detected port")))
            }
            DeviceOperationResult::FiringResult { success: true, operation, .. } => {
                Some(format!("{} completed successfully", operation))
            }
            DeviceOperationResult::ParameterResult { success: true, parameter_name, value, .. } => {
                if let Some(val) = value {
                    Some(format!("{} set to {}", parameter_name, val))
                } else {
                    Some(format!("{} operation completed", parameter_name))
                }
            }
            DeviceOperationResult::GeneralResult { success: true, message, .. } => {
                message.clone()
            }
            _ => None,
        }
    }
}
