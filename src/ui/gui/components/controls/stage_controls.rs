//! Stage control components for Lumidox II Controller GUI
//!
//! This module provides stage firing control components including individual
//! stage buttons with power information display and custom current input controls.
//! It implements reusable UI components using the Iced framework for stage-related
//! operations with proper error handling and user feedback.
//!
//! The stage controls system provides:
//! - Individual stage firing buttons (1-5) with power information
//! - Custom current input with validation and maximum current checking
//! - Real-time power information display integration
//! - Error handling and user feedback for stage operations
//! - Consistent styling and layout for stage controls

use iced::{
    widget::{button, column, row, text, text_input, container, Space},
    Element, Length, Alignment, Color, Background, Border, Theme,
};
use crate::core::Result;
use crate::device::LumidoxDevice;

/// Stage control components and functionality
pub struct StageControls;

impl StageControls {
    /// Create stage firing buttons with power information
    /// 
    /// Creates a row of stage firing buttons (1-5) with integrated power
    /// information display when available from the device.
    /// 
    /// # Arguments
    /// * `device` - Reference to device for power information queries
    /// * `on_stage_fire` - Callback function for stage firing events
    /// 
    /// # Returns
    /// * `Element<Message>` - Iced element containing stage buttons
    /// 
    /// # Example
    /// ```
    /// let stage_buttons = StageControls::create_stage_buttons(&device, Message::FireStage)?;
    /// ```
    pub fn create_stage_buttons<Message>(
        device: Option<&LumidoxDevice>,
        on_stage_fire: fn(u8) -> Message,
    ) -> Element<'static, Message>
    where
        Message: Clone + 'static,
    {
        let mut stage_row = row![].spacing(10).align_items(Alignment::Center);
        
        for stage in 1..=5 {
            let button_content = if let Some(dev) = device {
                if let Ok(power_info) = dev.get_power_info(stage) {
                    column![
                        text(format!("Stage {}", stage))
                            .size(14)
                            .horizontal_alignment(iced::alignment::Horizontal::Center),
                        text(format!("{} {}", power_info.total_power, power_info.total_units))
                            .size(12)
                            .horizontal_alignment(iced::alignment::Horizontal::Center),
                        text(format!("{} {}", power_info.per_power, power_info.per_units))
                            .size(10)
                            .horizontal_alignment(iced::alignment::Horizontal::Center),
                    ]
                    .align_items(Alignment::Center)
                    .spacing(2)
                } else {
                    column![
                        text(format!("Stage {}", stage))
                            .size(14)
                            .horizontal_alignment(iced::alignment::Horizontal::Center),
                        text("Power: N/A")
                            .size(10)
                            .horizontal_alignment(iced::alignment::Horizontal::Center),
                    ]
                    .align_items(Alignment::Center)
                    .spacing(2)
                }
            } else {
                column![
                    text(format!("Stage {}", stage))
                        .size(14)
                        .horizontal_alignment(iced::alignment::Horizontal::Center),
                    text("No Device")
                        .size(10)
                        .horizontal_alignment(iced::alignment::Horizontal::Center),
                ]
                .align_items(Alignment::Center)
                .spacing(2)
            };
            
            let stage_button = button(
                container(button_content)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_x()
                    .center_y()
            )
            .width(100)
            .height(80)
            .style(Self::stage_button_style())
            .on_press(on_stage_fire(stage));
            
            stage_row = stage_row.push(stage_button);
        }
        
        container(stage_row)
            .width(Length::Fill)
            .padding(10)
            .into()
    }
    
    /// Create custom current input control
    /// 
    /// Creates an input field for custom current firing with validation
    /// and maximum current display when available.
    /// 
    /// # Arguments
    /// * `current_value` - Current input value
    /// * `device` - Reference to device for maximum current information
    /// * `on_current_change` - Callback for current value changes
    /// * `on_fire_custom` - Callback for custom current firing
    /// 
    /// # Returns
    /// * `Element<Message>` - Iced element containing custom current controls
    /// 
    /// # Example
    /// ```
    /// let custom_controls = StageControls::create_custom_current_control(
    ///     &current_value, &device, Message::CurrentChanged, Message::FireCustom
    /// )?;
    /// ```
    pub fn create_custom_current_control<Message>(
        current_value: &str,
        device: Option<&LumidoxDevice>,
        on_current_change: fn(String) -> Message,
        on_fire_custom: Message,
    ) -> Element<'static, Message>
    where
        Message: Clone + 'static,
    {
        let max_current_text = if let Some(dev) = device {
            if let Ok(max_current) = dev.get_max_current() {
                format!("Max: {}mA", max_current)
            } else {
                "Max: Unknown".to_string()
            }
        } else {
            "Max: N/A".to_string()
        };
        
        let current_input = text_input("Enter current (mA)", current_value)
            .on_input(on_current_change)
            .width(150)
            .style(Self::input_style());
        
        let fire_button = button(
            text("Fire Custom")
                .horizontal_alignment(iced::alignment::Horizontal::Center)
        )
        .width(120)
        .height(40)
        .style(Self::fire_button_style())
        .on_press(on_fire_custom);
        
        let custom_row = row![
            column![
                text("Custom Current").size(14),
                text(max_current_text).size(12),
            ]
            .spacing(2),
            Space::with_width(10),
            current_input,
            Space::with_width(10),
            fire_button,
        ]
        .align_items(Alignment::Center)
        .spacing(5);
        
        container(custom_row)
            .width(Length::Fill)
            .padding(10)
            .into()
    }
    
    /// Create complete stage controls section
    /// 
    /// Combines stage buttons and custom current controls into a complete
    /// stage control section with proper layout and spacing.
    /// 
    /// # Arguments
    /// * `current_value` - Current input value for custom firing
    /// * `device` - Reference to device for information queries
    /// * `on_stage_fire` - Callback for stage firing events
    /// * `on_current_change` - Callback for current value changes
    /// * `on_fire_custom` - Callback for custom current firing
    /// 
    /// # Returns
    /// * `Element<Message>` - Complete stage controls section
    /// 
    /// # Example
    /// ```
    /// let stage_section = StageControls::create_stage_section(
    ///     &current_value, &device, Message::FireStage, 
    ///     Message::CurrentChanged, Message::FireCustom
    /// )?;
    /// ```
    pub fn create_stage_section<Message>(
        current_value: &str,
        device: Option<&LumidoxDevice>,
        on_stage_fire: fn(u8) -> Message,
        on_current_change: fn(String) -> Message,
        on_fire_custom: Message,
    ) -> Element<'static, Message>
    where
        Message: Clone + 'static,
    {
        let stage_buttons = Self::create_stage_buttons(device, on_stage_fire);
        let custom_controls = Self::create_custom_current_control(
            current_value, device, on_current_change, on_fire_custom
        );
        
        let section = column![
            text("Stage Controls")
                .size(18)
                .horizontal_alignment(iced::alignment::Horizontal::Center),
            Space::with_height(10),
            stage_buttons,
            Space::with_height(15),
            custom_controls,
        ]
        .spacing(5)
        .align_items(Alignment::Center);
        
        container(section)
            .width(Length::Fill)
            .padding(15)
            .style(Self::section_style())
            .into()
    }
    
    /// Stage button styling
    /// 
    /// Provides consistent styling for stage firing buttons.
    /// 
    /// # Returns
    /// * Button style theme function
    fn stage_button_style() -> fn(&Theme, iced::widget::button::Status) -> iced::widget::button::Appearance {
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
    
    /// Fire button styling
    /// 
    /// Provides styling for the custom current fire button.
    /// 
    /// # Returns
    /// * Button style theme function
    fn fire_button_style() -> fn(&Theme, iced::widget::button::Status) -> iced::widget::button::Appearance {
        |_theme, status| {
            let base_color = Color::from_rgb(0.8, 0.4, 0.2);
            let (background, text_color) = match status {
                iced::widget::button::Status::Active => (base_color, Color::WHITE),
                iced::widget::button::Status::Hovered => (
                    Color::from_rgb(0.9, 0.5, 0.3), Color::WHITE
                ),
                iced::widget::button::Status::Pressed => (
                    Color::from_rgb(0.7, 0.3, 0.1), Color::WHITE
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
