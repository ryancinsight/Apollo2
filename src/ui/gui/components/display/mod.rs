//! Display components module for Lumidox II Controller GUI
//!
//! This module organizes and exports all display-related UI components
//! for the Lumidox II Controller GUI application. It provides a centralized
//! access point for status display, information display, and error display
//! components with proper module organization and re-exports.
//!
//! The display module includes:
//! - Device status and connection status display
//! - Device information and parameter display matching CLI output
//! - Error messages, notifications, and recovery options
//! - Consistent styling and error handling across all displays
//! - Reusable components following the Iced framework patterns

// Import display component modules
pub mod status_display;
pub mod info_display;
pub mod error_display;

// Re-export display components for easy access
pub use status_display::StatusDisplay;
pub use info_display::InfoDisplay;
pub use error_display::{ErrorDisplay, ErrorType};

use iced::{
    widget::{column, container, Space},
    Element, Length, Alignment,
};
use crate::core::{LumidoxError, Result};
use crate::device::LumidoxDevice;

/// Display components coordinator
/// 
/// Provides high-level coordination and organization of all display
/// components with consistent layout and styling patterns.
pub struct DisplayComponents;

impl DisplayComponents {
    /// Create complete information panel
    /// 
    /// Combines status display and device information into a complete
    /// information panel with proper layout and spacing.
    /// 
    /// # Arguments
    /// * `device` - Reference to device for information queries
    /// * `is_connected` - Whether device is currently connected
    /// 
    /// # Returns
    /// * `Element<Message>` - Complete information panel
    /// 
    /// # Example
    /// ```
    /// let info_panel = DisplayComponents::create_info_panel(&device, true)?;
    /// ```
    pub fn create_info_panel<Message>(
        device: Option<&LumidoxDevice>,
        is_connected: bool,
    ) -> Element<'static, Message>
    where
        Message: Clone + 'static,
    {
        let status_display = StatusDisplay::create_device_status(device, is_connected);
        let info_display = InfoDisplay::create_device_info(device);
        
        let info_panel = column![
            status_display,
            Space::with_height(20),
            info_display,
        ]
        .spacing(10)
        .align_items(Alignment::Center);
        
        container(info_panel)
            .width(Length::Fill)
            .padding(20)
            .into()
    }
    
    /// Create status display only
    /// 
    /// Creates only the status display section for layouts that need
    /// individual display sections.
    /// 
    /// # Arguments
    /// * `device` - Reference to device for status information
    /// * `is_connected` - Whether device is currently connected
    /// 
    /// # Returns
    /// * `Element<Message>` - Status display section
    pub fn create_status_display_only<Message>(
        device: Option<&LumidoxDevice>,
        is_connected: bool,
    ) -> Element<'static, Message>
    where
        Message: Clone + 'static,
    {
        StatusDisplay::create_device_status(device, is_connected)
    }
    
    /// Create status summary for compact layouts
    /// 
    /// Creates a compact status summary for space-constrained layouts
    /// such as headers or sidebars.
    /// 
    /// # Arguments
    /// * `device` - Reference to device for status information
    /// * `is_connected` - Whether device is currently connected
    /// 
    /// # Returns
    /// * `Element<Message>` - Compact status summary
    pub fn create_status_summary<Message>(
        device: Option<&LumidoxDevice>,
        is_connected: bool,
    ) -> Element<'static, Message>
    where
        Message: Clone + 'static,
    {
        StatusDisplay::create_status_summary(device, is_connected)
    }
    
    /// Create device information display only
    /// 
    /// Creates only the device information section for layouts that need
    /// individual display sections.
    /// 
    /// # Arguments
    /// * `device` - Reference to device for information queries
    /// 
    /// # Returns
    /// * `Element<Message>` - Device information section
    pub fn create_info_display_only<Message>(
        device: Option<&LumidoxDevice>,
    ) -> Element<'static, Message>
    where
        Message: Clone + 'static,
    {
        InfoDisplay::create_device_info(device)
    }
    
    /// Create error notification
    /// 
    /// Creates an error notification with appropriate styling and
    /// recovery options based on error type.
    /// 
    /// # Arguments
    /// * `error` - Error to display
    /// * `show_details` - Whether to show detailed error information
    /// * `on_dismiss` - Callback for dismissing the error
    /// * `on_retry` - Optional callback for retry action
    /// 
    /// # Returns
    /// * `Element<Message>` - Error notification element
    /// 
    /// # Example
    /// ```
    /// let error_notification = DisplayComponents::create_error_notification(
    ///     &error, true, Message::DismissError, Some(Message::RetryOperation)
    /// );
    /// ```
    pub fn create_error_notification<Message>(
        error: &LumidoxError,
        show_details: bool,
        on_dismiss: Message,
        on_retry: Option<Message>,
    ) -> Element<'static, Message>
    where
        Message: Clone + 'static,
    {
        ErrorDisplay::create_error_message(error, show_details, on_dismiss, on_retry)
    }
    
    /// Create toast notification
    /// 
    /// Creates a temporary toast notification for status updates
    /// and non-critical messages.
    /// 
    /// # Arguments
    /// * `message` - Message to display in toast
    /// * `error_type` - Type of message for styling
    /// * `on_dismiss` - Callback for dismissing the toast
    /// 
    /// # Returns
    /// * `Element<Message>` - Toast notification element
    /// 
    /// # Example
    /// ```
    /// let toast = DisplayComponents::create_toast_notification(
    ///     "Operation completed successfully", ErrorType::Success, Message::DismissToast
    /// );
    /// ```
    pub fn create_toast_notification<Message>(
        message: &str,
        error_type: ErrorType,
        on_dismiss: Message,
    ) -> Element<'static, Message>
    where
        Message: Clone + 'static,
    {
        ErrorDisplay::create_toast_notification(message, error_type, on_dismiss)
    }
    
    /// Create inline validation feedback
    /// 
    /// Creates inline validation feedback for input fields with
    /// appropriate styling and error context.
    /// 
    /// # Arguments
    /// * `validation_message` - Validation error message (empty if valid)
    /// * `field_name` - Name of the field being validated
    /// 
    /// # Returns
    /// * `Element<Message>` - Inline validation feedback element
    /// 
    /// # Example
    /// ```
    /// let validation = DisplayComponents::create_inline_validation(
    ///     "Current value too high", "ARM Current"
    /// );
    /// ```
    pub fn create_inline_validation<Message>(
        validation_message: &str,
        field_name: &str,
    ) -> Element<'static, Message>
    where
        Message: Clone + 'static,
    {
        ErrorDisplay::create_inline_validation(validation_message, field_name)
    }
    
    /// Create error recovery options
    /// 
    /// Creates a set of recovery options for error scenarios with
    /// appropriate actions and guidance.
    /// 
    /// # Arguments
    /// * `error` - Error that occurred
    /// * `on_reconnect` - Callback for reconnection attempt
    /// * `on_reset` - Callback for reset operation
    /// * `on_continue` - Callback for continuing despite error
    /// 
    /// # Returns
    /// * `Element<Message>` - Error recovery options element
    /// 
    /// # Example
    /// ```
    /// let recovery = DisplayComponents::create_error_recovery(
    ///     &error, Message::Reconnect, Message::Reset, Message::Continue
    /// );
    /// ```
    pub fn create_error_recovery<Message>(
        error: &LumidoxError,
        on_reconnect: Message,
        on_reset: Message,
        on_continue: Message,
    ) -> Element<'static, Message>
    where
        Message: Clone + 'static,
    {
        ErrorDisplay::create_error_recovery(error, on_reconnect, on_reset, on_continue)
    }
    
    /// Format error for display
    /// 
    /// Formats an error for GUI display with appropriate detail level
    /// and user-friendly messaging.
    /// 
    /// # Arguments
    /// * `error` - Error to format
    /// * `show_technical_details` - Whether to include technical details
    /// 
    /// # Returns
    /// * `String` - Formatted error message for GUI display
    /// 
    /// # Example
    /// ```
    /// let formatted_error = DisplayComponents::format_error_for_gui(&error, false);
    /// ```
    pub fn format_error_for_gui(error: &LumidoxError, show_technical_details: bool) -> String {
        if show_technical_details {
            format!("{}", error)
        } else {
            match error {
                LumidoxError::DeviceError(_) => {
                    "Device operation failed. Please check the device connection and try again.".to_string()
                }
                LumidoxError::CommunicationError(_) => {
                    "Communication with device failed. Please verify the connection and retry.".to_string()
                }
                LumidoxError::ValidationError(_) => {
                    "Input validation failed. Please check your input values and try again.".to_string()
                }
                LumidoxError::SystemError(_) => {
                    "A system error occurred. Please try again or restart the application.".to_string()
                }
                LumidoxError::ConfigError(_) => {
                    "Configuration error detected. Please check your settings and try again.".to_string()
                }
            }
        }
    }
    
    /// Get error severity level
    /// 
    /// Determines the severity level of an error for appropriate
    /// display styling and user response.
    /// 
    /// # Arguments
    /// * `error` - Error to analyze
    /// 
    /// # Returns
    /// * `ErrorType` - Severity level of the error
    /// 
    /// # Example
    /// ```
    /// let severity = DisplayComponents::get_error_severity(&error);
    /// ```
    pub fn get_error_severity(error: &LumidoxError) -> ErrorType {
        match error {
            LumidoxError::DeviceError(_) => ErrorType::Error,
            LumidoxError::CommunicationError(_) => ErrorType::Error,
            LumidoxError::ValidationError(_) => ErrorType::Warning,
            LumidoxError::SystemError(_) => ErrorType::Error,
            LumidoxError::ConfigError(_) => ErrorType::Warning,
        }
    }
}
