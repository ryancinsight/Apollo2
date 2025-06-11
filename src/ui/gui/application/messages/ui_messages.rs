//! UI interaction messages for Lumidox II Controller GUI
//!
//! This module provides UI interaction message types for the Iced message-driven
//! architecture including view changes, input updates, error handling, and
//! notification management. These messages handle synchronous state updates
//! without requiring async Command operations.
//!
//! The UI messages system provides:
//! - View navigation and state management messages
//! - Input field updates with real-time validation
//! - Error dismissal and notification management
//! - User interface interaction and feedback messages
//! - Immediate state updates for responsive user experience
//! - Integration with the unified state management system

use crate::ui::gui::application::state::{AppView, NotificationType};

/// UI interaction messages
/// 
/// Defines all user interface interaction messages that result in
/// immediate state updates without requiring async operations.
#[derive(Debug, Clone)]
pub enum UiMessage {
    /// View changed to new application view
    /// 
    /// # Arguments
    /// * `view` - New application view to display
    ViewChanged { view: AppView },
    
    /// Input field value updated
    /// 
    /// # Arguments
    /// * `field_name` - Name of the input field
    /// * `value` - New input value
    InputUpdated { field_name: String, value: String },
    
    /// Input field focus changed
    /// 
    /// # Arguments
    /// * `field_name` - Name of the input field
    /// * `focused` - Whether field is now focused
    InputFocusChanged { field_name: String, focused: bool },
    
    /// Error message dismissed by user
    ErrorDismissed,
    
    /// Notification dismissed by user
    NotificationDismissed,
    
    /// Show notification to user
    /// 
    /// # Arguments
    /// * `message` - Notification message text
    /// * `notification_type` - Type of notification for styling
    /// * `auto_dismiss` - Optional auto-dismiss timer in seconds
    ShowNotification {
        message: String,
        notification_type: NotificationType,
        auto_dismiss: Option<u32>,
    },
    
    /// Refresh requested by user
    RefreshRequested,
    
    /// Settings changed
    /// 
    /// # Arguments
    /// * `setting_name` - Name of the setting
    /// * `value` - New setting value
    SettingChanged { setting_name: String, value: String },
    
    /// Layout mode changed
    /// 
    /// # Arguments
    /// * `compact` - Whether to use compact layout
    LayoutModeChanged { compact: bool },
    
    /// Theme changed
    /// 
    /// # Arguments
    /// * `theme_name` - Name of the new theme
    ThemeChanged { theme_name: String },
    
    /// Window resized
    /// 
    /// # Arguments
    /// * `width` - New window width
    /// * `height` - New window height
    WindowResized { width: u32, height: u32 },
    
    /// Application tick for periodic updates
    Tick,
    
    /// Clear all input fields
    ClearAllInputs,
    
    /// Reset application state
    ResetState,
    
    /// Toggle verbose mode
    ToggleVerbose,
    
    /// Toggle optimization mode
    ToggleOptimization,
}

impl UiMessage {
    /// Check if message affects input validation
    /// 
    /// Used to determine if validation should be re-run after
    /// processing this message.
    /// 
    /// # Returns
    /// * `bool` - True if message affects validation state
    /// 
    /// # Example
    /// ```
    /// let input_msg = UiMessage::InputUpdated { 
    ///     field_name: "arm_current".to_string(), 
    ///     value: "1000".to_string() 
    /// };
    /// assert!(input_msg.affects_validation());
    /// 
    /// let view_msg = UiMessage::ViewChanged { view: AppView::Main };
    /// assert!(!view_msg.affects_validation());
    /// ```
    pub fn affects_validation(&self) -> bool {
        match self {
            UiMessage::InputUpdated { .. } |
            UiMessage::ClearAllInputs |
            UiMessage::ResetState => true,
            _ => false,
        }
    }
    
    /// Check if message requires immediate UI update
    /// 
    /// Used to determine if the UI should be immediately refreshed
    /// after processing this message.
    /// 
    /// # Returns
    /// * `bool` - True if immediate UI update is required
    /// 
    /// # Example
    /// ```
    /// let notification_msg = UiMessage::ShowNotification { 
    ///     message: "Success".to_string(),
    ///     notification_type: NotificationType::Success,
    ///     auto_dismiss: Some(3)
    /// };
    /// assert!(notification_msg.requires_immediate_update());
    /// ```
    pub fn requires_immediate_update(&self) -> bool {
        match self {
            UiMessage::ShowNotification { .. } |
            UiMessage::ErrorDismissed |
            UiMessage::NotificationDismissed |
            UiMessage::ViewChanged { .. } |
            UiMessage::LayoutModeChanged { .. } |
            UiMessage::ThemeChanged { .. } |
            UiMessage::WindowResized { .. } => true,
            _ => false,
        }
    }
    
    /// Get message priority for processing order
    /// 
    /// Returns priority level for message processing to ensure
    /// critical UI updates are handled first.
    /// 
    /// # Returns
    /// * `MessagePriority` - Priority level of the message
    /// 
    /// # Example
    /// ```
    /// let error_msg = UiMessage::ErrorDismissed;
    /// assert_eq!(error_msg.get_priority(), MessagePriority::High);
    /// ```
    pub fn get_priority(&self) -> MessagePriority {
        match self {
            UiMessage::ErrorDismissed |
            UiMessage::ShowNotification { notification_type: NotificationType::Error, .. } => {
                MessagePriority::High
            }
            UiMessage::ViewChanged { .. } |
            UiMessage::NotificationDismissed |
            UiMessage::WindowResized { .. } => {
                MessagePriority::Medium
            }
            UiMessage::InputUpdated { .. } |
            UiMessage::InputFocusChanged { .. } |
            UiMessage::RefreshRequested => {
                MessagePriority::Normal
            }
            UiMessage::Tick |
            UiMessage::SettingChanged { .. } |
            UiMessage::LayoutModeChanged { .. } |
            UiMessage::ThemeChanged { .. } => {
                MessagePriority::Low
            }
            _ => MessagePriority::Normal,
        }
    }
    
    /// Get message category for logging and debugging
    /// 
    /// Categorizes messages for better organization and debugging.
    /// 
    /// # Returns
    /// * `MessageCategory` - Category of the message
    /// 
    /// # Example
    /// ```
    /// let input_msg = UiMessage::InputUpdated { 
    ///     field_name: "arm_current".to_string(), 
    ///     value: "1000".to_string() 
    /// };
    /// assert_eq!(input_msg.get_category(), MessageCategory::Input);
    /// ```
    pub fn get_category(&self) -> MessageCategory {
        match self {
            UiMessage::ViewChanged { .. } => MessageCategory::Navigation,
            UiMessage::InputUpdated { .. } |
            UiMessage::InputFocusChanged { .. } |
            UiMessage::ClearAllInputs => MessageCategory::Input,
            UiMessage::ErrorDismissed |
            UiMessage::NotificationDismissed |
            UiMessage::ShowNotification { .. } => MessageCategory::Notification,
            UiMessage::SettingChanged { .. } |
            UiMessage::LayoutModeChanged { .. } |
            UiMessage::ThemeChanged { .. } |
            UiMessage::ToggleVerbose |
            UiMessage::ToggleOptimization => MessageCategory::Settings,
            UiMessage::WindowResized { .. } => MessageCategory::Layout,
            UiMessage::RefreshRequested |
            UiMessage::ResetState |
            UiMessage::Tick => MessageCategory::System,
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
    /// let view_msg = UiMessage::ViewChanged { view: AppView::Settings };
    /// assert_eq!(view_msg.get_description(), "View changed to Settings");
    /// ```
    pub fn get_description(&self) -> String {
        match self {
            UiMessage::ViewChanged { view } => format!("View changed to {:?}", view),
            UiMessage::InputUpdated { field_name, .. } => format!("Input updated: {}", field_name),
            UiMessage::InputFocusChanged { field_name, focused } => {
                format!("Input focus {}: {}", if *focused { "gained" } else { "lost" }, field_name)
            }
            UiMessage::ErrorDismissed => "Error dismissed".to_string(),
            UiMessage::NotificationDismissed => "Notification dismissed".to_string(),
            UiMessage::ShowNotification { message, notification_type, .. } => {
                format!("Show {:?} notification: {}", notification_type, message)
            }
            UiMessage::RefreshRequested => "Refresh requested".to_string(),
            UiMessage::SettingChanged { setting_name, .. } => format!("Setting changed: {}", setting_name),
            UiMessage::LayoutModeChanged { compact } => {
                format!("Layout mode changed to {}", if *compact { "compact" } else { "normal" })
            }
            UiMessage::ThemeChanged { theme_name } => format!("Theme changed to {}", theme_name),
            UiMessage::WindowResized { width, height } => format!("Window resized to {}x{}", width, height),
            UiMessage::Tick => "Application tick".to_string(),
            UiMessage::ClearAllInputs => "Clear all inputs".to_string(),
            UiMessage::ResetState => "Reset application state".to_string(),
            UiMessage::ToggleVerbose => "Toggle verbose mode".to_string(),
            UiMessage::ToggleOptimization => "Toggle optimization mode".to_string(),
        }
    }
}

/// Message priority levels
/// 
/// Defines priority levels for message processing to ensure
/// critical updates are handled appropriately.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum MessagePriority {
    /// Low priority - can be deferred
    Low,
    /// Normal priority - standard processing
    Normal,
    /// Medium priority - should be processed promptly
    Medium,
    /// High priority - requires immediate processing
    High,
}

/// Message categories
/// 
/// Categorizes UI messages for better organization and debugging.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MessageCategory {
    /// Navigation and view changes
    Navigation,
    /// Input field interactions
    Input,
    /// Notifications and error handling
    Notification,
    /// Settings and configuration
    Settings,
    /// Layout and visual changes
    Layout,
    /// System and lifecycle messages
    System,
}

/// UI message utilities
/// 
/// Provides utility functions for UI message processing and management.
pub struct UiMessageUtils;

impl UiMessageUtils {
    /// Create input update message
    /// 
    /// Convenience function for creating input update messages.
    /// 
    /// # Arguments
    /// * `field_name` - Name of the input field
    /// * `value` - New input value
    /// 
    /// # Returns
    /// * `UiMessage` - Input update message
    /// 
    /// # Example
    /// ```
    /// let msg = UiMessageUtils::input_updated("arm_current", "1000");
    /// ```
    pub fn input_updated(field_name: &str, value: &str) -> UiMessage {
        UiMessage::InputUpdated {
            field_name: field_name.to_string(),
            value: value.to_string(),
        }
    }
    
    /// Create notification message
    /// 
    /// Convenience function for creating notification messages.
    /// 
    /// # Arguments
    /// * `message` - Notification message text
    /// * `notification_type` - Type of notification
    /// * `auto_dismiss` - Optional auto-dismiss timer
    /// 
    /// # Returns
    /// * `UiMessage` - Notification message
    /// 
    /// # Example
    /// ```
    /// let msg = UiMessageUtils::show_notification(
    ///     "Operation completed", 
    ///     NotificationType::Success, 
    ///     Some(3)
    /// );
    /// ```
    pub fn show_notification(
        message: &str,
        notification_type: NotificationType,
        auto_dismiss: Option<u32>,
    ) -> UiMessage {
        UiMessage::ShowNotification {
            message: message.to_string(),
            notification_type,
            auto_dismiss,
        }
    }
    
    /// Create view change message
    /// 
    /// Convenience function for creating view change messages.
    /// 
    /// # Arguments
    /// * `view` - New application view
    /// 
    /// # Returns
    /// * `UiMessage` - View change message
    /// 
    /// # Example
    /// ```
    /// let msg = UiMessageUtils::view_changed(AppView::Settings);
    /// ```
    pub fn view_changed(view: AppView) -> UiMessage {
        UiMessage::ViewChanged { view }
    }
}
