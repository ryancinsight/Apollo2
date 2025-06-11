//! Device control operations for Lumidox II Controller
//!
//! This module handles device control operations including firing, arming,
//! shutdown, and other control functions. It provides high-level control
//! interfaces that integrate with the device state management system.
//! 
//! The control operations system provides:
//! - Stage firing with optimization support
//! - Custom current firing operations
//! - Device arming and shutdown procedures
//! - Maximum current capability queries
//! - Integration with state management and optimization settings

use crate::core::Result;
use crate::device::models::DeviceMode;
use crate::device::operations::control;

/// Device control operations utilities and functionality
pub struct DeviceControlOperations;

impl DeviceControlOperations {
    /// Arm the device for firing operations
    /// 
    /// Prepares the device for firing by setting it to armed mode.
    /// This is a prerequisite for safe firing operations and updates
    /// the internal state tracking accordingly.
    /// 
    /// # Arguments
    /// * `device` - Mutable reference to the device controller
    /// 
    /// # Returns
    /// * `Result<()>` - Success or arming error
    /// 
    /// # State Changes
    /// - Device mode: Changed to Armed
    /// - Hardware state: Device prepared for firing
    /// - Internal tracking: Updated to reflect armed state
    /// 
    /// # Example
    /// ```
    /// DeviceControlOperations::arm_device(&mut device)?;
    /// println!("Device is now armed and ready for firing");
    /// ```
    pub fn arm_device(device: &mut super::super::LumidoxDevice) -> Result<()> {
        control::arm_device(&mut device.protocol)?;
        device.current_mode = Some(DeviceMode::Armed);
        Ok(())
    }
    
    /// Fire a specific stage with optimization support
    /// 
    /// Fires the specified stage using either optimized or standard firing
    /// logic based on the device's optimization settings. Optimized firing
    /// reduces unnecessary state transitions for improved performance.
    /// 
    /// # Arguments
    /// * `device` - Mutable reference to the device controller
    /// * `stage_num` - The stage number to fire (1-5)
    /// 
    /// # Returns
    /// * `Result<()>` - Success or firing error
    /// 
    /// # Optimization Behavior
    /// - When enabled: Uses smart transition logic based on current mode
    /// - When disabled: Always uses full safety sequence
    /// 
    /// # State Changes
    /// - Device mode: Updated to Remote after successful firing
    /// - Hardware state: Specified stage activated
    /// 
    /// # Example
    /// ```
    /// DeviceControlOperations::fire_stage(&mut device, 3)?;
    /// ```
    pub fn fire_stage(device: &mut super::super::LumidoxDevice, stage_num: u8) -> Result<()> {
        if device.optimize_transitions {
            control::fire_stage_smart(&mut device.protocol, stage_num, device.current_mode)?;
        } else {
            control::fire_stage(&mut device.protocol, stage_num)?;
        }
        // Update current mode after firing
        device.current_mode = Some(DeviceMode::Remote);
        Ok(())
    }
    
    /// Fire with a specific current value and optimization support
    /// 
    /// Fires the device with a custom current value using either optimized
    /// or standard firing logic based on the device's optimization settings.
    /// 
    /// # Arguments
    /// * `device` - Mutable reference to the device controller
    /// * `current_ma` - The current value in milliamps to use for firing
    /// 
    /// # Returns
    /// * `Result<()>` - Success or firing error
    /// 
    /// # Current Validation
    /// The current value should be within the device's supported range.
    /// Use get_max_current() to determine the maximum supported current.
    /// 
    /// # State Changes
    /// - Device mode: Updated to Remote after successful firing
    /// - Hardware state: Device firing with specified current
    /// 
    /// # Example
    /// ```
    /// DeviceControlOperations::fire_with_current(&mut device, 2500)?;
    /// ```
    pub fn fire_with_current(device: &mut super::super::LumidoxDevice, current_ma: u16) -> Result<()> {
        if device.optimize_transitions {
            control::fire_with_current_smart(&mut device.protocol, current_ma, device.current_mode)?;
        } else {
            control::fire_with_current(&mut device.protocol, current_ma)?;
        }
        // Update current mode after firing
        device.current_mode = Some(DeviceMode::Remote);
        Ok(())
    }
    
    /// Turn off the device safely
    /// 
    /// Safely turns off the device output while maintaining remote control
    /// capability. This puts the device in standby mode where it's ready
    /// for operation but not actively firing.
    /// 
    /// # Arguments
    /// * `device` - Mutable reference to the device controller
    /// 
    /// # Returns
    /// * `Result<()>` - Success or turn-off error
    /// 
    /// # State Changes
    /// - Device mode: Updated to Standby
    /// - Hardware state: Output disabled, device ready
    /// 
    /// # Example
    /// ```
    /// DeviceControlOperations::turn_off_device(&mut device)?;
    /// ```
    pub fn turn_off_device(device: &mut super::super::LumidoxDevice) -> Result<()> {
        control::turn_off(&mut device.protocol)?;
        device.current_mode = Some(DeviceMode::Standby);
        Ok(())
    }
    
    /// Shutdown device and return to local mode
    /// 
    /// Completely shuts down the device and returns it to local mode
    /// where it must be manually controlled. This is the safest state
    /// for device storage or when remote control is not needed.
    /// 
    /// # Arguments
    /// * `device` - Mutable reference to the device controller
    /// 
    /// # Returns
    /// * `Result<()>` - Success or shutdown error
    /// 
    /// # State Changes
    /// - Device mode: Updated to Local
    /// - Hardware state: Device in local control mode
    /// - Remote control: Disabled until re-initialization
    /// 
    /// # Example
    /// ```
    /// DeviceControlOperations::shutdown_device(&mut device)?;
    /// println!("Device returned to local mode");
    /// ```
    pub fn shutdown_device(device: &mut super::super::LumidoxDevice) -> Result<()> {
        control::shutdown(&mut device.protocol)?;
        device.current_mode = Some(DeviceMode::Local);
        Ok(())
    }
    
    /// Get maximum current capability of the device
    /// 
    /// Queries the device to determine its maximum current capability.
    /// This information is useful for validating current values before
    /// firing operations.
    /// 
    /// # Arguments
    /// * `device` - Mutable reference to the device controller
    /// 
    /// # Returns
    /// * `Result<u16>` - Maximum current in milliamps or query error
    /// 
    /// # Example
    /// ```
    /// let max_current = DeviceControlOperations::get_max_current(&mut device)?;
    /// println!("Device supports up to {}mA", max_current);
    /// ```
    pub fn get_max_current(device: &mut super::super::LumidoxDevice) -> Result<u16> {
        control::get_max_current(&mut device.protocol)
    }
    
    /// Validate firing readiness
    /// 
    /// Checks whether the device is ready for firing operations by
    /// validating current state, mode, and initialization status.
    /// 
    /// # Arguments
    /// * `device` - Reference to the device controller
    /// 
    /// # Returns
    /// * `FiringReadinessResult` - Detailed readiness assessment
    /// 
    /// # Readiness Criteria
    /// - Device is initialized with valid information
    /// - Current mode allows firing (Armed or Remote)
    /// - No conflicting states or errors
    /// 
    /// # Example
    /// ```
    /// let readiness = DeviceControlOperations::validate_firing_readiness(&device);
    /// if readiness.is_ready {
    ///     // Proceed with firing operation
    /// }
    /// ```
    pub fn validate_firing_readiness(
        device: &super::super::LumidoxDevice
    ) -> FiringReadinessResult {
        let has_info = device.info.is_some();
        let mode_allows_firing = matches!(
            device.current_mode, 
            Some(DeviceMode::Armed) | Some(DeviceMode::Remote)
        );
        let is_ready = has_info && mode_allows_firing;
        
        let readiness_issues = if !is_ready {
            let mut issues = Vec::new();
            if !has_info {
                issues.push("Device information not available".to_string());
            }
            if !mode_allows_firing {
                issues.push(format!(
                    "Current mode {:?} does not allow firing", 
                    device.current_mode
                ));
            }
            issues
        } else {
            Vec::new()
        };
        
        FiringReadinessResult {
            is_ready,
            has_device_info: has_info,
            mode_allows_firing,
            current_mode: device.current_mode,
            readiness_issues,
        }
    }
    
    /// Get control operation recommendations
    /// 
    /// Provides recommendations for control operations based on the
    /// current device state and typical operation patterns.
    /// 
    /// # Arguments
    /// * `device` - Reference to the device controller
    /// 
    /// # Returns
    /// * `ControlRecommendations` - Recommended operations and settings
    /// 
    /// # Example
    /// ```
    /// let recommendations = DeviceControlOperations::get_control_recommendations(&device);
    /// for rec in recommendations.recommended_actions {
    ///     println!("Recommended: {}", rec);
    /// }
    /// ```
    pub fn get_control_recommendations(
        device: &super::super::LumidoxDevice
    ) -> ControlRecommendations {
        let mut recommended_actions = Vec::new();
        let mut safety_warnings = Vec::new();
        
        match device.current_mode {
            None => {
                recommended_actions.push("Initialize device first".to_string());
                safety_warnings.push("Device not initialized".to_string());
            },
            Some(DeviceMode::Local) => {
                recommended_actions.push("Set to Standby mode for remote control".to_string());
            },
            Some(DeviceMode::Standby) => {
                recommended_actions.push("Arm device before firing".to_string());
            },
            Some(DeviceMode::Armed) => {
                recommended_actions.push("Device ready for firing operations".to_string());
            },
            Some(DeviceMode::Remote) => {
                recommended_actions.push("Turn off or shutdown when finished".to_string());
                safety_warnings.push("Device may be actively firing".to_string());
            },
        }
        
        if !device.info.is_some() {
            safety_warnings.push("Device information not available".to_string());
        }
        
        ControlRecommendations {
            current_mode: device.current_mode,
            optimization_enabled: device.optimize_transitions,
            recommended_actions,
            safety_warnings,
            is_safe_for_operation: matches!(
                device.current_mode, 
                Some(DeviceMode::Standby) | Some(DeviceMode::Armed)
            ) && device.info.is_some(),
        }
    }
    
    /// Execute safe control sequence
    /// 
    /// Executes a control operation with proper safety checks and
    /// state validation to ensure safe and reliable operation.
    /// 
    /// # Arguments
    /// * `device` - Mutable reference to the device controller
    /// * `operation` - The control operation to execute
    /// 
    /// # Returns
    /// * `Result<ControlOperationResult>` - Operation results
    /// 
    /// # Example
    /// ```
    /// let result = DeviceControlOperations::execute_safe_control_sequence(
    ///     &mut device, 
    ///     ControlOperation::ArmDevice
    /// )?;
    /// ```
    pub fn execute_safe_control_sequence(
        device: &mut super::super::LumidoxDevice,
        operation: ControlOperation
    ) -> Result<ControlOperationResult> {
        let initial_mode = device.current_mode;
        
        let result = match operation {
            ControlOperation::ArmDevice => {
                Self::arm_device(device).map(|_| "Device armed successfully".to_string())
            },
            ControlOperation::FireStage(stage) => {
                Self::fire_stage(device, stage).map(|_| format!("Stage {} fired successfully", stage))
            },
            ControlOperation::FireWithCurrent(current) => {
                Self::fire_with_current(device, current).map(|_| format!("Fired with {}mA successfully", current))
            },
            ControlOperation::TurnOff => {
                Self::turn_off_device(device).map(|_| "Device turned off successfully".to_string())
            },
            ControlOperation::Shutdown => {
                Self::shutdown_device(device).map(|_| "Device shutdown successfully".to_string())
            },
        };
        
        match result {
            Ok(message) => Ok(ControlOperationResult {
                success: true,
                operation,
                initial_mode,
                final_mode: device.current_mode,
                message,
                error: None,
            }),
            Err(e) => Ok(ControlOperationResult {
                success: false,
                operation,
                initial_mode,
                final_mode: device.current_mode,
                message: "Operation failed".to_string(),
                error: Some(format!("{}", e)),
            }),
        }
    }
}

/// Control operations that can be executed
#[derive(Debug, Clone, PartialEq)]
pub enum ControlOperation {
    /// Arm the device for firing
    ArmDevice,
    /// Fire a specific stage
    FireStage(u8),
    /// Fire with a specific current value
    FireWithCurrent(u16),
    /// Turn off the device
    TurnOff,
    /// Shutdown the device
    Shutdown,
}

/// Result of firing readiness validation
#[derive(Debug, Clone)]
pub struct FiringReadinessResult {
    /// Whether the device is ready for firing
    pub is_ready: bool,
    /// Whether device information is available
    pub has_device_info: bool,
    /// Whether the current mode allows firing
    pub mode_allows_firing: bool,
    /// The current device mode
    pub current_mode: Option<DeviceMode>,
    /// List of issues preventing firing readiness
    pub readiness_issues: Vec<String>,
}

/// Control operation recommendations
#[derive(Debug, Clone)]
pub struct ControlRecommendations {
    /// The current device mode
    pub current_mode: Option<DeviceMode>,
    /// Whether optimization is enabled
    pub optimization_enabled: bool,
    /// List of recommended actions
    pub recommended_actions: Vec<String>,
    /// List of safety warnings
    pub safety_warnings: Vec<String>,
    /// Whether the device is safe for operation
    pub is_safe_for_operation: bool,
}

/// Result of a control operation execution
#[derive(Debug, Clone)]
pub struct ControlOperationResult {
    /// Whether the operation was successful
    pub success: bool,
    /// The operation that was executed
    pub operation: ControlOperation,
    /// The device mode before the operation
    pub initial_mode: Option<DeviceMode>,
    /// The device mode after the operation
    pub final_mode: Option<DeviceMode>,
    /// Success or status message
    pub message: String,
    /// Error message if the operation failed
    pub error: Option<String>,
}
