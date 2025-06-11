//! Main layout components for Lumidox II Controller GUI
//!
//! This module provides the main application layout structure with responsive
//! design for controls and display panels. It implements the primary layout
//! organization using the Iced framework with proper container and column
//! layouts for optimal user experience.
//!
//! The main layout system provides:
//! - Responsive main application layout with proper spacing
//! - Control panel and information panel organization
//! - Consistent styling and layout patterns across the application
//! - Proper container management and element positioning
//! - Integration with existing control and display components

use iced::{
    widget::{column, row, container, Space},
    Element, Length, Alignment,
};

/// Main layout components coordinator
/// 
/// Provides the primary application layout structure with responsive design
/// for organizing control panels and information displays.
pub struct MainLayout;

impl MainLayout {
    /// Create main application layout
    /// 
    /// Creates the primary application layout with control panel on the left
    /// and information panel on the right, using responsive design principles.
    /// 
    /// # Arguments
    /// * `controls_panel` - Control panel element containing all device controls
    /// * `info_panel` - Information panel element containing status and device info
    /// 
    /// # Returns
    /// * `Element<Message>` - Complete main application layout
    /// 
    /// # Example
    /// ```
    /// let main_layout = MainLayout::create_main_layout(
    ///     controls_panel,
    ///     info_panel
    /// );
    /// ```
    pub fn create_main_layout<Message>(
        controls_panel: Element<'static, Message>,
        info_panel: Element<'static, Message>,
    ) -> Element<'static, Message>
    where
        Message: Clone + 'static,
    {
        let main_row = row![
            // Left side: Controls panel
            container(controls_panel)
                .width(Length::FillPortion(2))
                .padding(10)
                .style(Self::controls_container_style()),
            
            // Spacing between panels
            Space::with_width(15),
            
            // Right side: Information panel
            container(info_panel)
                .width(Length::FillPortion(3))
                .padding(10)
                .style(Self::info_container_style()),
        ]
        .align_items(Alignment::Start)
        .spacing(0);
        
        container(main_row)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .style(Self::main_container_style())
            .into()
    }
    
    /// Create application header
    /// 
    /// Creates the application header with title and connection status indicator.
    /// 
    /// # Arguments
    /// * `is_connected` - Whether device is currently connected
    /// * `device_info` - Optional device information for display
    /// 
    /// # Returns
    /// * `Element<Message>` - Application header element
    /// 
    /// # Example
    /// ```
    /// let header = MainLayout::create_app_header(true, Some("Lumidox II"));
    /// ```
    pub fn create_app_header<Message>(
        is_connected: bool,
        device_info: Option<&str>,
    ) -> Element<'static, Message>
    where
        Message: Clone + 'static,
    {
        use iced::widget::text;
        use iced::{Color, Background, Border};
        
        let title = text("Lumidox II Controller")
            .size(24)
            .horizontal_alignment(iced::alignment::Horizontal::Center);
        
        let connection_text = if is_connected {
            if let Some(info) = device_info {
                format!("Connected: {}", info)
            } else {
                "Connected".to_string()
            }
        } else {
            "Disconnected".to_string()
        };
        
        let connection_color = if is_connected {
            Color::from_rgb(0.2, 0.8, 0.2)
        } else {
            Color::from_rgb(0.8, 0.2, 0.2)
        };
        
        let connection_status = text(connection_text)
            .size(14)
            .horizontal_alignment(iced::alignment::Horizontal::Center)
            .style(move |_theme| {
                iced::widget::text::Appearance {
                    color: Some(connection_color),
                }
            });
        
        let header_content = column![
            title,
            Space::with_height(5),
            connection_status,
        ]
        .align_items(Alignment::Center)
        .spacing(0);
        
        container(header_content)
            .width(Length::Fill)
            .padding(15)
            .style(Self::header_container_style())
            .into()
    }
    
    /// Create complete application layout with header
    /// 
    /// Creates the complete application layout including header, controls,
    /// and information panels with proper spacing and organization.
    /// 
    /// # Arguments
    /// * `controls_panel` - Control panel element
    /// * `info_panel` - Information panel element
    /// * `is_connected` - Whether device is currently connected
    /// * `device_info` - Optional device information for header
    /// 
    /// # Returns
    /// * `Element<Message>` - Complete application layout with header
    /// 
    /// # Example
    /// ```
    /// let complete_layout = MainLayout::create_complete_layout(
    ///     controls_panel, info_panel, true, Some("Lumidox II")
    /// );
    /// ```
    pub fn create_complete_layout<Message>(
        controls_panel: Element<'static, Message>,
        info_panel: Element<'static, Message>,
        is_connected: bool,
        device_info: Option<&str>,
    ) -> Element<'static, Message>
    where
        Message: Clone + 'static,
    {
        let header = Self::create_app_header(is_connected, device_info);
        let main_content = Self::create_main_layout(controls_panel, info_panel);
        
        column![
            header,
            Space::with_height(10),
            main_content,
        ]
        .spacing(0)
        .into()
    }
    
    /// Main container styling
    fn main_container_style() -> fn(&iced::Theme) -> iced::widget::container::Appearance {
        |_theme| {
            use iced::{Background, Color, Border};
            iced::widget::container::Appearance {
                background: Some(Background::Color(Color::from_rgb(0.95, 0.95, 0.95))),
                border: Border {
                    color: Color::from_rgb(0.8, 0.8, 0.8),
                    width: 1.0,
                    radius: 5.0.into(),
                },
                text_color: Some(Color::BLACK),
                ..Default::default()
            }
        }
    }
    
    /// Controls container styling
    fn controls_container_style() -> fn(&iced::Theme) -> iced::widget::container::Appearance {
        |_theme| {
            use iced::{Background, Color, Border};
            iced::widget::container::Appearance {
                background: Some(Background::Color(Color::from_rgb(0.98, 0.98, 0.98))),
                border: Border {
                    color: Color::from_rgb(0.7, 0.7, 0.7),
                    width: 1.0,
                    radius: 8.0.into(),
                },
                text_color: Some(Color::BLACK),
                ..Default::default()
            }
        }
    }
    
    /// Information container styling
    fn info_container_style() -> fn(&iced::Theme) -> iced::widget::container::Appearance {
        |_theme| {
            use iced::{Background, Color, Border};
            iced::widget::container::Appearance {
                background: Some(Background::Color(Color::from_rgb(0.97, 0.97, 0.97))),
                border: Border {
                    color: Color::from_rgb(0.7, 0.7, 0.7),
                    width: 1.0,
                    radius: 8.0.into(),
                },
                text_color: Some(Color::BLACK),
                ..Default::default()
            }
        }
    }
    
    /// Header container styling
    fn header_container_style() -> fn(&iced::Theme) -> iced::widget::container::Appearance {
        |_theme| {
            use iced::{Background, Color, Border};
            iced::widget::container::Appearance {
                background: Some(Background::Color(Color::from_rgb(0.9, 0.9, 0.9))),
                border: Border {
                    color: Color::from_rgb(0.6, 0.6, 0.6),
                    width: 2.0,
                    radius: 10.0.into(),
                },
                text_color: Some(Color::BLACK),
                ..Default::default()
            }
        }
    }
}
