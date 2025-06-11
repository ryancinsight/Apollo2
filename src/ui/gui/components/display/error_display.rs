//! Error display components for Lumidox II Controller GUI
//!
//! This module provides error message and notification display components
//! including error dialogs, toast notifications, inline validation feedback,
//! and recovery options. It implements reusable UI components using the Iced
//! framework for error handling with proper user feedback and recovery mechanisms.
//!
//! The error display system provides:
//! - Error message display with GUI-friendly formatting
//! - Toast notifications for temporary error feedback
//! - Inline validation feedback for input fields
//! - Recovery options and error resolution guidance
//! - Consistent error styling and visual hierarchy

use iced::{
    widget::{column, row, text, container, button, Space},
    Element, Length, Alignment, Color, Background, Border, Theme,
};
use crate::core::{LumidoxError, Result};

/// Error display components and functionality
pub struct ErrorDisplay;

impl ErrorDisplay {
    /// Create error message display
    /// 
    /// Creates a comprehensive error message display with appropriate
    /// styling, error details, and recovery options when available.
    /// 
    /// # Arguments
    /// * `error` - Error to display
    /// * `show_details` - Whether to show detailed error information
    /// * `on_dismiss` - Callback for dismissing the error
    /// * `on_retry` - Optional callback for retry action
    /// 
    /// # Returns
    /// * `Element<Message>` - Error message display element
    /// 
    /// # Example
    /// ```
    /// let error_display = ErrorDisplay::create_error_message(
    ///     &error, true, Message::DismissError, Some(Message::RetryOperation)
    /// )?;
    /// ```
    pub fn create_error_message<Message>(
        error: &LumidoxError,
        show_details: bool,
        on_dismiss: Message,
        on_retry: Option<Message>,
    ) -> Element<'static, Message>
    where
        Message: Clone + 'static,
    {
        let (error_title, error_color) = Self::get_error_display_info(error);
        let error_message = Self::format_error_message(error, show_details);
        
        let title_row = row![
            text("⚠")
                .size(20)
                .style(move |_theme| iced::widget::text::Appearance {
                    color: Some(error_color),
                }),
            Space::with_width(10),
            text(error_title)
                .size(16)
                .style(move |_theme| iced::widget::text::Appearance {
                    color: Some(error_color),
                }),
        ]
        .align_items(Alignment::Center);
        
        let message_text = text(error_message)
            .size(14)
            .horizontal_alignment(iced::alignment::Horizontal::Left);
        
        let mut button_row = row![].spacing(10).align_items(Alignment::Center);
        
        if let Some(retry_msg) = on_retry {
            let retry_button = button(
                text("Retry")
                    .horizontal_alignment(iced::alignment::Horizontal::Center)
            )
            .width(80)
            .height(35)
            .style(Self::retry_button_style())
            .on_press(retry_msg);
            
            button_row = button_row.push(retry_button);
        }
        
        let dismiss_button = button(
            text("Dismiss")
                .horizontal_alignment(iced::alignment::Horizontal::Center)
        )
        .width(80)
        .height(35)
        .style(Self::dismiss_button_style())
        .on_press(on_dismiss);
        
        button_row = button_row.push(dismiss_button);
        
        let error_content = column![
            title_row,
            Space::with_height(10),
            message_text,
            Space::with_height(15),
            container(button_row)
                .width(Length::Fill)
                .center_x(),
        ]
        .spacing(5)
        .align_items(Alignment::Start);
        
        container(error_content)
            .width(Length::Fill)
            .padding(15)
            .style(Self::error_container_style(error_color))
            .into()
    }
    
    /// Create toast notification
    /// 
    /// Creates a temporary toast notification for non-critical errors
    /// and status messages with auto-dismiss functionality.
    /// 
    /// # Arguments
    /// * `message` - Message to display in toast
    /// * `error_type` - Type of error for styling
    /// * `on_dismiss` - Callback for dismissing the toast
    /// 
    /// # Returns
    /// * `Element<Message>` - Toast notification element
    /// 
    /// # Example
    /// ```
    /// let toast = ErrorDisplay::create_toast_notification(
    ///     "Operation completed", ErrorType::Success, Message::DismissToast
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
        let (icon, color) = match error_type {
            ErrorType::Error => ("⚠", Color::from_rgb(0.8, 0.2, 0.2)),
            ErrorType::Warning => ("⚠", Color::from_rgb(0.8, 0.6, 0.2)),
            ErrorType::Success => ("✓", Color::from_rgb(0.2, 0.8, 0.2)),
            ErrorType::Info => ("ℹ", Color::from_rgb(0.2, 0.6, 0.8)),
        };
        
        let toast_content = row![
            text(icon)
                .size(16)
                .style(move |_theme| iced::widget::text::Appearance {
                    color: Some(color),
                }),
            Space::with_width(8),
            text(message)
                .size(14)
                .width(Length::Fill),
            Space::with_width(8),
            button(
                text("×")
                    .size(16)
                    .horizontal_alignment(iced::alignment::Horizontal::Center)
            )
            .width(25)
            .height(25)
            .style(Self::close_button_style())
            .on_press(on_dismiss),
        ]
        .align_items(Alignment::Center)
        .spacing(5);
        
        container(toast_content)
            .width(Length::Fill)
            .padding(10)
            .style(Self::toast_style(color))
            .into()
    }
    
    /// Create inline validation feedback
    /// 
    /// Creates inline validation feedback for input fields with
    /// appropriate styling and error context.
    /// 
    /// # Arguments
    /// * `validation_message` - Validation error message
    /// * `field_name` - Name of the field being validated
    /// 
    /// # Returns
    /// * `Element<Message>` - Inline validation feedback element
    /// 
    /// # Example
    /// ```
    /// let validation = ErrorDisplay::create_inline_validation(
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
        let validation_text = if validation_message.is_empty() {
            format!("{} is valid", field_name)
        } else {
            validation_message.to_string()
        };
        
        let (icon, color) = if validation_message.is_empty() {
            ("✓", Color::from_rgb(0.2, 0.8, 0.2))
        } else {
            ("⚠", Color::from_rgb(0.8, 0.2, 0.2))
        };
        
        let validation_row = row![
            text(icon)
                .size(12)
                .style(move |_theme| iced::widget::text::Appearance {
                    color: Some(color),
                }),
            Space::with_width(5),
            text(validation_text)
                .size(12)
                .style(move |_theme| iced::widget::text::Appearance {
                    color: Some(color),
                }),
        ]
        .align_items(Alignment::Center);
        
        container(validation_row)
            .width(Length::Fill)
            .padding(5)
            .style(Self::validation_style(color))
            .into()
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
    /// let recovery = ErrorDisplay::create_error_recovery(
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
        let recovery_suggestions = Self::get_recovery_suggestions(error);
        
        let suggestion_text = text(recovery_suggestions)
            .size(14)
            .horizontal_alignment(iced::alignment::Horizontal::Left);
        
        let recovery_buttons = row![
            button(
                text("Reconnect")
                    .horizontal_alignment(iced::alignment::Horizontal::Center)
            )
            .width(100)
            .height(35)
            .style(Self::recovery_button_style())
            .on_press(on_reconnect),
            Space::with_width(10),
            button(
                text("Reset")
                    .horizontal_alignment(iced::alignment::Horizontal::Center)
            )
            .width(100)
            .height(35)
            .style(Self::recovery_button_style())
            .on_press(on_reset),
            Space::with_width(10),
            button(
                text("Continue")
                    .horizontal_alignment(iced::alignment::Horizontal::Center)
            )
            .width(100)
            .height(35)
            .style(Self::continue_button_style())
            .on_press(on_continue),
        ]
        .align_items(Alignment::Center);
        
        let recovery_content = column![
            text("Recovery Options")
                .size(16)
                .horizontal_alignment(iced::alignment::Horizontal::Center),
            Space::with_height(10),
            suggestion_text,
            Space::with_height(15),
            container(recovery_buttons)
                .width(Length::Fill)
                .center_x(),
        ]
        .spacing(5)
        .align_items(Alignment::Center);
        
        container(recovery_content)
            .width(Length::Fill)
            .padding(15)
            .style(Self::recovery_container_style())
            .into()
    }
    
    /// Get error display information
    /// 
    /// Extracts display title and color for error presentation.
    /// 
    /// # Arguments
    /// * `error` - Error to analyze
    /// 
    /// # Returns
    /// * `(String, Color)` - Error title and display color
    fn get_error_display_info(error: &LumidoxError) -> (String, Color) {
        match error {
            LumidoxError::DeviceError(_) => (
                "Device Error".to_string(),
                Color::from_rgb(0.8, 0.2, 0.2)
            ),
            LumidoxError::CommunicationError(_) => (
                "Communication Error".to_string(),
                Color::from_rgb(0.8, 0.4, 0.2)
            ),
            LumidoxError::ValidationError(_) => (
                "Validation Error".to_string(),
                Color::from_rgb(0.8, 0.6, 0.2)
            ),
            LumidoxError::SystemError(_) => (
                "System Error".to_string(),
                Color::from_rgb(0.6, 0.2, 0.8)
            ),
            LumidoxError::ConfigError(_) => (
                "Configuration Error".to_string(),
                Color::from_rgb(0.2, 0.4, 0.8)
            ),
        }
    }
    
    /// Format error message for display
    /// 
    /// Formats error message with appropriate detail level.
    /// 
    /// # Arguments
    /// * `error` - Error to format
    /// * `show_details` - Whether to include detailed information
    /// 
    /// # Returns
    /// * `String` - Formatted error message
    fn format_error_message(error: &LumidoxError, show_details: bool) -> String {
        if show_details {
            format!("{}", error)
        } else {
            match error {
                LumidoxError::DeviceError(_) => "A device operation failed. Check device connection and try again.".to_string(),
                LumidoxError::CommunicationError(_) => "Communication with device failed. Verify connection and retry.".to_string(),
                LumidoxError::ValidationError(_) => "Input validation failed. Please check your input and try again.".to_string(),
                LumidoxError::SystemError(_) => "A system error occurred. Please try again or restart the application.".to_string(),
                LumidoxError::ConfigError(_) => "Configuration error detected. Please check settings and try again.".to_string(),
            }
        }
    }
    
    /// Get recovery suggestions for error
    /// 
    /// Provides contextual recovery suggestions based on error type.
    /// 
    /// # Arguments
    /// * `error` - Error to provide suggestions for
    /// 
    /// # Returns
    /// * `String` - Recovery suggestions
    fn get_recovery_suggestions(error: &LumidoxError) -> String {
        match error {
            LumidoxError::DeviceError(_) => "Try reconnecting to the device or check if the device is powered on and properly connected.".to_string(),
            LumidoxError::CommunicationError(_) => "Check the serial connection, verify the correct port is selected, and ensure no other applications are using the device.".to_string(),
            LumidoxError::ValidationError(_) => "Verify your input values are within the acceptable range and format.".to_string(),
            LumidoxError::SystemError(_) => "Try restarting the application or check system resources.".to_string(),
            LumidoxError::ConfigError(_) => "Reset configuration to defaults or check configuration file permissions.".to_string(),
        }
    }

    /// Error container styling
    ///
    /// Provides styling for error message containers.
    ///
    /// # Arguments
    /// * `error_color` - Color for error styling
    ///
    /// # Returns
    /// * Container style theme function
    fn error_container_style(error_color: Color) -> fn(&Theme) -> iced::widget::container::Appearance {
        move |_theme| {
            iced::widget::container::Appearance {
                background: Some(Background::Color(Color::from_rgb(1.0, 0.95, 0.95))),
                border: Border {
                    color: error_color,
                    width: 2.0,
                    radius: 8.0.into(),
                },
                text_color: Some(Color::BLACK),
                ..Default::default()
            }
        }
    }

    /// Toast notification styling
    ///
    /// Provides styling for toast notifications.
    ///
    /// # Arguments
    /// * `color` - Color for toast styling
    ///
    /// # Returns
    /// * Container style theme function
    fn toast_style(color: Color) -> fn(&Theme) -> iced::widget::container::Appearance {
        move |_theme| {
            iced::widget::container::Appearance {
                background: Some(Background::Color(Color::from_rgb(0.95, 0.95, 0.95))),
                border: Border {
                    color,
                    width: 1.0,
                    radius: 5.0.into(),
                },
                text_color: Some(Color::BLACK),
                ..Default::default()
            }
        }
    }

    /// Validation feedback styling
    ///
    /// Provides styling for validation feedback.
    ///
    /// # Arguments
    /// * `color` - Color for validation styling
    ///
    /// # Returns
    /// * Container style theme function
    fn validation_style(color: Color) -> fn(&Theme) -> iced::widget::container::Appearance {
        move |_theme| {
            iced::widget::container::Appearance {
                background: Some(Background::Color(Color::from_rgb(0.98, 0.98, 0.98))),
                border: Border {
                    color,
                    width: 1.0,
                    radius: 3.0.into(),
                },
                text_color: Some(color),
                ..Default::default()
            }
        }
    }

    /// Recovery container styling
    ///
    /// Provides styling for recovery option containers.
    ///
    /// # Returns
    /// * Container style theme function
    fn recovery_container_style() -> fn(&Theme) -> iced::widget::container::Appearance {
        |_theme| {
            iced::widget::container::Appearance {
                background: Some(Background::Color(Color::from_rgb(0.95, 0.98, 1.0))),
                border: Border {
                    color: Color::from_rgb(0.2, 0.6, 0.8),
                    width: 1.0,
                    radius: 8.0.into(),
                },
                text_color: Some(Color::BLACK),
                ..Default::default()
            }
        }
    }

    /// Retry button styling
    ///
    /// Provides styling for retry buttons.
    ///
    /// # Returns
    /// * Button style theme function
    fn retry_button_style() -> fn(&Theme, iced::widget::button::Status) -> iced::widget::button::Appearance {
        |_theme, status| {
            let base_color = Color::from_rgb(0.2, 0.6, 0.8);
            let (background, text_color) = match status {
                iced::widget::button::Status::Active => (base_color, Color::WHITE),
                iced::widget::button::Status::Hovered => (
                    Color::from_rgb(0.3, 0.7, 0.9), Color::WHITE
                ),
                iced::widget::button::Status::Pressed => (
                    Color::from_rgb(0.1, 0.5, 0.7), Color::WHITE
                ),
                iced::widget::button::Status::Disabled => (
                    Color::from_rgb(0.5, 0.5, 0.5), Color::from_rgb(0.7, 0.7, 0.7)
                ),
            };

            iced::widget::button::Appearance {
                background: Some(Background::Color(background)),
                text_color,
                border: Border::with_radius(5),
                ..Default::default()
            }
        }
    }

    /// Dismiss button styling
    ///
    /// Provides styling for dismiss buttons.
    ///
    /// # Returns
    /// * Button style theme function
    fn dismiss_button_style() -> fn(&Theme, iced::widget::button::Status) -> iced::widget::button::Appearance {
        |_theme, status| {
            let base_color = Color::from_rgb(0.6, 0.6, 0.6);
            let (background, text_color) = match status {
                iced::widget::button::Status::Active => (base_color, Color::WHITE),
                iced::widget::button::Status::Hovered => (
                    Color::from_rgb(0.7, 0.7, 0.7), Color::WHITE
                ),
                iced::widget::button::Status::Pressed => (
                    Color::from_rgb(0.5, 0.5, 0.5), Color::WHITE
                ),
                iced::widget::button::Status::Disabled => (
                    Color::from_rgb(0.4, 0.4, 0.4), Color::from_rgb(0.6, 0.6, 0.6)
                ),
            };

            iced::widget::button::Appearance {
                background: Some(Background::Color(background)),
                text_color,
                border: Border::with_radius(5),
                ..Default::default()
            }
        }
    }

    /// Close button styling
    ///
    /// Provides styling for close buttons in toasts.
    ///
    /// # Returns
    /// * Button style theme function
    fn close_button_style() -> fn(&Theme, iced::widget::button::Status) -> iced::widget::button::Appearance {
        |_theme, status| {
            let base_color = Color::from_rgb(0.8, 0.2, 0.2);
            let (background, text_color) = match status {
                iced::widget::button::Status::Active => (Color::TRANSPARENT, base_color),
                iced::widget::button::Status::Hovered => (
                    Color::from_rgb(0.9, 0.3, 0.3), Color::WHITE
                ),
                iced::widget::button::Status::Pressed => (
                    Color::from_rgb(0.7, 0.1, 0.1), Color::WHITE
                ),
                iced::widget::button::Status::Disabled => (
                    Color::TRANSPARENT, Color::from_rgb(0.5, 0.5, 0.5)
                ),
            };

            iced::widget::button::Appearance {
                background: Some(Background::Color(background)),
                text_color,
                border: Border::with_radius(3),
                ..Default::default()
            }
        }
    }

    /// Recovery button styling
    ///
    /// Provides styling for recovery action buttons.
    ///
    /// # Returns
    /// * Button style theme function
    fn recovery_button_style() -> fn(&Theme, iced::widget::button::Status) -> iced::widget::button::Appearance {
        |_theme, status| {
            let base_color = Color::from_rgb(0.2, 0.6, 0.8);
            let (background, text_color) = match status {
                iced::widget::button::Status::Active => (base_color, Color::WHITE),
                iced::widget::button::Status::Hovered => (
                    Color::from_rgb(0.3, 0.7, 0.9), Color::WHITE
                ),
                iced::widget::button::Status::Pressed => (
                    Color::from_rgb(0.1, 0.5, 0.7), Color::WHITE
                ),
                iced::widget::button::Status::Disabled => (
                    Color::from_rgb(0.5, 0.5, 0.5), Color::from_rgb(0.7, 0.7, 0.7)
                ),
            };

            iced::widget::button::Appearance {
                background: Some(Background::Color(background)),
                text_color,
                border: Border::with_radius(5),
                ..Default::default()
            }
        }
    }

    /// Continue button styling
    ///
    /// Provides styling for continue action buttons.
    ///
    /// # Returns
    /// * Button style theme function
    fn continue_button_style() -> fn(&Theme, iced::widget::button::Status) -> iced::widget::button::Appearance {
        |_theme, status| {
            let base_color = Color::from_rgb(0.2, 0.8, 0.2);
            let (background, text_color) = match status {
                iced::widget::button::Status::Active => (base_color, Color::WHITE),
                iced::widget::button::Status::Hovered => (
                    Color::from_rgb(0.3, 0.9, 0.3), Color::WHITE
                ),
                iced::widget::button::Status::Pressed => (
                    Color::from_rgb(0.1, 0.7, 0.1), Color::WHITE
                ),
                iced::widget::button::Status::Disabled => (
                    Color::from_rgb(0.5, 0.5, 0.5), Color::from_rgb(0.7, 0.7, 0.7)
                ),
            };

            iced::widget::button::Appearance {
                background: Some(Background::Color(background)),
                text_color,
                border: Border::with_radius(5),
                ..Default::default()
            }
        }
    }
}

/// Error type for toast notifications and styling
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorType {
    /// Critical error requiring attention
    Error,
    /// Warning that should be noted
    Warning,
    /// Success message
    Success,
    /// Informational message
    Info,
}
