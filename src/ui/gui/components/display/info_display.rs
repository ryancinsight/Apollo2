//! Information display components for Lumidox II Controller GUI
//!
//! This module provides device information display components including device
//! parameters, stage information, power details, and comprehensive device status.
//! It implements reusable UI components using the Iced framework for information
//! display with proper error handling and data formatting.
//!
//! The info display system provides:
//! - Device parameter information display matching CLI info commands
//! - Stage-specific parameter and power information
//! - Complete device status and configuration display
//! - ARM current settings and synchronization status
//! - Consistent formatting and layout for information presentation

use iced::{
    widget::{column, row, text, container, Space, scrollable},
    Element, Length, Alignment, Color, Background, Border, Theme,
};
use crate::core::Result;
use crate::core::calculations::irradiance::IrradianceCalculator;
use crate::device::LumidoxDevice;

/// Information display components and functionality
pub struct InfoDisplay;

impl InfoDisplay {
    /// Create device information display
    /// 
    /// Creates a comprehensive device information display showing device
    /// parameters, stage information, and current settings matching CLI info output.
    /// 
    /// # Arguments
    /// * `device` - Reference to device for information queries
    /// 
    /// # Returns
    /// * `Element<Message>` - Device information display element
    /// 
    /// # Example
    /// ```
    /// let info_display = InfoDisplay::create_device_info(&device)?;
    /// ```
    pub fn create_device_info<Message>(
        device: Option<&LumidoxDevice>,
    ) -> Element<'static, Message>
    where
        Message: Clone + 'static,
    {
        let device_params = Self::create_device_parameters_display(device);
        let current_settings = Self::create_current_settings_display(device);
        let stage_info = Self::create_stage_information_display(device);
        
        let info_column = column![
            text("Device Information")
                .size(18)
                .horizontal_alignment(iced::alignment::Horizontal::Center),
            Space::with_height(15),
            device_params,
            Space::with_height(15),
            current_settings,
            Space::with_height(15),
            stage_info,
        ]
        .spacing(10)
        .align_items(Alignment::Center);
        
        let scrollable_content = scrollable(info_column)
            .width(Length::Fill)
            .height(Length::Fill);
        
        container(scrollable_content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(15)
            .style(Self::section_style())
            .into()
    }
    
    /// Create device parameters display
    /// 
    /// Creates a display showing device parameters including voltage settings,
    /// ARM current, and device configuration matching CLI parameter output.
    /// 
    /// # Arguments
    /// * `device` - Reference to device for parameter queries
    /// 
    /// # Returns
    /// * `Element<Message>` - Device parameters display element
    pub fn create_device_parameters_display<Message>(
        device: Option<&LumidoxDevice>,
    ) -> Element<'static, Message>
    where
        Message: Clone + 'static,
    {
        let params_content = if let Some(dev) = device {
            match dev.get_complete_parameters() {
                Ok(params) => {
                    column![
                        Self::create_parameter_row("ARM Current:", &format!("{}mA", params.arm_current)),
                        Self::create_parameter_row("Voltage 1:", &format!("{}V", params.voltage_1)),
                        Self::create_parameter_row("Voltage 2:", &format!("{}V", params.voltage_2)),
                        Self::create_parameter_row("Voltage 3:", &format!("{}V", params.voltage_3)),
                        Self::create_parameter_row("Voltage 4:", &format!("{}V", params.voltage_4)),
                        Self::create_parameter_row("Voltage 5:", &format!("{}V", params.voltage_5)),
                    ]
                    .spacing(8)
                }
                Err(e) => {
                    column![
                        text("Parameter Error")
                            .size(14)
                            .style(|_theme| iced::widget::text::Appearance {
                                color: Some(Color::from_rgb(0.8, 0.2, 0.2)),
                            }),
                        text(format!("Failed to read parameters: {}", e))
                            .size(12)
                            .style(|_theme| iced::widget::text::Appearance {
                                color: Some(Color::from_rgb(0.6, 0.6, 0.6)),
                            }),
                    ]
                    .spacing(5)
                }
            }
        } else {
            column![
                text("No Device Connected")
                    .size(14)
                    .style(|_theme| iced::widget::text::Appearance {
                        color: Some(Color::from_rgb(0.5, 0.5, 0.5)),
                    }),
            ]
            .spacing(5)
        };
        
        let params_section = column![
            text("Device Parameters")
                .size(16)
                .horizontal_alignment(iced::alignment::Horizontal::Center),
            Space::with_height(10),
            params_content,
        ]
        .spacing(5)
        .align_items(Alignment::Center);
        
        container(params_section)
            .width(Length::Fill)
            .padding(12)
            .style(Self::info_style())
            .into()
    }
    
    /// Create current settings display
    /// 
    /// Creates a display showing current ARM and fire current settings
    /// with synchronization status matching CLI current display.
    /// 
    /// # Arguments
    /// * `device` - Reference to device for current information
    /// 
    /// # Returns
    /// * `Element<Message>` - Current settings display element
    pub fn create_current_settings_display<Message>(
        device: Option<&LumidoxDevice>,
    ) -> Element<'static, Message>
    where
        Message: Clone + 'static,
    {
        let current_content = if let Some(dev) = device {
            let arm_current = dev.read_arm_current()
                .map(|c| format!("{}mA", c))
                .unwrap_or_else(|_| "Error".to_string());
            
            let fire_current = dev.read_fire_current()
                .map(|c| format!("{}mA", c))
                .unwrap_or_else(|_| "Error".to_string());
            
            let sync_status = match dev.are_currents_synchronized() {
                Ok(true) => ("Synchronized", Color::from_rgb(0.2, 0.8, 0.2)),
                Ok(false) => ("Not Synchronized", Color::from_rgb(0.8, 0.6, 0.2)),
                Err(_) => ("Unknown", Color::from_rgb(0.8, 0.2, 0.2)),
            };
            
            column![
                Self::create_parameter_row("ARM Current:", &arm_current),
                Self::create_parameter_row("Fire Current:", &fire_current),
                row![
                    text("Sync Status:")
                        .size(14)
                        .horizontal_alignment(iced::alignment::Horizontal::Left),
                    Space::with_width(10),
                    text(sync_status.0)
                        .size(14)
                        .style(move |_theme| iced::widget::text::Appearance {
                            color: Some(sync_status.1),
                        }),
                ]
                .align_items(Alignment::Center),
            ]
            .spacing(8)
        } else {
            column![
                text("No Device Connected")
                    .size(14)
                    .style(|_theme| iced::widget::text::Appearance {
                        color: Some(Color::from_rgb(0.5, 0.5, 0.5)),
                    }),
            ]
            .spacing(5)
        };
        
        let current_section = column![
            text("Current Settings")
                .size(16)
                .horizontal_alignment(iced::alignment::Horizontal::Center),
            Space::with_height(10),
            current_content,
        ]
        .spacing(5)
        .align_items(Alignment::Center);
        
        container(current_section)
            .width(Length::Fill)
            .padding(12)
            .style(Self::info_style())
            .into()
    }
    
    /// Create stage information display
    /// 
    /// Creates a display showing stage-specific information including
    /// power details and irradiance calculations for each stage matching CLI stage info output.
    /// 
    /// # Arguments
    /// * `device` - Reference to device for stage information
    /// 
    /// # Returns
    /// * `Element<Message>` - Stage information display element
    pub fn create_stage_information_display<Message>(
        device: Option<&LumidoxDevice>,
    ) -> Element<'static, Message>
    where
        Message: Clone + 'static,
    {
        let stage_content = if let Some(dev) = device {
            let mut stage_rows = Vec::new();
            
            for stage in 1..=5 {
                match dev.get_power_info(stage) {
                    Ok(power_info) => {
                        // Format power information
                        let power_text = format!("{} {} ({} {})", 
                            power_info.total_power, power_info.total_units, 
                            power_info.per_power, power_info.per_units);
                        
                        // Calculate irradiance
                        let irradiance_text = match IrradianceCalculator::calculate_irradiance(&power_info) {
                            Ok(irradiance_data) => {
                                format!("{:.3} mW/cm²", irradiance_data.surface_irradiance_mw_cm2)
                            }
                            Err(_) => "Error calculating irradiance".to_string(),
                        };
                        
                        // Create stage info with both power and irradiance
                        let stage_info = column![
                            row![
                                text(&format!("Stage {}:", stage))
                                    .size(14)
                                    .horizontal_alignment(iced::alignment::Horizontal::Left),
                                Space::with_width(10),
                                text(&power_text)
                                    .size(14)
                                    .style(|_theme| iced::widget::text::Appearance {
                                        color: Some(Color::from_rgb(0.3, 0.6, 0.9)),
                                    }),
                            ]
                            .align_items(Alignment::Center),
                            row![
                                Space::with_width(70), // Indent for alignment
                                text("Irradiance:")
                                    .size(12)
                                    .style(|_theme| iced::widget::text::Appearance {
                                        color: Some(Color::from_rgb(0.6, 0.6, 0.6)),
                                    }),
                                Space::with_width(10),
                                text(&irradiance_text)
                                    .size(12)
                                    .style(|_theme| iced::widget::text::Appearance {
                                        color: Some(Color::from_rgb(0.2, 0.8, 0.4)),
                                    }),
                            ]
                            .align_items(Alignment::Center),
                        ]
                        .spacing(3);
                        
                        stage_rows.push(stage_info);
                    }
                    Err(_) => {
                        let error_info = Self::create_parameter_row(
                            &format!("Stage {}:", stage), 
                            "Error reading stage data"
                        );
                        stage_rows.push(error_info);
                    }
                }
            }
            
            column(stage_rows).spacing(8)
        } else {
            column![
                text("No Device Connected")
                    .size(14)
                    .style(|_theme| iced::widget::text::Appearance {
                        color: Some(Color::from_rgb(0.5, 0.5, 0.5)),
                    }),
            ]
            .spacing(5)
        };
        
        let stage_section = column![
            text("Stage Information")
                .size(16)
                .horizontal_alignment(iced::alignment::Horizontal::Center),
            Space::with_height(10),
            stage_content,
        ]
        .spacing(5)
        .align_items(Alignment::Center);
        
        container(stage_section)
            .width(Length::Fill)
            .padding(12)
            .style(Self::info_style())
            .into()
    }
    
    /// Create parameter row
    /// 
    /// Creates a formatted parameter row with label and value.
    /// 
    /// # Arguments
    /// * `label` - Parameter label
    /// * `value` - Parameter value
    /// 
    /// # Returns
    /// * `Element<Message>` - Parameter row element
    fn create_parameter_row<Message>(
        label: &str,
        value: &str,
    ) -> Element<'static, Message>
    where
        Message: Clone + 'static,
    {
        row![
            text(label)
                .size(14)
                .horizontal_alignment(iced::alignment::Horizontal::Left)
                .width(Length::FillPortion(2)),
            text(value)
                .size(14)
                .horizontal_alignment(iced::alignment::Horizontal::Left)
                .width(Length::FillPortion(3)),
        ]
        .align_items(Alignment::Center)
        .spacing(10)
        .into()
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
