//! Message system module for Lumidox II Controller GUI
//!
//! This module organizes and exports all message-related components for the
//! Lumidox II Controller GUI application. It provides a centralized access
//! point for device messages, UI messages, and unified message coordination
//! with proper routing and categorization for the Iced message-driven architecture.
//!
//! The message module includes:
//! - Device operation messages with async Command integration
//! - UI interaction messages for synchronous state updates
//! - Unified message coordination and routing
//! - Proper message categorization and safety levels
//! - Integration with existing CLI operations and state management

// Import message modules
pub mod device_messages;
pub mod ui_messages;

// Re-export message components for easy access
pub use device_messages::{
    DeviceMessage, DeviceOperationResult, SafetyLevel
};
pub use ui_messages::{
    UiMessage, MessagePriority, MessageCategory, UiMessageUtils
};

use crate::ui::gui::application::state::UnifiedState;
use crate::core::{LumidoxError, Result};

/// Unified message system
/// 
/// Combines device messages and UI messages into a unified message
/// system for comprehensive GUI message handling and routing.
#[derive(Debug, Clone)]
pub enum Message {
    /// Device operation message
    Device(DeviceMessage),
    /// UI interaction message
    Ui(UiMessage),
    /// Device operation completed with result
    DeviceOperationCompleted(DeviceOperationResult),
    /// Periodic timer tick for updates
    Tick,
}

impl Message {
    /// Check if message requires async processing
    /// 
    /// Used to determine if message should be processed with async
    /// Command operations or immediate state updates.
    /// 
    /// # Returns
    /// * `bool` - True if message requires async processing
    /// 
    /// # Example
    /// ```
    /// let device_msg = Message::Device(DeviceMessage::Connect { 
    ///     port_name: None, auto_detect: true, verbose: false, optimize_transitions: true 
    /// });
    /// assert!(device_msg.requires_async_processing());
    /// 
    /// let ui_msg = Message::Ui(UiMessage::ViewChanged { view: AppView::Main });
    /// assert!(!ui_msg.requires_async_processing());
    /// ```
    pub fn requires_async_processing(&self) -> bool {
        match self {
            Message::Device(_) => true,
            Message::Ui(ui_msg) => {
                // Most UI messages are synchronous, but some may trigger async operations
                match ui_msg {
                    UiMessage::RefreshRequested => true,
                    _ => false,
                }
            }
            Message::DeviceOperationCompleted(_) => false,
            Message::Tick => false,
        }
    }
    
    /// Check if message is potentially destructive
    /// 
    /// Used for safety checks and confirmation dialogs in the GUI.
    /// 
    /// # Returns
    /// * `bool` - True if message represents a destructive operation
    /// 
    /// # Example
    /// ```
    /// let fire_msg = Message::Device(DeviceMessage::FireStage { stage: 1 });
    /// assert!(fire_msg.is_destructive());
    /// 
    /// let read_msg = Message::Device(DeviceMessage::ReadDeviceStatus);
    /// assert!(!read_msg.is_destructive());
    /// ```
    pub fn is_destructive(&self) -> bool {
        match self {
            Message::Device(device_msg) => device_msg.is_destructive(),
            _ => false,
        }
    }
    
    /// Get message priority for processing order
    /// 
    /// Returns priority level for message processing to ensure
    /// critical operations are handled appropriately.
    /// 
    /// # Returns
    /// * `MessagePriority` - Priority level of the message
    /// 
    /// # Example
    /// ```
    /// let error_result = Message::DeviceOperationCompleted(
    ///     DeviceOperationResult::GeneralResult { 
    ///         success: false, operation: "test".to_string(), 
    ///         message: None, error: Some("error".to_string()) 
    ///     }
    /// );
    /// assert_eq!(error_result.get_priority(), MessagePriority::High);
    /// ```
    pub fn get_priority(&self) -> MessagePriority {
        match self {
            Message::Device(device_msg) => {
                match device_msg.get_safety_level() {
                    SafetyLevel::High => MessagePriority::High,
                    SafetyLevel::Medium => MessagePriority::Medium,
                    SafetyLevel::Low => MessagePriority::Normal,
                    SafetyLevel::None => MessagePriority::Normal,
                }
            }
            Message::Ui(ui_msg) => ui_msg.get_priority(),
            Message::DeviceOperationCompleted(result) => {
                if result.is_success() {
                    MessagePriority::Normal
                } else {
                    MessagePriority::High
                }
            }
            Message::Tick => MessagePriority::Low,
        }
    }
    
    /// Get message category for organization
    /// 
    /// Categorizes messages for better organization and debugging.
    /// 
    /// # Returns
    /// * `MessageCategory` - Category of the message
    /// 
    /// # Example
    /// ```
    /// let device_msg = Message::Device(DeviceMessage::ReadDeviceStatus);
    /// assert_eq!(device_msg.get_category(), MessageCategory::System);
    /// ```
    pub fn get_category(&self) -> MessageCategory {
        match self {
            Message::Device(_) => MessageCategory::System,
            Message::Ui(ui_msg) => ui_msg.get_category(),
            Message::DeviceOperationCompleted(_) => MessageCategory::System,
            Message::Tick => MessageCategory::System,
        }
    }
    
    /// Get human-readable description of the message
    /// 
    /// Provides description for logging and debugging purposes.
    /// 
    /// # Returns
    /// * `String` - Human-readable message description
    /// 
    /// # Example
    /// ```
    /// let device_msg = Message::Device(DeviceMessage::FireStage { stage: 2 });
    /// assert_eq!(device_msg.get_description(), "Device: Fire Stage 2");
    /// ```
    pub fn get_description(&self) -> String {
        match self {
            Message::Device(device_msg) => format!("Device: {}", device_msg.get_description()),
            Message::Ui(ui_msg) => format!("UI: {}", ui_msg.get_description()),
            Message::DeviceOperationCompleted(result) => {
                if result.is_success() {
                    if let Some(msg) = result.get_success_message() {
                        format!("Operation completed: {}", msg)
                    } else {
                        "Operation completed successfully".to_string()
                    }
                } else {
                    if let Some(error) = result.get_error() {
                        format!("Operation failed: {}", error)
                    } else {
                        "Operation failed".to_string()
                    }
                }
            }
            Message::Tick => "Application tick".to_string(),
        }
    }
    
    /// Validate message before processing
    /// 
    /// Performs validation on message parameters and state requirements
    /// before processing to ensure they are valid.
    /// 
    /// # Arguments
    /// * `state` - Current application state for validation context
    /// 
    /// # Returns
    /// * `Result<()>` - Ok if message is valid, Err with validation error
    /// 
    /// # Example
    /// ```
    /// let device_msg = Message::Device(DeviceMessage::FireStage { stage: 3 });
    /// assert!(device_msg.validate(&state).is_ok());
    /// ```
    pub fn validate(&self, state: &UnifiedState) -> Result<()> {
        match self {
            Message::Device(device_msg) => {
                // Validate device message parameters
                device_msg.validate()?;
                
                // Check if device connection is required
                if device_msg.requires_connection() && !state.is_device_connected() {
                    return Err(LumidoxError::DeviceNotConnected(
                        "Device operation requires active connection".to_string()
                    ));
                }
                
                // Check if device is busy for destructive operations
                if device_msg.is_destructive() && state.is_busy() {
                    return Err(LumidoxError::DeviceBusy(
                        "Cannot perform destructive operation while device is busy".to_string()
                    ));
                }
            }
            Message::Ui(ui_msg) => {
                // UI messages generally don't require validation
                // but we can add specific checks if needed
                match ui_msg {
                    UiMessage::InputUpdated { field_name, value } => {
                        // Validate input field updates
                        if field_name.is_empty() {
                            return Err(LumidoxError::InvalidInput(
                                "Field name cannot be empty".to_string()
                            ));
                        }
                        // Additional validation can be added here
                    }
                    _ => {}
                }
            }
            _ => {} // Other messages don't require validation
        }
        
        Ok(())
    }
}

/// Message routing utilities
/// 
/// Provides utility functions for message routing and processing
/// coordination across the GUI application.
pub struct MessageRouter;

impl MessageRouter {
    /// Route message to appropriate handler
    /// 
    /// Determines the appropriate handler for a message based on its
    /// type and current application state.
    /// 
    /// # Arguments
    /// * `message` - Message to route
    /// * `state` - Current application state
    /// 
    /// # Returns
    /// * `MessageRoute` - Routing information for the message
    /// 
    /// # Example
    /// ```
    /// let route = MessageRouter::route_message(&message, &state);
    /// ```
    pub fn route_message(message: &Message, state: &UnifiedState) -> MessageRoute {
        match message {
            Message::Device(_) => {
                if state.is_device_connected() {
                    MessageRoute::DeviceHandler
                } else {
                    MessageRoute::ConnectionHandler
                }
            }
            Message::Ui(_) => MessageRoute::UiHandler,
            Message::DeviceOperationCompleted(_) => MessageRoute::ResultHandler,
            Message::Tick => MessageRoute::SystemHandler,
        }
    }
    
    /// Check if message should be queued
    /// 
    /// Determines if a message should be queued for later processing
    /// based on current system state and message priority.
    /// 
    /// # Arguments
    /// * `message` - Message to check
    /// * `state` - Current application state
    /// 
    /// # Returns
    /// * `bool` - True if message should be queued
    /// 
    /// # Example
    /// ```
    /// let should_queue = MessageRouter::should_queue_message(&message, &state);
    /// ```
    pub fn should_queue_message(message: &Message, state: &UnifiedState) -> bool {
        // Queue destructive operations if system is busy
        if message.is_destructive() && state.is_busy() {
            return true;
        }
        
        // Queue device operations if not connected (except connect operations)
        if let Message::Device(device_msg) = message {
            if device_msg.requires_connection() && !state.is_device_connected() {
                return true;
            }
        }
        
        false
    }
    
    /// Get message processing timeout
    /// 
    /// Returns appropriate timeout for message processing based on
    /// message type and complexity.
    /// 
    /// # Arguments
    /// * `message` - Message to get timeout for
    /// 
    /// # Returns
    /// * `Option<u64>` - Timeout in seconds, None for no timeout
    /// 
    /// # Example
    /// ```
    /// let timeout = MessageRouter::get_processing_timeout(&message);
    /// ```
    pub fn get_processing_timeout(message: &Message) -> Option<u64> {
        match message {
            Message::Device(device_msg) => {
                match device_msg {
                    DeviceMessage::Connect { .. } => Some(30), // 30 seconds for connection
                    DeviceMessage::FireStage { .. } |
                    DeviceMessage::FireCustom { .. } => Some(10), // 10 seconds for firing
                    DeviceMessage::ReadDeviceStatus |
                    DeviceMessage::ReadDeviceInfo => Some(5), // 5 seconds for reading
                    _ => Some(15), // 15 seconds default for device operations
                }
            }
            Message::Ui(_) => None, // UI operations should be immediate
            Message::DeviceOperationCompleted(_) => None,
            Message::Tick => None,
        }
    }
}

/// Message routing destinations
/// 
/// Defines the different handlers that messages can be routed to
/// for processing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MessageRoute {
    /// Route to device operation handler
    DeviceHandler,
    /// Route to connection management handler
    ConnectionHandler,
    /// Route to UI state handler
    UiHandler,
    /// Route to operation result handler
    ResultHandler,
    /// Route to system/lifecycle handler
    SystemHandler,
}

/// Message factory utilities
/// 
/// Provides convenience functions for creating common message types.
pub struct MessageFactory;

impl MessageFactory {
    /// Create device connection message
    pub fn connect_device(
        port_name: Option<String>,
        auto_detect: bool,
        verbose: bool,
        optimize_transitions: bool,
    ) -> Message {
        Message::Device(DeviceMessage::Connect {
            port_name,
            auto_detect,
            verbose,
            optimize_transitions,
        })
    }
    
    /// Create device disconnection message
    pub fn disconnect_device() -> Message {
        Message::Device(DeviceMessage::Disconnect)
    }
    
    /// Create stage firing message
    pub fn fire_stage(stage: u8) -> Message {
        Message::Device(DeviceMessage::FireStage { stage })
    }
    
    /// Create ARM current setting message
    pub fn set_arm_current(current: u16) -> Message {
        Message::Device(DeviceMessage::SetArmCurrent { current })
    }
    
    /// Create view change message
    pub fn change_view(view: crate::ui::gui::application::state::AppView) -> Message {
        Message::Ui(UiMessage::ViewChanged { view })
    }
    
    /// Create input update message
    pub fn update_input(field_name: String, value: String) -> Message {
        Message::Ui(UiMessage::InputUpdated { field_name, value })
    }
    
    /// Create notification message
    pub fn show_notification(
        message: String,
        notification_type: crate::ui::gui::application::state::NotificationType,
        auto_dismiss: Option<u32>,
    ) -> Message {
        Message::Ui(UiMessage::ShowNotification {
            message,
            notification_type,
            auto_dismiss,
        })
    }
}
