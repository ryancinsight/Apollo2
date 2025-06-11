//! Status display components for Lumidox II Controller GUI
//!
//! This module provides device status display components including device state,
//! remote mode display, connection status, and real-time status updates.
//! It implements reusable UI components using the Iced framework for status
//! information display with proper error handling and visual feedback.
//!
//! The status display system provides:
//! - Real-time device state and remote mode display
//! - Connection status indicator with visual feedback
//! - Device mode transitions and status changes
//! - Error handling and status validation
//! - Consistent styling and layout for status information

use iced::{
    widget::{column, row, text, container, Space},
    Element, Length, Alignment, Color, Background, Border, Theme,
};
use crate::core::Result;
use crate::device::{LumidoxDevice, models::DeviceMode};

/// Status display components and functionality
pub struct StatusDisplay;

impl StatusDisplay {
    /// Create device status display
    /// 
    /// Creates a comprehensive device status display showing current device
    /// state, remote mode, and connection status with visual indicators.
    /// 
    /// # Arguments
    /// * `device` - Reference to device for status information
    /// * `is_connected` - Whether device is currently connected
    /// 
    /// # Returns
    /// * `Element<Message>` - Device status display element
    /// 
    /// # Example
    /// ```
    /// let status_display = StatusDisplay::create_device_status(&device, true)?;
    /// ```
    pub fn create_device_status<Message>(
        device: Option<&LumidoxDevice>,
        is_connected: bool,
    ) -> Element<'static, Message>
    where
        Message: Clone + 'static,
    {
        let connection_status = Self::create_connection_indicator(is_connected);
        let device_mode_status = Self::create_device_mode_display(device);
        let remote_mode_status = Self::create_remote_mode_display(device);
        
        let status_column = column![
            text("Device Status")
                .size(18)
                .horizontal_alignment(iced::alignment::Horizontal::Center),
            Space::with_height(10),
            connection_status,
            Space::with_height(10),
            device_mode_status,
            Space::with_height(10),
            remote_mode_status,
        ]
        .spacing(5)
        .align_items(Alignment::Center);
        
        container(status_column)
            .width(Length::Fill)
            .padding(15)
            .style(Self::section_style())
            .into()
    }
    
    /// Create connection status indicator
    /// 
    /// Creates a visual indicator showing device connection status with
    /// appropriate colors and text feedback.
    /// 
    /// # Arguments
    /// * `is_connected` - Whether device is currently connected
    /// 
    /// # Returns
    /// * `Element<Message>` - Connection status indicator
    /// 
    /// # Example
    /// ```
    /// let connection_indicator = StatusDisplay::create_connection_indicator(true);
    /// ```
    pub fn create_connection_indicator<Message>(
        is_connected: bool,
    ) -> Element<'static, Message>
    where
        Message: Clone + 'static,
    {
        let (status_text, status_color, background_color) = if is_connected {
            ("CONNECTED", Color::from_rgb(0.2, 0.8, 0.2), Color::from_rgb(0.9, 1.0, 0.9))
        } else {
            ("DISCONNECTED", Color::from_rgb(0.8, 0.2, 0.2), Color::from_rgb(1.0, 0.9, 0.9))
        };
        
        let connection_indicator = container(
            text(status_text)
                .size(16)
                .horizontal_alignment(iced::alignment::Horizontal::Center)
                .style(move |_theme| {
                    iced::widget::text::Appearance {
                        color: Some(status_color),
                    }
                })
        )
        .width(Length::Fill)
        .padding(10)
        .style(move |_theme| {
            iced::widget::container::Appearance {
                background: Some(Background::Color(background_color)),
                border: Border {
                    color: status_color,
                    width: 2.0,
                    radius: 8.0.into(),
                },
                text_color: Some(status_color),
                ..Default::default()
            }
        });
        
        connection_indicator.into()
    }
    
    /// Create device mode display
    /// 
    /// Creates a display showing the current device operational mode
    /// with appropriate visual styling and status information.
    /// 
    /// # Arguments
    /// * `device` - Reference to device for mode information
    /// 
    /// # Returns
    /// * `Element<Message>` - Device mode display element
    /// 
    /// # Example
    /// ```
    /// let mode_display = StatusDisplay::create_device_mode_display(&device);
    /// ```
    pub fn create_device_mode_display<Message>(
        device: Option<&LumidoxDevice>,
    ) -> Element<'static, Message>
    where
        Message: Clone + 'static,
    {
        let (mode_text, mode_color) = if let Some(dev) = device {
            match dev.read_remote_mode() {
                Ok(mode) => {
                    match mode {
                        DeviceMode::Local => ("Local Mode", Color::from_rgb(0.8, 0.6, 0.2)),
                        DeviceMode::Standby => ("Remote Standby", Color::from_rgb(0.2, 0.6, 0.8)),
                        DeviceMode::Armed => ("Armed", Color::from_rgb(0.8, 0.4, 0.2)),
                        DeviceMode::Remote => ("Remote Active", Color::from_rgb(0.2, 0.8, 0.2)),
                    }
                }
                Err(_) => ("Mode Unknown", Color::from_rgb(0.5, 0.5, 0.5))
            }
        } else {
            ("No Device", Color::from_rgb(0.5, 0.5, 0.5))
        };
        
        let mode_row = row![
            text("Device Mode:")
                .size(14)
                .horizontal_alignment(iced::alignment::Horizontal::Left),
            Space::with_width(10),
            text(mode_text)
                .size(14)
                .horizontal_alignment(iced::alignment::Horizontal::Left)
                .style(move |_theme| {
                    iced::widget::text::Appearance {
                        color: Some(mode_color),
                    }
                }),
        ]
        .align_items(Alignment::Center);
        
        container(mode_row)
            .width(Length::Fill)
            .padding(8)
            .style(Self::info_style())
            .into()
    }
    
    /// Create remote mode display
    /// 
    /// Creates a detailed display of remote mode status including
    /// additional mode-specific information and status details.
    /// 
    /// # Arguments
    /// * `device` - Reference to device for remote mode information
    /// 
    /// # Returns
    /// * `Element<Message>` - Remote mode display element
    /// 
    /// # Example
    /// ```
    /// let remote_display = StatusDisplay::create_remote_mode_display(&device);
    /// ```
    pub fn create_remote_mode_display<Message>(
        device: Option<&LumidoxDevice>,
    ) -> Element<'static, Message>
    where
        Message: Clone + 'static,
    {
        let (remote_text, remote_description, remote_color) = if let Some(dev) = device {
            match dev.read_remote_mode() {
                Ok(mode) => {
                    match mode {
                        DeviceMode::Local => (
                            "Local Control",
                            "Device controlled locally",
                            Color::from_rgb(0.8, 0.6, 0.2)
                        ),
                        DeviceMode::Standby => (
                            "Remote Standby",
                            "Ready for remote commands",
                            Color::from_rgb(0.2, 0.6, 0.8)
                        ),
                        DeviceMode::Armed => (
                            "Armed State",
                            "Device armed and ready to fire",
                            Color::from_rgb(0.8, 0.4, 0.2)
                        ),
                        DeviceMode::Remote => (
                            "Remote Active",
                            "Remote control active",
                            Color::from_rgb(0.2, 0.8, 0.2)
                        ),
                    }
                }
                Err(_) => (
                    "Status Error",
                    "Cannot read device status",
                    Color::from_rgb(0.8, 0.2, 0.2)
                )
            }
        } else {
            (
                "No Device",
                "Device not connected",
                Color::from_rgb(0.5, 0.5, 0.5)
            )
        };
        
        let remote_column = column![
            row![
                text("Remote Status:")
                    .size(14)
                    .horizontal_alignment(iced::alignment::Horizontal::Left),
                Space::with_width(10),
                text(remote_text)
                    .size(14)
                    .horizontal_alignment(iced::alignment::Horizontal::Left)
                    .style(move |_theme| {
                        iced::widget::text::Appearance {
                            color: Some(remote_color),
                        }
                    }),
            ]
            .align_items(Alignment::Center),
            Space::with_height(5),
            text(remote_description)
                .size(12)
                .horizontal_alignment(iced::alignment::Horizontal::Left)
                .style(move |_theme| {
                    iced::widget::text::Appearance {
                        color: Some(Color::from_rgb(0.6, 0.6, 0.6)),
                    }
                }),
        ]
        .spacing(2)
        .align_items(Alignment::Start);
        
        container(remote_column)
            .width(Length::Fill)
            .padding(8)
            .style(Self::info_style())
            .into()
    }
    
    /// Create status summary display
    /// 
    /// Creates a compact status summary showing key device information
    /// in a condensed format for space-constrained layouts.
    /// 
    /// # Arguments
    /// * `device` - Reference to device for status information
    /// * `is_connected` - Whether device is currently connected
    /// 
    /// # Returns
    /// * `Element<Message>` - Status summary display element
    /// 
    /// # Example
    /// ```
    /// let status_summary = StatusDisplay::create_status_summary(&device, true);
    /// ```
    pub fn create_status_summary<Message>(
        device: Option<&LumidoxDevice>,
        is_connected: bool,
    ) -> Element<'static, Message>
    where
        Message: Clone + 'static,
    {
        let status_text = if !is_connected {
            "DISCONNECTED".to_string()
        } else if let Some(dev) = device {
            match dev.read_remote_mode() {
                Ok(mode) => {
                    match mode {
                        DeviceMode::Local => "LOCAL".to_string(),
                        DeviceMode::Standby => "STANDBY".to_string(),
                        DeviceMode::Armed => "ARMED".to_string(),
                        DeviceMode::Remote => "REMOTE".to_string(),
                    }
                }
                Err(_) => "ERROR".to_string()
            }
        } else {
            "NO DEVICE".to_string()
        };
        
        let status_color = if !is_connected {
            Color::from_rgb(0.8, 0.2, 0.2)
        } else if let Some(dev) = device {
            match dev.read_remote_mode() {
                Ok(mode) => {
                    match mode {
                        DeviceMode::Local => Color::from_rgb(0.8, 0.6, 0.2),
                        DeviceMode::Standby => Color::from_rgb(0.2, 0.6, 0.8),
                        DeviceMode::Armed => Color::from_rgb(0.8, 0.4, 0.2),
                        DeviceMode::Remote => Color::from_rgb(0.2, 0.8, 0.2),
                    }
                }
                Err(_) => Color::from_rgb(0.8, 0.2, 0.2)
            }
        } else {
            Color::from_rgb(0.5, 0.5, 0.5)
        };
        
        let summary_display = container(
            text(status_text)
                .size(12)
                .horizontal_alignment(iced::alignment::Horizontal::Center)
                .style(move |_theme| {
                    iced::widget::text::Appearance {
                        color: Some(status_color),
                    }
                })
        )
        .width(Length::Fill)
        .padding(5)
        .style(move |_theme| {
            iced::widget::container::Appearance {
                background: Some(Background::Color(Color::from_rgb(0.95, 0.95, 0.95))),
                border: Border {
                    color: status_color,
                    width: 1.0,
                    radius: 3.0.into(),
                },
                text_color: Some(status_color),
                ..Default::default()
            }
        });
        
        summary_display.into()
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
    
    /// Info container styling
    /// 
    /// Provides styling for information display containers.
    /// 
    /// # Returns
    /// * Container style theme function
    fn info_style() -> fn(&Theme) -> iced::widget::container::Appearance {
        |_theme| {
            iced::widget::container::Appearance {
                background: Some(Background::Color(Color::from_rgb(0.98, 0.98, 0.98))),
                border: Border {
                    color: Color::from_rgb(0.9, 0.9, 0.9),
                    width: 1.0,
                    radius: 5.0.into(),
                },
                text_color: Some(Color::BLACK),
                ..Default::default()
            }
        }
    }
}
