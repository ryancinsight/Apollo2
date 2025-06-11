//! Control components module for Lumidox II Controller GUI
//!
//! This module organizes and exports all control-related UI components
//! for the Lumidox II Controller GUI application. It provides a centralized
//! access point for stage controls, device controls, and current controls
//! with proper module organization and re-exports.
//!
//! The controls module includes:
//! - Stage firing controls with power information display
//! - Device control buttons (arm, turn off, shutdown)
//! - ARM current setting controls with validation
//! - Consistent styling and error handling across all controls
//! - Reusable components following the Iced framework patterns

// Import control component modules
pub mod stage_controls;
pub mod device_controls;
pub mod current_controls;

// Re-export control components for easy access
pub use stage_controls::StageControls;
pub use device_controls::DeviceControls;
pub use current_controls::CurrentControls;

use iced::{
    widget::{column, container, Space},
    Element, Length, Alignment,
};
use crate::device::LumidoxDevice;

/// Control components coordinator
/// 
/// Provides high-level coordination and organization of all control
/// components with consistent layout and styling patterns.
pub struct ControlComponents;

impl ControlComponents {
    /// Create complete controls panel
    /// 
    /// Combines all control components (stage, device, current) into a
    /// complete controls panel with proper layout and spacing.
    /// 
    /// # Arguments
    /// * `device` - Reference to device for operations and information
    /// * `is_connected` - Whether device is currently connected
    /// * `stage_current_value` - Current value for stage custom current input
    /// * `arm_current_value` - Current value for ARM current input
    /// * `arm_validation_message` - Optional validation message for ARM current
    /// * `on_stage_fire` - Callback for stage firing events
    /// * `on_custom_fire` - Callback for custom current firing
    /// * `on_stage_current_change` - Callback for stage current value changes
    /// * `on_arm_current_change` - Callback for ARM current value changes
    /// * `on_set_arm_current` - Callback for setting ARM current
    /// * `on_arm_device` - Callback for arming device
    /// * `on_turn_off` - Callback for turning off device
    /// * `on_shutdown` - Callback for shutdown and quit
    /// 
    /// # Returns
    /// * `Element<Message>` - Complete controls panel
    /// 
    /// # Example
    /// ```
    /// let controls_panel = ControlComponents::create_controls_panel(
    ///     &device, true, &stage_current, &arm_current, None,
    ///     Message::FireStage, Message::FireCustom, Message::StageCurrentChanged,
    ///     Message::ArmCurrentChanged, Message::SetArmCurrent,
    ///     Message::ArmDevice, Message::TurnOffDevice, Message::ShutdownDevice
    /// )?;
    /// ```
    pub fn create_controls_panel<Message>(
        device: Option<&LumidoxDevice>,
        is_connected: bool,
        stage_current_value: &str,
        arm_current_value: &str,
        arm_validation_message: Option<&str>,
        on_stage_fire: fn(u8) -> Message,
        on_custom_fire: Message,
        on_stage_current_change: fn(String) -> Message,
        on_arm_current_change: fn(String) -> Message,
        on_set_arm_current: Message,
        on_arm_device: Message,
        on_turn_off: Message,
        on_shutdown: Message,
    ) -> Element<'static, Message>
    where
        Message: Clone + 'static,
    {
        let stage_section = StageControls::create_stage_section(
            stage_current_value,
            device,
            on_stage_fire,
            on_stage_current_change,
            on_custom_fire,
        );
        
        let device_section = DeviceControls::create_device_section(
            device,
            is_connected,
            on_arm_device,
            on_turn_off,
            on_shutdown,
        );
        
        let current_section = CurrentControls::create_current_section(
            arm_current_value,
            device,
            arm_validation_message,
            on_arm_current_change,
            on_set_arm_current,
        );
        
        let controls_panel = column![
            stage_section,
            Space::with_height(20),
            device_section,
            Space::with_height(20),
            current_section,
        ]
        .spacing(10)
        .align_items(Alignment::Center);
        
        container(controls_panel)
            .width(Length::Fill)
            .padding(20)
            .into()
    }
    
    /// Create stage controls only
    /// 
    /// Creates only the stage control section for layouts that need
    /// individual control sections.
    /// 
    /// # Arguments
    /// * `device` - Reference to device for operations and information
    /// * `current_value` - Current value for custom current input
    /// * `on_stage_fire` - Callback for stage firing events
    /// * `on_current_change` - Callback for current value changes
    /// * `on_custom_fire` - Callback for custom current firing
    /// 
    /// # Returns
    /// * `Element<Message>` - Stage controls section
    pub fn create_stage_controls_only<Message>(
        device: Option<&LumidoxDevice>,
        current_value: &str,
        on_stage_fire: fn(u8) -> Message,
        on_current_change: fn(String) -> Message,
        on_custom_fire: Message,
    ) -> Element<'static, Message>
    where
        Message: Clone + 'static,
    {
        StageControls::create_stage_section(
            current_value,
            device,
            on_stage_fire,
            on_current_change,
            on_custom_fire,
        )
    }
    
    /// Create device controls only
    /// 
    /// Creates only the device control section for layouts that need
    /// individual control sections.
    /// 
    /// # Arguments
    /// * `device` - Reference to device for operations and information
    /// * `is_connected` - Whether device is currently connected
    /// * `on_arm` - Callback for arming device
    /// * `on_turn_off` - Callback for turning off device
    /// * `on_shutdown` - Callback for shutdown and quit
    /// 
    /// # Returns
    /// * `Element<Message>` - Device controls section
    pub fn create_device_controls_only<Message>(
        device: Option<&LumidoxDevice>,
        is_connected: bool,
        on_arm: Message,
        on_turn_off: Message,
        on_shutdown: Message,
    ) -> Element<'static, Message>
    where
        Message: Clone + 'static,
    {
        DeviceControls::create_device_section(
            device,
            is_connected,
            on_arm,
            on_turn_off,
            on_shutdown,
        )
    }
    
    /// Create current controls only
    /// 
    /// Creates only the current control section for layouts that need
    /// individual control sections.
    /// 
    /// # Arguments
    /// * `device` - Reference to device for operations and information
    /// * `current_value` - Current value for ARM current input
    /// * `validation_message` - Optional validation message to display
    /// * `on_current_change` - Callback for current value changes
    /// * `on_set_current` - Callback for setting ARM current
    /// 
    /// # Returns
    /// * `Element<Message>` - Current controls section
    pub fn create_current_controls_only<Message>(
        device: Option<&LumidoxDevice>,
        current_value: &str,
        validation_message: Option<&str>,
        on_current_change: fn(String) -> Message,
        on_set_current: Message,
    ) -> Element<'static, Message>
    where
        Message: Clone + 'static,
    {
        CurrentControls::create_current_section(
            current_value,
            device,
            validation_message,
            on_current_change,
            on_set_current,
        )
    }
    
    /// Validate stage current input
    /// 
    /// Validates stage custom current input value against device constraints.
    /// 
    /// # Arguments
    /// * `current_value` - Current input value to validate
    /// * `device` - Reference to device for validation context
    /// 
    /// # Returns
    /// * `Option<String>` - Validation error message if invalid, None if valid
    /// 
    /// # Example
    /// ```
    /// let validation_error = ControlComponents::validate_stage_current("1500", &device);
    /// ```
    pub fn validate_stage_current(
        current_value: &str,
        device: Option<&LumidoxDevice>,
    ) -> Option<String> {
        if current_value.is_empty() {
            return Some("Current value is required".to_string());
        }
        
        match current_value.parse::<u16>() {
            Ok(value) => {
                if value == 0 {
                    Some("Current cannot be zero".to_string())
                } else if let Some(dev) = device {
                    if let Ok(max_current) = dev.get_max_current() {
                        if value > max_current {
                            Some(format!("Current too high (max: {}mA)", max_current))
                        } else {
                            None
                        }
                    } else {
                        Some("Cannot validate - device error".to_string())
                    }
                } else {
                    Some("Cannot validate - no device".to_string())
                }
            }
            Err(_) => Some("Invalid number format".to_string())
        }
    }
    
    /// Validate ARM current input
    /// 
    /// Validates ARM current input value against device constraints.
    /// 
    /// # Arguments
    /// * `current_value` - Current input value to validate
    /// * `device` - Reference to device for validation context
    /// 
    /// # Returns
    /// * `Option<String>` - Validation error message if invalid, None if valid
    /// 
    /// # Example
    /// ```
    /// let validation_error = ControlComponents::validate_arm_current("800", &device);
    /// ```
    pub fn validate_arm_current(
        current_value: &str,
        device: Option<&LumidoxDevice>,
    ) -> Option<String> {
        if current_value.is_empty() {
            return Some("ARM current value is required".to_string());
        }
        
        match current_value.parse::<u16>() {
            Ok(value) => {
                if value == 0 {
                    Some("ARM current cannot be zero".to_string())
                } else if let Some(dev) = device {
                    if let Ok(max_current) = dev.get_max_current() {
                        if value > max_current {
                            Some(format!("ARM current too high (max: {}mA)", max_current))
                        } else {
                            None
                        }
                    } else {
                        Some("Cannot validate - device error".to_string())
                    }
                } else {
                    Some("Cannot validate - no device".to_string())
                }
            }
            Err(_) => Some("Invalid number format".to_string())
        }
    }
}
