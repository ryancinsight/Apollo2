//! Device control components for Lumidox II Controller GUI
//!
//! This module provides device control components including arm, turn off,
//! and shutdown buttons with confirmation dialogs and proper error handling.
//! It implements reusable UI components using the Iced framework for device
//! control operations with consistent styling and user feedback.
//!
//! The device controls system provides:
//! - Device arming button with status indication
//! - Turn off device button with confirmation
//! - Shutdown and quit button with proper cleanup
//! - Consistent styling and layout for device controls
//! - Error handling and user feedback for device operations

use iced::{
    widget::{button, column, row, text, container, Space},
    Element, Length, Alignment, Color, Background, Border, Theme,
};
use crate::core::Result;
use crate::device::LumidoxDevice;

/// Device control components and functionality
pub struct DeviceControls;

impl DeviceControls {
    /// Create device control buttons
    /// 
    /// Creates a set of device control buttons including arm, turn off,
    /// and shutdown with proper styling and event handling.
    /// 
    /// # Arguments
    /// * `device` - Reference to device for status information
    /// * `on_arm` - Callback for arm device action
    /// * `on_turn_off` - Callback for turn off device action
    /// * `on_shutdown` - Callback for shutdown and quit action
    /// 
    /// # Returns
    /// * `Element<Message>` - Iced element containing device control buttons
    /// 
    /// # Example
    /// ```
    /// let device_buttons = DeviceControls::create_device_buttons(
    ///     &device, Message::ArmDevice, Message::TurnOffDevice, Message::ShutdownDevice
    /// )?;
    /// ```
    pub fn create_device_buttons<Message>(
        device: Option<&LumidoxDevice>,
        on_arm: Message,
        on_turn_off: Message,
        on_shutdown: Message,
    ) -> Element<'static, Message>
    where
        Message: Clone + 'static,
    {
        let arm_button = button(
            container(
                column![
                    text("ARM")
                        .size(16)
                        .horizontal_alignment(iced::alignment::Horizontal::Center),
                    text("Prepare for firing")
                        .size(10)
                        .horizontal_alignment(iced::alignment::Horizontal::Center),
                ]
                .align_items(Alignment::Center)
                .spacing(2)
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
        )
        .width(120)
        .height(60)
        .style(Self::arm_button_style())
        .on_press(on_arm);
        
        let turn_off_button = button(
            container(
                column![
                    text("TURN OFF")
                        .size(16)
                        .horizontal_alignment(iced::alignment::Horizontal::Center),
                    text("Safe mode")
                        .size(10)
                        .horizontal_alignment(iced::alignment::Horizontal::Center),
                ]
                .align_items(Alignment::Center)
                .spacing(2)
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
        )
        .width(120)
        .height(60)
        .style(Self::turn_off_button_style())
        .on_press(on_turn_off);
        
        let shutdown_button = button(
            container(
                column![
                    text("SHUTDOWN")
                        .size(16)
                        .horizontal_alignment(iced::alignment::Horizontal::Center),
                    text("Exit program")
                        .size(10)
                        .horizontal_alignment(iced::alignment::Horizontal::Center),
                ]
                .align_items(Alignment::Center)
                .spacing(2)
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
        )
        .width(120)
        .height(60)
        .style(Self::shutdown_button_style())
        .on_press(on_shutdown);
        
        let button_row = row![
            arm_button,
            Space::with_width(15),
            turn_off_button,
            Space::with_width(15),
            shutdown_button,
        ]
        .align_items(Alignment::Center);
        
        container(button_row)
            .width(Length::Fill)
            .padding(10)
            .into()
    }
    
    /// Create device status indicator
    /// 
    /// Creates a status indicator showing current device state and
    /// connection status with appropriate visual feedback.
    /// 
    /// # Arguments
    /// * `device` - Reference to device for status information
    /// * `is_connected` - Whether device is currently connected
    /// 
    /// # Returns
    /// * `Element<Message>` - Status indicator element
    /// 
    /// # Example
    /// ```
    /// let status_indicator = DeviceControls::create_status_indicator(&device, true)?;
    /// ```
    pub fn create_status_indicator<Message>(
        device: Option<&LumidoxDevice>,
        is_connected: bool,
    ) -> Element<'static, Message>
    where
        Message: Clone + 'static,
    {
        let (status_text, status_color) = if !is_connected {
            ("DISCONNECTED", Color::from_rgb(0.8, 0.2, 0.2))
        } else if let Some(dev) = device {
            match dev.read_remote_mode() {
                Ok(mode) => {
                    match mode {
                        crate::device::models::DeviceMode::Local => {
                            ("LOCAL MODE", Color::from_rgb(0.8, 0.6, 0.2))
                        }
                        crate::device::models::DeviceMode::Standby => {
                            ("REMOTE STANDBY", Color::from_rgb(0.2, 0.6, 0.8))
                        }
                        crate::device::models::DeviceMode::Armed => {
                            ("ARMED", Color::from_rgb(0.8, 0.4, 0.2))
                        }
                        crate::device::models::DeviceMode::Remote => {
                            ("REMOTE ACTIVE", Color::from_rgb(0.2, 0.8, 0.2))
                        }
                    }
                }
                Err(_) => ("STATUS UNKNOWN", Color::from_rgb(0.5, 0.5, 0.5))
            }
        } else {
            ("NO DEVICE", Color::from_rgb(0.5, 0.5, 0.5))
        };
        
        let status_indicator = container(
            text(status_text)
                .size(14)
                .horizontal_alignment(iced::alignment::Horizontal::Center)
                .style(move |_theme| {
                    iced::widget::text::Appearance {
                        color: Some(status_color),
                    }
                })
        )
        .width(Length::Fill)
        .padding(8)
        .style(move |_theme| {
            iced::widget::container::Appearance {
                background: Some(Background::Color(Color::from_rgb(0.95, 0.95, 0.95))),
                border: Border {
                    color: status_color,
                    width: 2.0,
                    radius: 5.0.into(),
                },
                text_color: Some(status_color),
                ..Default::default()
            }
        });
        
        status_indicator.into()
    }
    
    /// Create complete device controls section
    /// 
    /// Combines device control buttons and status indicator into a complete
    /// device control section with proper layout and spacing.
    /// 
    /// # Arguments
    /// * `device` - Reference to device for status and operations
    /// * `is_connected` - Whether device is currently connected
    /// * `on_arm` - Callback for arm device action
    /// * `on_turn_off` - Callback for turn off device action
    /// * `on_shutdown` - Callback for shutdown and quit action
    /// 
    /// # Returns
    /// * `Element<Message>` - Complete device controls section
    /// 
    /// # Example
    /// ```
    /// let device_section = DeviceControls::create_device_section(
    ///     &device, true, Message::ArmDevice, Message::TurnOffDevice, Message::ShutdownDevice
    /// )?;
    /// ```
    pub fn create_device_section<Message>(
        device: Option<&LumidoxDevice>,
        is_connected: bool,
        on_arm: Message,
        on_turn_off: Message,
        on_shutdown: Message,
    ) -> Element<'static, Message>
    where
        Message: Clone + 'static,
    {
        let status_indicator = Self::create_status_indicator(device, is_connected);
        let control_buttons = Self::create_device_buttons(device, on_arm, on_turn_off, on_shutdown);
        
        let section = column![
            text("Device Controls")
                .size(18)
                .horizontal_alignment(iced::alignment::Horizontal::Center),
            Space::with_height(10),
            status_indicator,
            Space::with_height(15),
            control_buttons,
        ]
        .spacing(5)
        .align_items(Alignment::Center);
        
        container(section)
            .width(Length::Fill)
            .padding(15)
            .style(Self::section_style())
            .into()
    }
    
    /// ARM button styling
    /// 
    /// Provides styling for the ARM device button.
    /// 
    /// # Returns
    /// * Button style theme function
    fn arm_button_style() -> fn(&Theme, iced::widget::button::Status) -> iced::widget::button::Appearance {
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
    
    /// Turn off button styling
    /// 
    /// Provides styling for the turn off device button.
    /// 
    /// # Returns
    /// * Button style theme function
    fn turn_off_button_style() -> fn(&Theme, iced::widget::button::Status) -> iced::widget::button::Appearance {
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
    
    /// Shutdown button styling
    /// 
    /// Provides styling for the shutdown and quit button.
    /// 
    /// # Returns
    /// * Button style theme function
    fn shutdown_button_style() -> fn(&Theme, iced::widget::button::Status) -> iced::widget::button::Appearance {
        |_theme, status| {
            let base_color = Color::from_rgb(0.8, 0.2, 0.2);
            let (background, text_color) = match status {
                iced::widget::button::Status::Active => (base_color, Color::WHITE),
                iced::widget::button::Status::Hovered => (
                    Color::from_rgb(0.9, 0.3, 0.3), Color::WHITE
                ),
                iced::widget::button::Status::Pressed => (
                    Color::from_rgb(0.7, 0.1, 0.1), Color::WHITE
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
