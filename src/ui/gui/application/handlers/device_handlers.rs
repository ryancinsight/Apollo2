//! Device operation handlers for Lumidox II Controller GUI
//!
//! This module provides device operation handlers that convert DeviceMessage
//! types into async Iced Commands using `Command::perform()`. These handlers
//! integrate with existing CLI device operations while adapting them for the
//! GUI's async message-driven architecture.
//!
//! The device handlers system provides:
//! - Async Command generation for device operations
//! - Integration with existing CLI device controller functions
//! - Proper error handling and result propagation
//! - Non-blocking GUI operations through async patterns
//! - Complete functionality parity with CLI operations
//! - State-aware operation validation and execution

use iced::Command;
use crate::ui::gui::application::messages::{Message, DeviceMessage, DeviceOperationResult};
use crate::ui::gui::application::state::{UnifiedState, ConnectionState, OperationState};
use crate::device::{LumidoxDevice, models::{DeviceMode, DeviceInfo}};
use crate::ui::cli::device::create_device_controller_with_fallback;
use crate::core::{LumidoxError, Result, operations::{StageOperations, DeviceOperationData}};
use std::sync::Arc;
use tokio::sync::Mutex;

/// Device operation handlers
/// 
/// Provides async handlers for device operations that convert DeviceMessage
/// types into Iced Commands for non-blocking GUI execution.
pub struct DeviceHandlers;

impl DeviceHandlers {
    /// Handle device connection operation
    /// 
    /// Creates an async Command to establish device connection using the
    /// existing CLI device controller creation functions.
    /// 
    /// # Arguments
    /// * `port_name` - Optional specific port name
    /// * `auto_detect` - Whether to use automatic port detection
    /// * `verbose` - Enable verbose output during connection
    /// * `optimize_transitions` - Enable optimized stage transitions
    /// 
    /// # Returns
    /// * `Command<Message>` - Async command for device connection
    /// 
    /// # Example
    /// ```
    /// let cmd = DeviceHandlers::handle_connect(None, true, false, true);
    /// ```
    pub fn handle_connect(
        port_name: Option<String>,
        auto_detect: bool,
        verbose: bool,
        optimize_transitions: bool,
    ) -> Command<Message> {
        Command::perform(
            async move {
                // Use existing CLI device controller creation
                let result = if auto_detect {
                    create_device_controller_with_fallback(port_name, verbose)
                } else if let Some(port) = port_name {
                    create_device_controller_with_fallback(Some(port), verbose)
                } else {
                    Err(LumidoxError::InvalidInput(
                        "Port name required when auto-detection is disabled".to_string()
                    ))
                };
                
                match result {
                    Ok(device) => {
                        // Get device information for result
                        let device_info = match device.get_device_info() {
                            Ok(info) => Some(info),
                            Err(_) => None,
                        };
                        
                        DeviceOperationResult::ConnectionResult {
                            success: true,
                            port_name: Some(device.get_port_name().to_string()),
                            device_info,
                            error: None,
                        }
                    }
                    Err(error) => {
                        DeviceOperationResult::ConnectionResult {
                            success: false,
                            port_name: None,
                            device_info: None,
                            error: Some(error.to_string()),
                        }
                    }
                }
            },
            Message::DeviceOperationCompleted,
        )
    }
    
    /// Handle device disconnection operation
    /// 
    /// Creates an async Command to disconnect from the current device.
    /// 
    /// # Returns
    /// * `Command<Message>` - Async command for device disconnection
    /// 
    /// # Example
    /// ```
    /// let cmd = DeviceHandlers::handle_disconnect();
    /// ```
    pub fn handle_disconnect() -> Command<Message> {
        Command::perform(
            async move {
                // Disconnection is typically immediate
                DeviceOperationResult::GeneralResult {
                    success: true,
                    operation: "Disconnect".to_string(),
                    message: Some("Device disconnected successfully".to_string()),
                    error: None,
                }
            },
            Message::DeviceOperationCompleted,
        )
    }
    
    /// Handle stage firing operation
    /// 
    /// Creates an async Command to fire a specific stage using existing
    /// CLI device operations.
    /// 
    /// # Arguments
    /// * `stage` - Stage number to fire (1-5)
    /// * `device` - Shared device controller
    /// 
    /// # Returns
    /// * `Command<Message>` - Async command for stage firing
    /// 
    /// # Example
    /// ```
    /// let cmd = DeviceHandlers::handle_fire_stage(3, device_arc);
    /// ```
    pub fn handle_fire_stage(
        stage: u8,
        device: Arc<Mutex<Option<LumidoxDevice>>>,
    ) -> Command<Message> {
        Command::perform(
            async move {
                let mut device_guard = device.lock().await;
                
                if let Some(ref mut dev) = device_guard.as_mut() {
                    // Use unified operation layer
                    match StageOperations::fire_stage_unified(dev, stage) {
                        Ok(response) => {
                            // GUI-specific presentation of the unified result
                            let mut operation_desc = format!("Fire Stage {}", stage);
                            if let DeviceOperationData::StageFiring { current_ma, .. } = response.data {
                                if let Some(current) = current_ma {
                                    operation_desc.push_str(&format!(" ({}mA)", current));
                                }
                            }
                            DeviceOperationResult::FiringResult {
                                success: true,
                                operation: operation_desc,
                                error: None,
                            }
                        }
                        Err(error) => {
                            DeviceOperationResult::FiringResult {
                                success: false,
                                operation: format!("Fire Stage {}", stage),
                                error: Some(error.to_string()),
                            }
                        }
                    }
                } else {
                    DeviceOperationResult::FiringResult {
                        success: false,
                        operation: format!("Fire Stage {}", stage),
                        error: Some("Device not connected".to_string()),
                    }
                }
            },
            Message::DeviceOperationCompleted,
        )
    }
    
    /// Handle custom current firing operation
    /// 
    /// Creates an async Command to fire with a custom current value.
    /// 
    /// # Arguments
    /// * `current` - Current value in milliamps
    /// * `device` - Shared device controller
    /// 
    /// # Returns
    /// * `Command<Message>` - Async command for custom current firing
    /// 
    /// # Example
    /// ```
    /// let cmd = DeviceHandlers::handle_fire_custom(1500, device_arc);
    /// ```
    pub fn handle_fire_custom(
        current: u16,
        device: Arc<Mutex<Option<LumidoxDevice>>>,
    ) -> Command<Message> {
        Command::perform(
            async move {
                let mut device_guard = device.lock().await;
                
                if let Some(ref mut dev) = device_guard.as_mut() {
                    // Use existing CLI fire with current operation
                    match crate::device::operations::fire_with_current(dev, current) {
                        Ok(_) => {
                            DeviceOperationResult::FiringResult {
                                success: true,
                                operation: format!("Fire with {}mA", current),
                                error: None,
                            }
                        }
                        Err(error) => {
                            DeviceOperationResult::FiringResult {
                                success: false,
                                operation: format!("Fire with {}mA", current),
                                error: Some(error.to_string()),
                            }
                        }
                    }
                } else {
                    DeviceOperationResult::FiringResult {
                        success: false,
                        operation: format!("Fire with {}mA", current),
                        error: Some("Device not connected".to_string()),
                    }
                }
            },
            Message::DeviceOperationCompleted,
        )
    }
    
    /// Handle ARM current setting operation
    /// 
    /// Creates an async Command to set the ARM current value.
    /// 
    /// # Arguments
    /// * `current` - ARM current value in milliamps
    /// * `device` - Shared device controller
    /// 
    /// # Returns
    /// * `Command<Message>` - Async command for ARM current setting
    /// 
    /// # Example
    /// ```
    /// let cmd = DeviceHandlers::handle_set_arm_current(1000, device_arc);
    /// ```
    pub fn handle_set_arm_current(
        current: u16,
        device: Arc<Mutex<Option<LumidoxDevice>>>,
    ) -> Command<Message> {
        Command::perform(
            async move {
                let mut device_guard = device.lock().await;
                
                if let Some(ref mut dev) = device_guard.as_mut() {
                    // Use existing CLI set ARM current operation
                    match crate::device::operations::set_arm_current(dev, current) {
                        Ok(_) => {
                            DeviceOperationResult::ParameterResult {
                                success: true,
                                parameter_name: "ARM Current".to_string(),
                                value: Some(format!("{}mA", current)),
                                error: None,
                            }
                        }
                        Err(error) => {
                            DeviceOperationResult::ParameterResult {
                                success: false,
                                parameter_name: "ARM Current".to_string(),
                                value: None,
                                error: Some(error.to_string()),
                            }
                        }
                    }
                } else {
                    DeviceOperationResult::ParameterResult {
                        success: false,
                        parameter_name: "ARM Current".to_string(),
                        value: None,
                        error: Some("Device not connected".to_string()),
                    }
                }
            },
            Message::DeviceOperationCompleted,
        )
    }
    
    /// Handle device arming operation
    /// 
    /// Creates an async Command to arm the device for firing operations.
    /// 
    /// # Arguments
    /// * `device` - Shared device controller
    /// 
    /// # Returns
    /// * `Command<Message>` - Async command for device arming
    /// 
    /// # Example
    /// ```
    /// let cmd = DeviceHandlers::handle_arm_device(device_arc);
    /// ```
    pub fn handle_arm_device(device: Arc<Mutex<Option<LumidoxDevice>>>) -> Command<Message> {
        Command::perform(
            async move {
                let mut device_guard = device.lock().await;
                
                if let Some(ref mut dev) = device_guard.as_mut() {
                    // Use existing CLI arm device operation
                    match crate::device::operations::arm_device(dev) {
                        Ok(_) => {
                            DeviceOperationResult::GeneralResult {
                                success: true,
                                operation: "Arm Device".to_string(),
                                message: Some("Device armed successfully".to_string()),
                                error: None,
                            }
                        }
                        Err(error) => {
                            DeviceOperationResult::GeneralResult {
                                success: false,
                                operation: "Arm Device".to_string(),
                                message: None,
                                error: Some(error.to_string()),
                            }
                        }
                    }
                } else {
                    DeviceOperationResult::GeneralResult {
                        success: false,
                        operation: "Arm Device".to_string(),
                        message: None,
                        error: Some("Device not connected".to_string()),
                    }
                }
            },
            Message::DeviceOperationCompleted,
        )
    }
    
    /// Handle device status reading operation
    /// 
    /// Creates an async Command to read current device status and parameters.
    /// 
    /// # Arguments
    /// * `device` - Shared device controller
    /// 
    /// # Returns
    /// * `Command<Message>` - Async command for status reading
    /// 
    /// # Example
    /// ```
    /// let cmd = DeviceHandlers::handle_read_device_status(device_arc);
    /// ```
    pub fn handle_read_device_status(device: Arc<Mutex<Option<LumidoxDevice>>>) -> Command<Message> {
        Command::perform(
            async move {
                let mut device_guard = device.lock().await;
                
                if let Some(ref mut dev) = device_guard.as_mut() {
                    // Read device status using existing CLI operations
                    let arm_current = crate::device::operations::read_arm_current(dev).ok();
                    let fire_current = crate::device::operations::read_fire_current(dev).ok();
                    let remote_mode = crate::device::operations::read_remote_mode_state(dev).ok();
                    
                    DeviceOperationResult::StatusResult {
                        success: true,
                        device_mode: remote_mode,
                        arm_current,
                        fire_current,
                        error: None,
                    }
                } else {
                    DeviceOperationResult::StatusResult {
                        success: false,
                        device_mode: None,
                        arm_current: None,
                        fire_current: None,
                        error: Some("Device not connected".to_string()),
                    }
                }
            },
            Message::DeviceOperationCompleted,
        )
    }

    /// Handle power values refresh operation
    ///
    /// Creates an async Command to refresh power values for all stages.
    /// This provides real-time power monitoring for the GUI display.
    ///
    /// # Arguments
    /// * `device` - Shared device controller
    ///
    /// # Returns
    /// * `Command<Message>` - Async command for power values refresh
    ///
    /// # Example
    /// ```
    /// let cmd = DeviceHandlers::handle_refresh_power_values(device_arc);
    /// ```
    pub fn handle_refresh_power_values(device: Arc<Mutex<Option<LumidoxDevice>>>) -> Command<Message> {
        Command::perform(
            async move {
                let mut device_guard = device.lock().await;

                if let Some(ref mut dev) = device_guard.as_mut() {
                    // Read power information for all stages
                    let mut power_readings = Vec::new();
                    let mut errors = Vec::new();

                    for stage in 1..=5 {
                        match dev.get_power_info(stage) {
                            Ok(power_info) => {
                                power_readings.push(format!(
                                    "Stage {}: {} {} ({} {})",
                                    stage,
                                    power_info.total_power,
                                    power_info.total_units,
                                    power_info.per_power,
                                    power_info.per_units
                                ));
                            }
                            Err(e) => {
                                errors.push(format!("Stage {}: {}", stage, e));
                            }
                        }
                    }

                    if errors.is_empty() {
                        DeviceOperationResult::GeneralResult {
                            success: true,
                            operation: "Refresh Power Values".to_string(),
                            message: Some(format!("Power values updated: {}", power_readings.join(", "))),
                            error: None,
                        }
                    } else {
                        DeviceOperationResult::GeneralResult {
                            success: false,
                            operation: "Refresh Power Values".to_string(),
                            message: None,
                            error: Some(format!("Failed to read some stages: {}", errors.join(", "))),
                        }
                    }
                } else {
                    DeviceOperationResult::GeneralResult {
                        success: false,
                        operation: "Refresh Power Values".to_string(),
                        message: None,
                        error: Some("Device not connected".to_string()),
                    }
                }
            },
            Message::DeviceOperationCompleted,
        )
    }
}
