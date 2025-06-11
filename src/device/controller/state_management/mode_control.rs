//! Device mode control and state management for Lumidox II Controller
//!
//! This module handles device mode transitions, state tracking, and lifecycle
//! management for the LumidoxDevice controller. It provides centralized control
//! over device operating modes and ensures proper state consistency.
//! 
//! The state management system provides:
//! - Device mode setting and tracking with validation
//! - State transition management with proper sequencing
//! - Mode consistency verification and error handling
//! - Integration with device control operations

use crate::core::Result;
use crate::device::models::DeviceMode;
use crate::device::operations::control;

/// Device state management utilities and functionality
pub struct DeviceStateManager;

impl DeviceStateManager {
    /// Set device operating mode with state tracking
    /// 
    /// Sets the device to the specified operating mode and updates the
    /// internal state tracking to maintain consistency between hardware
    /// and software state representations.
    /// 
    /// # Arguments
    /// * `device` - Mutable reference to the device controller
    /// * `mode` - The target operating mode to set
    /// 
    /// # Returns
    /// * `Result<()>` - Success or mode setting error
    /// 
    /// # Mode Transitions
    /// - Local: Device controlled locally (manual operation)
    /// - Standby: Device on but output disabled (safe state)
    /// - Armed: Device ready for firing operations
    /// - Remote: Device actively firing or in remote control mode
    /// 
    /// # Example
    /// ```
    /// DeviceStateManager::set_device_mode(&mut device, DeviceMode::Standby)?;
    /// ```
    pub fn set_device_mode(
        device: &mut super::super::LumidoxDevice, 
        mode: DeviceMode
    ) -> Result<()> {
        control::set_mode(&mut device.protocol, mode)?;
        device.current_mode = Some(mode);
        Ok(())
    }
    
    /// Get current device mode with validation
    /// 
    /// Retrieves the current device mode from internal state tracking.
    /// This represents the last known mode set through the controller.
    /// 
    /// # Arguments
    /// * `device` - Reference to the device controller
    /// 
    /// # Returns
    /// * `Option<DeviceMode>` - Current mode if set, None if uninitialized
    /// 
    /// # Example
    /// ```
    /// if let Some(mode) = DeviceStateManager::get_current_mode(&device) {
    ///     println!("Device is in {:?} mode", mode);
    /// }
    /// ```
    pub fn get_current_mode(device: &super::super::LumidoxDevice) -> Option<DeviceMode> {
        device.current_mode
    }
    
    /// Validate current device state consistency
    /// 
    /// Performs validation checks to ensure the device state is consistent
    /// and ready for operations. This includes checking mode validity and
    /// state coherence.
    /// 
    /// # Arguments
    /// * `device` - Reference to the device controller to validate
    /// 
    /// # Returns
    /// * `Result<StateValidationResult>` - Validation results
    /// 
    /// # Validation Checks
    /// - Current mode is set and valid
    /// - Device initialization status
    /// - State consistency between components
    /// 
    /// # Example
    /// ```
    /// let validation = DeviceStateManager::validate_state(&device)?;
    /// if validation.is_valid {
    ///     println!("Device state is consistent and ready");
    /// }
    /// ```
    pub fn validate_state(
        device: &super::super::LumidoxDevice
    ) -> Result<StateValidationResult> {
        let has_mode = device.current_mode.is_some();
        let has_info = device.info.is_some();
        let is_initialized = has_mode && has_info;
        
        let mode_description = match device.current_mode {
            Some(mode) => format!("{:?}", mode),
            None => "Uninitialized".to_string(),
        };
        
        Ok(StateValidationResult {
            is_valid: is_initialized,
            has_current_mode: has_mode,
            has_device_info: has_info,
            is_fully_initialized: is_initialized,
            current_mode_description: mode_description,
        })
    }
    
    /// Check if device is ready for firing operations
    /// 
    /// Determines whether the device is in an appropriate state for
    /// firing operations based on current mode and initialization status.
    /// 
    /// # Arguments
    /// * `device` - Reference to the device controller
    /// 
    /// # Returns
    /// * `bool` - True if ready for firing, false otherwise
    /// 
    /// # Ready Conditions
    /// - Device is initialized with valid mode
    /// - Current mode allows firing operations (Armed or Remote)
    /// - Device information is available
    /// 
    /// # Example
    /// ```
    /// if DeviceStateManager::is_ready_for_firing(&device) {
    ///     device.fire_stage(1)?;
    /// } else {
    ///     println!("Device not ready for firing operations");
    /// }
    /// ```
    pub fn is_ready_for_firing(device: &super::super::LumidoxDevice) -> bool {
        match device.current_mode {
            Some(DeviceMode::Armed) | Some(DeviceMode::Remote) => device.info.is_some(),
            _ => false,
        }
    }
    
    /// Check if device is in safe state
    /// 
    /// Determines whether the device is in a safe state where no firing
    /// operations can occur accidentally.
    /// 
    /// # Arguments
    /// * `device` - Reference to the device controller
    /// 
    /// # Returns
    /// * `bool` - True if in safe state, false otherwise
    /// 
    /// # Safe States
    /// - Local mode (manual control only)
    /// - Standby mode (output disabled)
    /// - Uninitialized state
    /// 
    /// # Example
    /// ```
    /// if DeviceStateManager::is_in_safe_state(&device) {
    ///     println!("Device is in safe state");
    /// }
    /// ```
    pub fn is_in_safe_state(device: &super::super::LumidoxDevice) -> bool {
        match device.current_mode {
            Some(DeviceMode::Local) | Some(DeviceMode::Standby) | None => true,
            _ => false,
        }
    }
    
    /// Get recommended next mode based on current state
    /// 
    /// Provides recommendations for the next appropriate mode transition
    /// based on the current device state and typical operation patterns.
    /// 
    /// # Arguments
    /// * `device` - Reference to the device controller
    /// * `intended_operation` - The intended next operation
    /// 
    /// # Returns
    /// * `Option<DeviceMode>` - Recommended next mode, if applicable
    /// 
    /// # Example
    /// ```
    /// let next_mode = DeviceStateManager::get_recommended_next_mode(
    ///     &device, 
    ///     IntendedOperation::Firing
    /// );
    /// ```
    pub fn get_recommended_next_mode(
        device: &super::super::LumidoxDevice,
        intended_operation: IntendedOperation
    ) -> Option<DeviceMode> {
        match (device.current_mode, intended_operation) {
            (None, _) => Some(DeviceMode::Standby), // Always start with standby
            (Some(DeviceMode::Local), _) => Some(DeviceMode::Standby),
            (Some(DeviceMode::Standby), IntendedOperation::Firing) => Some(DeviceMode::Armed),
            (Some(DeviceMode::Standby), IntendedOperation::Shutdown) => Some(DeviceMode::Local),
            (Some(DeviceMode::Armed), IntendedOperation::Firing) => None, // Already ready
            (Some(DeviceMode::Armed), IntendedOperation::Shutdown) => Some(DeviceMode::Standby),
            (Some(DeviceMode::Remote), IntendedOperation::Shutdown) => Some(DeviceMode::Standby),
            _ => None, // No specific recommendation
        }
    }
    
    /// Perform safe mode transition with validation
    /// 
    /// Executes a mode transition with proper validation and error handling
    /// to ensure safe and reliable mode changes.
    /// 
    /// # Arguments
    /// * `device` - Mutable reference to the device controller
    /// * `target_mode` - The target mode to transition to
    /// * `validate_transition` - Whether to validate the transition is safe
    /// 
    /// # Returns
    /// * `Result<ModeTransitionResult>` - Transition results
    /// 
    /// # Example
    /// ```
    /// let result = DeviceStateManager::safe_mode_transition(
    ///     &mut device, 
    ///     DeviceMode::Armed, 
    ///     true
    /// )?;
    /// ```
    pub fn safe_mode_transition(
        device: &mut super::super::LumidoxDevice,
        target_mode: DeviceMode,
        validate_transition: bool
    ) -> Result<ModeTransitionResult> {
        let previous_mode = device.current_mode;
        
        if validate_transition {
            if let Err(reason) = Self::validate_mode_transition(device.current_mode, target_mode) {
                return Ok(ModeTransitionResult {
                    success: false,
                    previous_mode,
                    new_mode: device.current_mode,
                    error_message: Some(reason.to_string()),
                });
            }
        }
        
        match Self::set_device_mode(device, target_mode) {
            Ok(()) => Ok(ModeTransitionResult {
                success: true,
                previous_mode,
                new_mode: Some(target_mode),
                error_message: None,
            }),
            Err(e) => Ok(ModeTransitionResult {
                success: false,
                previous_mode,
                new_mode: device.current_mode,
                error_message: Some(format!("Mode transition failed: {}", e)),
            }),
        }
    }
    
    /// Validate mode transition safety
    /// 
    /// Checks whether a mode transition is safe and appropriate based on
    /// current state and device safety requirements.
    /// 
    /// # Arguments
    /// * `current_mode` - The current device mode
    /// * `target_mode` - The target mode for transition
    /// 
    /// # Returns
    /// * `Result<()>` - Ok if safe, Err with reason if unsafe
    /// 
    /// # Example
    /// ```
    /// match DeviceStateManager::validate_mode_transition(current, target) {
    ///     Ok(()) => println!("Transition is safe"),
    ///     Err(reason) => println!("Unsafe transition: {}", reason),
    /// }
    /// ```
    pub fn validate_mode_transition(
        current_mode: Option<DeviceMode>,
        target_mode: DeviceMode
    ) -> Result<()> {
        match (current_mode, target_mode) {
            // Always allow transition to standby (safe state)
            (_, DeviceMode::Standby) => Ok(()),
            
            // Allow transition to local from any state (shutdown)
            (_, DeviceMode::Local) => Ok(()),
            
            // Only allow arming from standby
            (Some(DeviceMode::Standby), DeviceMode::Armed) => Ok(()),
            (None, DeviceMode::Armed) => Err(crate::core::LumidoxError::InvalidInput("Cannot arm uninitialized device".to_string())),
            (Some(current), DeviceMode::Armed) => {
                Err(crate::core::LumidoxError::InvalidInput(format!("Cannot arm from {:?} mode, must be in Standby first", current)))
            },

            // Only allow remote mode from armed state
            (Some(DeviceMode::Armed), DeviceMode::Remote) => Ok(()),
            (Some(current), DeviceMode::Remote) => {
                Err(crate::core::LumidoxError::InvalidInput(format!("Cannot enter Remote mode from {:?}, must be Armed first", current)))
            },
            (None, DeviceMode::Remote) => Err(crate::core::LumidoxError::InvalidInput("Cannot enter Remote mode from uninitialized state".to_string())),
        }
    }
    
    /// Get state transition history and recommendations
    /// 
    /// Provides information about valid state transitions and recommendations
    /// for safe device operation.
    /// 
    /// # Arguments
    /// * `current_mode` - The current device mode
    /// 
    /// # Returns
    /// * `StateTransitionInfo` - Information about valid transitions
    /// 
    /// # Example
    /// ```
    /// let info = DeviceStateManager::get_transition_info(device.current_mode);
    /// println!("Valid transitions: {:?}", info.valid_transitions);
    /// ```
    pub fn get_transition_info(current_mode: Option<DeviceMode>) -> StateTransitionInfo {
        let valid_transitions = match current_mode {
            None => vec![DeviceMode::Standby],
            Some(DeviceMode::Local) => vec![DeviceMode::Standby],
            Some(DeviceMode::Standby) => vec![DeviceMode::Local, DeviceMode::Armed],
            Some(DeviceMode::Armed) => vec![DeviceMode::Standby, DeviceMode::Remote],
            Some(DeviceMode::Remote) => vec![DeviceMode::Standby],
        };
        
        let recommended_next = match current_mode {
            None => Some(DeviceMode::Standby),
            Some(DeviceMode::Local) => Some(DeviceMode::Standby),
            _ => None,
        };
        
        StateTransitionInfo {
            current_mode,
            valid_transitions,
            recommended_next,
            is_safe_state: Self::is_in_safe_state_static(current_mode),
        }
    }
    
    /// Check if a mode represents a safe state (static version)
    /// 
    /// Static version of is_in_safe_state for use without device reference.
    /// 
    /// # Arguments
    /// * `mode` - The mode to check
    /// 
    /// # Returns
    /// * `bool` - True if the mode is safe, false otherwise
    fn is_in_safe_state_static(mode: Option<DeviceMode>) -> bool {
        match mode {
            Some(DeviceMode::Local) | Some(DeviceMode::Standby) | None => true,
            _ => false,
        }
    }
}

/// Intended operation types for mode recommendations
#[derive(Debug, Clone, PartialEq)]
pub enum IntendedOperation {
    /// Firing operations (stages or custom current)
    Firing,
    /// Device shutdown and return to local mode
    Shutdown,
    /// Information reading operations
    Information,
    /// Parameter configuration operations
    Configuration,
}

/// Result of device state validation
#[derive(Debug, Clone)]
pub struct StateValidationResult {
    /// Whether the overall state is valid
    pub is_valid: bool,
    /// Whether the device has a current mode set
    pub has_current_mode: bool,
    /// Whether device information is available
    pub has_device_info: bool,
    /// Whether the device is fully initialized
    pub is_fully_initialized: bool,
    /// Description of the current mode
    pub current_mode_description: String,
}

/// Result of a mode transition operation
#[derive(Debug, Clone)]
pub struct ModeTransitionResult {
    /// Whether the transition was successful
    pub success: bool,
    /// The mode before the transition
    pub previous_mode: Option<DeviceMode>,
    /// The mode after the transition
    pub new_mode: Option<DeviceMode>,
    /// Error message if the transition failed
    pub error_message: Option<String>,
}

/// Information about state transitions
#[derive(Debug, Clone)]
pub struct StateTransitionInfo {
    /// The current device mode
    pub current_mode: Option<DeviceMode>,
    /// List of valid modes that can be transitioned to
    pub valid_transitions: Vec<DeviceMode>,
    /// Recommended next mode for typical operations
    pub recommended_next: Option<DeviceMode>,
    /// Whether the current mode is considered safe
    pub is_safe_state: bool,
}
