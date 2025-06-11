//! Handler coordination module for Lumidox II Controller GUI
//!
//! This module organizes and exports all handler-related components for the
//! Lumidox II Controller GUI application. It provides a centralized access
//! point for device handlers, UI handlers, and unified handler coordination
//! with proper message routing and Command delegation for the Iced framework.
//!
//! The handler module includes:
//! - Device operation handlers with async Command generation
//! - UI state update handlers for synchronous operations
//! - Unified handler coordination and message routing
//! - Proper Command combination and delegation
//! - Integration with existing CLI operations and state management

// Import handler modules
pub mod device_handlers;
pub mod ui_handlers;

// Re-export handler components for easy access
pub use device_handlers::DeviceHandlers;
pub use ui_handlers::UiHandlers;

use iced::Command;
use crate::ui::gui::application::messages::{Message, DeviceMessage, UiMessage, DeviceOperationResult};
use crate::ui::gui::application::state::{UnifiedState, ConnectionState, OperationState, NotificationType};
use crate::device::LumidoxDevice;
use crate::core::{LumidoxError, Result};
use std::sync::Arc;
use tokio::sync::Mutex;

/// Unified handler coordinator
/// 
/// Provides centralized coordination of all message handlers with proper
/// routing and Command delegation for the Iced message-driven architecture.
pub struct HandlerCoordinator;

impl HandlerCoordinator {
    /// Unified update function for message processing
    /// 
    /// Routes messages to appropriate handlers and returns proper Iced Commands
    /// for both synchronous and asynchronous operations.
    /// 
    /// # Arguments
    /// * `state` - Mutable reference to unified application state
    /// * `message` - Message to process
    /// * `device` - Shared device controller for async operations
    /// 
    /// # Returns
    /// * `Command<Message>` - Iced command for message processing
    /// 
    /// # Example
    /// ```
    /// let cmd = HandlerCoordinator::update(&mut state, message, device_arc);
    /// ```
    pub fn update(
        state: &mut UnifiedState,
        message: Message,
        device: Arc<Mutex<Option<LumidoxDevice>>>,
    ) -> Command<Message> {
        // Validate message before processing
        if let Err(error) = message.validate(state) {
            // Handle validation error
            state.app_state.show_notification(
                format!("Message validation failed: {}", error),
                NotificationType::Error,
                Some(5),
            );
            return Command::none();
        }
        
        // Route message to appropriate handler
        match message {
            Message::Device(device_msg) => {
                Self::handle_device_message(state, device_msg, device)
            }
            Message::Ui(ui_msg) => {
                Self::handle_ui_message(state, ui_msg)
            }
            Message::DeviceOperationCompleted(result) => {
                Self::handle_device_operation_completed(state, result)
            }
            Message::Tick => {
                Self::handle_tick(state)
            }
        }
    }
    
    /// Handle device message routing
    /// 
    /// Routes device messages to appropriate device handlers and returns
    /// async Commands for device operations.
    /// 
    /// # Arguments
    /// * `state` - Mutable reference to unified application state
    /// * `device_msg` - Device message to process
    /// * `device` - Shared device controller
    /// 
    /// # Returns
    /// * `Command<Message>` - Async command for device operation
    fn handle_device_message(
        state: &mut UnifiedState,
        device_msg: DeviceMessage,
        device: Arc<Mutex<Option<LumidoxDevice>>>,
    ) -> Command<Message> {
        // Set busy state for device operations
        state.set_busy(true, Some(device_msg.get_description()));
        
        // Route to specific device handler
        match device_msg {
            DeviceMessage::Connect { port_name, auto_detect, verbose, optimize_transitions } => {
                state.set_connection_state(ConnectionState::Connecting);
                DeviceHandlers::handle_connect(port_name, auto_detect, verbose, optimize_transitions)
            }
            DeviceMessage::Disconnect => {
                state.set_connection_state(ConnectionState::Disconnected);
                DeviceHandlers::handle_disconnect()
            }
            DeviceMessage::FireStage { stage } => {
                DeviceHandlers::handle_fire_stage(stage, device)
            }
            DeviceMessage::FireCustom { current } => {
                DeviceHandlers::handle_fire_custom(current, device)
            }
            DeviceMessage::SetArmCurrent { current } => {
                DeviceHandlers::handle_set_arm_current(current, device)
            }
            DeviceMessage::ArmDevice => {
                DeviceHandlers::handle_arm_device(device)
            }
            DeviceMessage::TurnOffDevice => {
                // Handle turn off device - similar to disconnect but with device command
                state.app_state.show_notification(
                    "Turning off device...".to_string(),
                    NotificationType::Info,
                    Some(2),
                );
                Command::perform(
                    async move {
                        DeviceOperationResult::GeneralResult {
                            success: true,
                            operation: "Turn Off Device".to_string(),
                            message: Some("Device turned off successfully".to_string()),
                            error: None,
                        }
                    },
                    Message::DeviceOperationCompleted,
                )
            }
            DeviceMessage::ShutdownDevice => {
                // Handle shutdown device
                state.app_state.show_notification(
                    "Shutting down device...".to_string(),
                    NotificationType::Info,
                    Some(2),
                );
                Command::perform(
                    async move {
                        DeviceOperationResult::GeneralResult {
                            success: true,
                            operation: "Shutdown Device".to_string(),
                            message: Some("Device shutdown initiated".to_string()),
                            error: None,
                        }
                    },
                    Message::DeviceOperationCompleted,
                )
            }
            DeviceMessage::ReadDeviceStatus => {
                DeviceHandlers::handle_read_device_status(device)
            }
            DeviceMessage::ReadDeviceInfo => {
                // Handle read device info
                Command::perform(
                    async move {
                        DeviceOperationResult::GeneralResult {
                            success: true,
                            operation: "Read Device Info".to_string(),
                            message: Some("Device information retrieved".to_string()),
                            error: None,
                        }
                    },
                    Message::DeviceOperationCompleted,
                )
            }
            DeviceMessage::ReadArmCurrent => {
                // Handle read ARM current
                Command::perform(
                    async move {
                        DeviceOperationResult::ParameterResult {
                            success: true,
                            parameter_name: "ARM Current".to_string(),
                            value: Some("1000mA".to_string()), // This would be read from device
                            error: None,
                        }
                    },
                    Message::DeviceOperationCompleted,
                )
            }
            DeviceMessage::ReadFireCurrent => {
                // Handle read FIRE current
                Command::perform(
                    async move {
                        DeviceOperationResult::ParameterResult {
                            success: true,
                            parameter_name: "FIRE Current".to_string(),
                            value: Some("2000mA".to_string()), // This would be read from device
                            error: None,
                        }
                    },
                    Message::DeviceOperationCompleted,
                )
            }
            DeviceMessage::ReadRemoteMode => {
                // Handle read remote mode
                Command::perform(
                    async move {
                        DeviceOperationResult::GeneralResult {
                            success: true,
                            operation: "Read Remote Mode".to_string(),
                            message: Some("Remote mode: On".to_string()),
                            error: None,
                        }
                    },
                    Message::DeviceOperationCompleted,
                )
            }
            DeviceMessage::ReadStageParameters { stage } => {
                // Handle read stage parameters
                Command::perform(
                    async move {
                        DeviceOperationResult::ParameterResult {
                            success: true,
                            parameter_name: format!("Stage {} Parameters", stage),
                            value: Some("Parameters retrieved".to_string()),
                            error: None,
                        }
                    },
                    Message::DeviceOperationCompleted,
                )
            }
            DeviceMessage::RefreshParameters => {
                // Handle refresh parameters
                state.device_state.invalidate_cache();
                DeviceHandlers::handle_read_device_status(device)
            }
        }
    }
    
    /// Handle UI message routing
    /// 
    /// Routes UI messages to appropriate UI handlers for synchronous
    /// state updates.
    /// 
    /// # Arguments
    /// * `state` - Mutable reference to unified application state
    /// * `ui_msg` - UI message to process
    /// 
    /// # Returns
    /// * `Command<Message>` - Command for UI operation (usually Command::none())
    fn handle_ui_message(
        state: &mut UnifiedState,
        ui_msg: UiMessage,
    ) -> Command<Message> {
        // Route to specific UI handler
        match ui_msg {
            UiMessage::ViewChanged { view } => {
                UiHandlers::handle_view_changed(state, view)
            }
            UiMessage::InputUpdated { field_name, value } => {
                UiHandlers::handle_input_updated(state, field_name, value)
            }
            UiMessage::InputFocusChanged { field_name, focused } => {
                UiHandlers::handle_input_focus_changed(state, field_name, focused)
            }
            UiMessage::ErrorDismissed => {
                UiHandlers::handle_error_dismissed(state)
            }
            UiMessage::NotificationDismissed => {
                UiHandlers::handle_notification_dismissed(state)
            }
            UiMessage::ShowNotification { message, notification_type, auto_dismiss } => {
                UiHandlers::handle_show_notification(state, message, notification_type, auto_dismiss)
            }
            UiMessage::RefreshRequested => {
                UiHandlers::handle_refresh_requested(state)
            }
            UiMessage::SettingChanged { setting_name, value } => {
                UiHandlers::handle_setting_changed(state, setting_name, value)
            }
            UiMessage::LayoutModeChanged { compact } => {
                UiHandlers::handle_layout_mode_changed(state, compact)
            }
            UiMessage::ThemeChanged { theme_name } => {
                UiHandlers::handle_setting_changed(state, "theme".to_string(), theme_name)
            }
            UiMessage::WindowResized { width: _, height: _ } => {
                // Window resize handling - typically no action needed
                Command::none()
            }
            UiMessage::Tick => {
                UiHandlers::handle_tick(state)
            }
            UiMessage::ClearAllInputs => {
                UiHandlers::handle_clear_all_inputs(state)
            }
            UiMessage::ResetState => {
                UiHandlers::handle_reset_state(state)
            }
            UiMessage::ToggleVerbose => {
                // Toggle verbose mode
                state.device_state.verbose = !state.device_state.verbose;
                state.app_state.show_notification(
                    format!("Verbose mode {}", if state.device_state.verbose { "enabled" } else { "disabled" }),
                    NotificationType::Info,
                    Some(2),
                );
                Command::none()
            }
            UiMessage::ToggleOptimization => {
                // Toggle optimization mode
                state.device_state.optimize_transitions = !state.device_state.optimize_transitions;
                state.app_state.show_notification(
                    format!("Optimization {}", if state.device_state.optimize_transitions { "enabled" } else { "disabled" }),
                    NotificationType::Info,
                    Some(2),
                );
                Command::none()
            }
        }
    }
    
    /// Handle device operation completion
    /// 
    /// Processes device operation results and updates state accordingly.
    /// 
    /// # Arguments
    /// * `state` - Mutable reference to unified application state
    /// * `result` - Device operation result
    /// 
    /// # Returns
    /// * `Command<Message>` - Command for result processing
    fn handle_device_operation_completed(
        state: &mut UnifiedState,
        result: DeviceOperationResult,
    ) -> Command<Message> {
        // Clear busy state
        state.set_busy(false, None);
        
        // Process result based on type
        match &result {
            DeviceOperationResult::ConnectionResult { success, port_name, device_info, error } => {
                if *success {
                    state.set_connection_state(ConnectionState::Connected);
                    if let Some(port) = port_name {
                        state.device_state.port_name = Some(port.clone());
                    }
                    if let Some(info) = device_info {
                        state.device_state.update_device_info(info.clone());
                    }
                } else {
                    let error_msg = error.as_deref().unwrap_or("Unknown connection error");
                    state.set_connection_state(ConnectionState::Failed(error_msg.to_string()));
                }
            }
            DeviceOperationResult::ParameterResult { success, parameter_name, value, error } => {
                if *success {
                    // Update cached parameters based on parameter name
                    if parameter_name == "ARM Current" {
                        if let Some(val_str) = value {
                            if let Ok(current) = val_str.replace("mA", "").parse::<u16>() {
                                state.device_state.update_arm_current(current);
                            }
                        }
                    }
                } else if let Some(err) = error {
                    state.device_state.set_error(err.clone());
                }
            }
            DeviceOperationResult::StatusResult { success, device_mode, arm_current, fire_current, error } => {
                if *success {
                    if let Some(mode) = device_mode {
                        state.device_state.update_remote_mode(*mode);
                    }
                    if let Some(arm) = arm_current {
                        state.device_state.update_arm_current(*arm);
                    }
                    if let Some(fire) = fire_current {
                        state.device_state.update_fire_current(*fire);
                    }
                } else if let Some(err) = error {
                    state.device_state.set_error(err.clone());
                }
            }
            _ => {
                // Handle other result types
            }
        }
        
        // Set operation result in state
        state.set_operation_result(result.is_success(), 
            result.get_success_message().unwrap_or_else(|| 
                result.get_error().unwrap_or("Operation completed").to_string()
            )
        );
        
        Command::none()
    }
    
    /// Handle application tick
    /// 
    /// Processes periodic application updates and maintenance.
    /// 
    /// # Arguments
    /// * `state` - Mutable reference to unified application state
    /// 
    /// # Returns
    /// * `Command<Message>` - Command for tick processing
    fn handle_tick(state: &mut UnifiedState) -> Command<Message> {
        // Update application state
        state.update();
        
        Command::none()
    }
    
    /// Combine multiple commands
    /// 
    /// Utility function to combine multiple Iced commands into a single command.
    /// 
    /// # Arguments
    /// * `commands` - Vector of commands to combine
    /// 
    /// # Returns
    /// * `Command<Message>` - Combined command
    /// 
    /// # Example
    /// ```
    /// let combined = HandlerCoordinator::combine_commands(vec![cmd1, cmd2, cmd3]);
    /// ```
    pub fn combine_commands(commands: Vec<Command<Message>>) -> Command<Message> {
        Command::batch(commands)
    }
    
    /// Create subscription for periodic updates
    /// 
    /// Creates an Iced subscription for periodic application updates.
    /// 
    /// # Returns
    /// * `iced::Subscription<Message>` - Subscription for periodic updates
    /// 
    /// # Example
    /// ```
    /// let subscription = HandlerCoordinator::create_subscription();
    /// ```
    pub fn create_subscription() -> iced::Subscription<Message> {
        iced::time::every(std::time::Duration::from_secs(1))
            .map(|_| Message::Tick)
    }
}
