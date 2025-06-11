//! Current control components for Lumidox II Controller GUI
//!
//! This module provides ARM current setting control components including
//! current input fields, validation, and setting operations. It implements
//! reusable UI components using the Iced framework for ARM current management
//! with proper error handling and user feedback.
//!
//! The current controls system provides:
//! - ARM current input field with validation
//! - Current setting button with confirmation
//! - Real-time current value display and validation
//! - Error handling and user feedback for current operations
//! - Consistent styling and layout for current controls

use iced::{
    widget::{button, column, row, text, text_input, container, Space},
    Element, Length, Alignment, Color, Background, Border, Theme,
};
use crate::core::Result;
use crate::device::LumidoxDevice;

/// Current control components and functionality
pub struct CurrentControls;

impl CurrentControls {
    /// Create ARM current input control
    /// 
    /// Creates an input field for setting ARM current with validation
    /// and current value display when available from the device.
    /// 
    /// # Arguments
    /// * `current_value` - Current input value
    /// * `device` - Reference to device for current information
    /// * `on_current_change` - Callback for current value changes
    /// * `on_set_current` - Callback for setting ARM current
    /// 
    /// # Returns
    /// * `Element<Message>` - Iced element containing ARM current controls
    /// 
    /// # Example
    /// ```
    /// let current_controls = CurrentControls::create_arm_current_control(
    ///     &current_value, &device, Message::ArmCurrentChanged, Message::SetArmCurrent
    /// )?;
    /// ```
    pub fn create_arm_current_control<Message>(
        current_value: &str,
        device: Option<&LumidoxDevice>,
        on_current_change: fn(String) -> Message,
        on_set_current: Message,
    ) -> Element<'static, Message>
    where
        Message: Clone + 'static,
    {
        let current_arm_text = if let Some(dev) = device {
            if let Ok(arm_current) = dev.read_arm_current() {
                format!("Current ARM: {}mA", arm_current)
            } else {
                "Current ARM: Unknown".to_string()
            }
        } else {
            "Current ARM: N/A".to_string()
        };
        
        let max_current_text = if let Some(dev) = device {
            if let Ok(max_current) = dev.get_max_current() {
                format!("Max: {}mA", max_current)
            } else {
                "Max: Unknown".to_string()
            }
        } else {
            "Max: N/A".to_string()
        };
        
        let current_input = text_input("Enter ARM current (mA)", current_value)
            .on_input(on_current_change)
            .width(150)
            .style(Self::input_style());
        
        let set_button = button(
            text("Set ARM Current")
                .horizontal_alignment(iced::alignment::Horizontal::Center)
        )
        .width(140)
        .height(40)
        .style(Self::set_button_style())
        .on_press(on_set_current);
        
        let current_row = row![
            column![
                text("ARM Current Setting").size(14),
                text(current_arm_text).size(12),
                text(max_current_text).size(10),
            ]
            .spacing(2),
            Space::with_width(10),
            current_input,
            Space::with_width(10),
            set_button,
        ]
        .align_items(Alignment::Center)
        .spacing(5);
        
        container(current_row)
            .width(Length::Fill)
            .padding(10)
            .into()
    }
    
    /// Create current validation display
    /// 
    /// Creates a display showing current validation status and any
    /// validation errors or warnings for user feedback.
    /// 
    /// # Arguments
    /// * `current_value` - Current input value to validate
    /// * `device` - Reference to device for validation context
    /// * `validation_message` - Optional validation message to display
    /// 
    /// # Returns
    /// * `Element<Message>` - Validation display element
    /// 
    /// # Example
    /// ```
    /// let validation_display = CurrentControls::create_validation_display(
    ///     &current_value, &device, Some("Invalid current value")
    /// )?;
    /// ```
    pub fn create_validation_display<Message>(
        current_value: &str,
        device: Option<&LumidoxDevice>,
        validation_message: Option<&str>,
    ) -> Element<'static, Message>
    where
        Message: Clone + 'static,
    {
        let validation_text = if let Some(message) = validation_message {
            message.to_string()
        } else if current_value.is_empty() {
            "Enter a current value".to_string()
        } else {
            match current_value.parse::<u16>() {
                Ok(value) => {
                    if let Some(dev) = device {
                        if let Ok(max_current) = dev.get_max_current() {
                            if value > max_current {
                                format!("Current too high (max: {}mA)", max_current)
                            } else if value == 0 {
                                "Current cannot be zero".to_string()
                            } else {
                                "Valid current value".to_string()
                            }
                        } else {
                            "Cannot validate - device error".to_string()
                        }
                    } else {
                        "Cannot validate - no device".to_string()
                    }
                }
                Err(_) => "Invalid number format".to_string()
            }
        };
        
        let (text_color, background_color) = if validation_message.is_some() {
            (Color::from_rgb(0.8, 0.2, 0.2), Color::from_rgb(1.0, 0.9, 0.9))
        } else if current_value.is_empty() {
            (Color::from_rgb(0.5, 0.5, 0.5), Color::from_rgb(0.95, 0.95, 0.95))
        } else {
            match current_value.parse::<u16>() {
                Ok(value) => {
                    if let Some(dev) = device {
                        if let Ok(max_current) = dev.get_max_current() {
                            if value > max_current || value == 0 {
                                (Color::from_rgb(0.8, 0.2, 0.2), Color::from_rgb(1.0, 0.9, 0.9))
                            } else {
                                (Color::from_rgb(0.2, 0.6, 0.2), Color::from_rgb(0.9, 1.0, 0.9))
                            }
                        } else {
                            (Color::from_rgb(0.8, 0.6, 0.2), Color::from_rgb(1.0, 0.95, 0.9))
                        }
                    } else {
                        (Color::from_rgb(0.5, 0.5, 0.5), Color::from_rgb(0.95, 0.95, 0.95))
                    }
                }
                Err(_) => (Color::from_rgb(0.8, 0.2, 0.2), Color::from_rgb(1.0, 0.9, 0.9))
            }
        };
        
        let validation_display = container(
            text(validation_text)
                .size(12)
                .horizontal_alignment(iced::alignment::Horizontal::Center)
                .style(move |_theme| {
                    iced::widget::text::Appearance {
                        color: Some(text_color),
                    }
                })
        )
        .width(Length::Fill)
        .padding(8)
        .style(move |_theme| {
            iced::widget::container::Appearance {
                background: Some(Background::Color(background_color)),
                border: Border {
                    color: text_color,
                    width: 1.0,
                    radius: 5.0.into(),
                },
                text_color: Some(text_color),
                ..Default::default()
            }
        });
        
        validation_display.into()
    }
    
    /// Create complete current controls section
    /// 
    /// Combines ARM current controls and validation display into a complete
    /// current control section with proper layout and spacing.
    /// 
    /// # Arguments
    /// * `current_value` - Current input value
    /// * `device` - Reference to device for information and operations
    /// * `validation_message` - Optional validation message to display
    /// * `on_current_change` - Callback for current value changes
    /// * `on_set_current` - Callback for setting ARM current
    /// 
    /// # Returns
    /// * `Element<Message>` - Complete current controls section
    /// 
    /// # Example
    /// ```
    /// let current_section = CurrentControls::create_current_section(
    ///     &current_value, &device, None, Message::ArmCurrentChanged, Message::SetArmCurrent
    /// )?;
    /// ```
    pub fn create_current_section<Message>(
        current_value: &str,
        device: Option<&LumidoxDevice>,
        validation_message: Option<&str>,
        on_current_change: fn(String) -> Message,
        on_set_current: Message,
    ) -> Element<'static, Message>
    where
        Message: Clone + 'static,
    {
        let current_controls = Self::create_arm_current_control(
            current_value, device, on_current_change, on_set_current
        );
        let validation_display = Self::create_validation_display(
            current_value, device, validation_message
        );
        
        let section = column![
            text("Current Controls")
                .size(18)
                .horizontal_alignment(iced::alignment::Horizontal::Center),
            Space::with_height(10),
            current_controls,
            Space::with_height(10),
            validation_display,
        ]
        .spacing(5)
        .align_items(Alignment::Center);
        
        container(section)
            .width(Length::Fill)
            .padding(15)
            .style(Self::section_style())
            .into()
    }
    
    /// Set button styling
    /// 
    /// Provides styling for the set ARM current button.
    /// 
    /// # Returns
    /// * Button style theme function
    fn set_button_style() -> fn(&Theme, iced::widget::button::Status) -> iced::widget::button::Appearance {
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
    
    /// Input field styling
    /// 
    /// Provides styling for text input fields.
    /// 
    /// # Returns
    /// * Text input style theme function
    fn input_style() -> fn(&Theme, iced::widget::text_input::Status) -> iced::widget::text_input::Appearance {
        |_theme, status| {
            let base_color = Color::WHITE;
            let border_color = match status {
                iced::widget::text_input::Status::Active => Color::from_rgb(0.2, 0.6, 0.8),
                iced::widget::text_input::Status::Hovered => Color::from_rgb(0.3, 0.7, 0.9),
                iced::widget::text_input::Status::Focused => Color::from_rgb(0.1, 0.5, 0.7),
                iced::widget::text_input::Status::Disabled => Color::from_rgb(0.5, 0.5, 0.5),
            };
            
            iced::widget::text_input::Appearance {
                background: Background::Color(base_color),
                border: Border {
                    color: border_color,
                    width: 2.0,
                    radius: 5.0.into(),
                },
                icon_color: Color::from_rgb(0.5, 0.5, 0.5),
                placeholder_color: Color::from_rgb(0.7, 0.7, 0.7),
                value_color: Color::BLACK,
                selection_color: Color::from_rgb(0.2, 0.6, 0.8),
            }
        }
    }
    
    /// Section container styling
    /// 
    /// Provides styling for section containers.
    /// 
    /// # Returns
    /// * Container style theme function
    fn section_style() -> fn(&Theme) -> iced::widget::container::Appearance {
        |_theme| {
            iced::widget::container::Appearance {
                background: Some(Background::Color(Color::from_rgb(0.95, 0.95, 0.95))),
                border: Border {
                    color: Color::from_rgb(0.8, 0.8, 0.8),
                    width: 1.0,
                    radius: 8.0.into(),
                },
                text_color: Some(Color::BLACK),
                ..Default::default()
            }
        }
    }
}
