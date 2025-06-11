//! Control panel layout components for Lumidox II Controller GUI
//!
//! This module provides control panel layout organization matching the CLI
//! menu structure with sections for Device Control, Stage Control, Parameter
//! Management, and System Operations. It implements responsive layout design
//! using the Iced framework with proper spacing and organization.
//!
//! The control panel layout system provides:
//! - Organized control sections matching CLI menu structure
//! - Responsive layout with proper spacing and alignment
//! - Consistent styling and visual hierarchy
//! - Integration with existing control components
//! - Proper container management and element positioning

use iced::{
    widget::{column, row, container, Space, scrollable},
    Element, Length, Alignment,
};
use crate::device::LumidoxDevice;

/// Control panel layout coordinator
/// 
/// Provides layout organization for control panels matching the CLI menu
/// structure with proper sections and responsive design.
pub struct ControlPanelLayout;

impl ControlPanelLayout {
    /// Create organized control panel layout
    /// 
    /// Creates a comprehensive control panel layout with sections for stage
    /// controls, device controls, and current controls, organized vertically
    /// with proper spacing and visual separation.
    /// 
    /// # Arguments
    /// * `stage_controls` - Stage firing controls element
    /// * `device_controls` - Device control buttons element
    /// * `current_controls` - ARM current setting controls element
    /// 
    /// # Returns
    /// * `Element<Message>` - Complete organized control panel
    /// 
    /// # Example
    /// ```
    /// let control_panel = ControlPanelLayout::create_control_panel(
    ///     stage_controls,
    ///     device_controls,
    ///     current_controls
    /// );
    /// ```
    pub fn create_control_panel<Message>(
        stage_controls: Element<'static, Message>,
        device_controls: Element<'static, Message>,
        current_controls: Element<'static, Message>,
    ) -> Element<'static, Message>
    where
        Message: Clone + 'static,
    {
        let control_sections = column![
            // Stage Controls Section
            container(stage_controls)
                .width(Length::Fill)
                .padding(10)
                .style(Self::section_container_style()),
            
            Space::with_height(15),
            
            // Device Controls Section
            container(device_controls)
                .width(Length::Fill)
                .padding(10)
                .style(Self::section_container_style()),
            
            Space::with_height(15),
            
            // Current Controls Section
            container(current_controls)
                .width(Length::Fill)
                .padding(10)
                .style(Self::section_container_style()),
        ]
        .spacing(0)
        .align_items(Alignment::Fill);
        
        // Make the control panel scrollable for smaller screens
        let scrollable_controls = scrollable(control_sections)
            .width(Length::Fill)
            .height(Length::Fill);
        
        container(scrollable_controls)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(5)
            .style(Self::main_panel_style())
            .into()
    }
    
    /// Create compact control panel layout
    /// 
    /// Creates a more compact control panel layout suitable for smaller
    /// screen sizes or when space is limited.
    /// 
    /// # Arguments
    /// * `stage_controls` - Stage firing controls element
    /// * `device_controls` - Device control buttons element
    /// * `current_controls` - ARM current setting controls element
    /// 
    /// # Returns
    /// * `Element<Message>` - Compact control panel layout
    /// 
    /// # Example
    /// ```
    /// let compact_panel = ControlPanelLayout::create_compact_panel(
    ///     stage_controls,
    ///     device_controls,
    ///     current_controls
    /// );
    /// ```
    pub fn create_compact_panel<Message>(
        stage_controls: Element<'static, Message>,
        device_controls: Element<'static, Message>,
        current_controls: Element<'static, Message>,
    ) -> Element<'static, Message>
    where
        Message: Clone + 'static,
    {
        let control_sections = column![
            // Compact Stage Controls
            container(stage_controls)
                .width(Length::Fill)
                .padding(5)
                .style(Self::compact_section_style()),
            
            Space::with_height(8),
            
            // Compact Device Controls
            container(device_controls)
                .width(Length::Fill)
                .padding(5)
                .style(Self::compact_section_style()),
            
            Space::with_height(8),
            
            // Compact Current Controls
            container(current_controls)
                .width(Length::Fill)
                .padding(5)
                .style(Self::compact_section_style()),
        ]
        .spacing(0)
        .align_items(Alignment::Fill);
        
        let scrollable_controls = scrollable(control_sections)
            .width(Length::Fill)
            .height(Length::Fill);
        
        container(scrollable_controls)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(3)
            .style(Self::compact_panel_style())
            .into()
    }
    
    /// Create horizontal control panel layout
    /// 
    /// Creates a horizontal layout for control panels suitable for wide
    /// screen configurations where horizontal space is abundant.
    /// 
    /// # Arguments
    /// * `stage_controls` - Stage firing controls element
    /// * `device_controls` - Device control buttons element
    /// * `current_controls` - ARM current setting controls element
    /// 
    /// # Returns
    /// * `Element<Message>` - Horizontal control panel layout
    /// 
    /// # Example
    /// ```
    /// let horizontal_panel = ControlPanelLayout::create_horizontal_panel(
    ///     stage_controls,
    ///     device_controls,
    ///     current_controls
    /// );
    /// ```
    pub fn create_horizontal_panel<Message>(
        stage_controls: Element<'static, Message>,
        device_controls: Element<'static, Message>,
        current_controls: Element<'static, Message>,
    ) -> Element<'static, Message>
    where
        Message: Clone + 'static,
    {
        let control_row = row![
            // Stage Controls Column
            container(stage_controls)
                .width(Length::FillPortion(2))
                .padding(8)
                .style(Self::section_container_style()),
            
            Space::with_width(10),
            
            // Device and Current Controls Column
            column![
                container(device_controls)
                    .width(Length::Fill)
                    .padding(8)
                    .style(Self::section_container_style()),
                
                Space::with_height(10),
                
                container(current_controls)
                    .width(Length::Fill)
                    .padding(8)
                    .style(Self::section_container_style()),
            ]
            .width(Length::FillPortion(1))
            .spacing(0),
        ]
        .align_items(Alignment::Start)
        .spacing(0);
        
        container(control_row)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(10)
            .style(Self::main_panel_style())
            .into()
    }
    
    /// Create control panel with status bar
    /// 
    /// Creates a control panel layout that includes a status bar at the
    /// bottom for displaying connection status and quick device information.
    /// 
    /// # Arguments
    /// * `stage_controls` - Stage firing controls element
    /// * `device_controls` - Device control buttons element
    /// * `current_controls` - ARM current setting controls element
    /// * `status_bar` - Status bar element for bottom display
    /// 
    /// # Returns
    /// * `Element<Message>` - Control panel with status bar
    /// 
    /// # Example
    /// ```
    /// let panel_with_status = ControlPanelLayout::create_panel_with_status(
    ///     stage_controls, device_controls, current_controls, status_bar
    /// );
    /// ```
    pub fn create_panel_with_status<Message>(
        stage_controls: Element<'static, Message>,
        device_controls: Element<'static, Message>,
        current_controls: Element<'static, Message>,
        status_bar: Element<'static, Message>,
    ) -> Element<'static, Message>
    where
        Message: Clone + 'static,
    {
        let main_controls = Self::create_control_panel(
            stage_controls,
            device_controls,
            current_controls,
        );
        
        column![
            // Main control panel
            container(main_controls)
                .width(Length::Fill)
                .height(Length::FillPortion(10)),
            
            Space::with_height(5),
            
            // Status bar at bottom
            container(status_bar)
                .width(Length::Fill)
                .height(Length::Shrink)
                .padding(5)
                .style(Self::status_bar_style()),
        ]
        .spacing(0)
        .into()
    }
    
    /// Main panel styling
    fn main_panel_style() -> fn(&iced::Theme) -> iced::widget::container::Appearance {
        |_theme| {
            use iced::{Background, Color, Border};
            iced::widget::container::Appearance {
                background: Some(Background::Color(Color::from_rgb(0.96, 0.96, 0.96))),
                border: Border {
                    color: Color::from_rgb(0.75, 0.75, 0.75),
                    width: 1.0,
                    radius: 10.0.into(),
                },
                text_color: Some(Color::BLACK),
                ..Default::default()
            }
        }
    }
    
    /// Section container styling
    fn section_container_style() -> fn(&iced::Theme) -> iced::widget::container::Appearance {
        |_theme| {
            use iced::{Background, Color, Border};
            iced::widget::container::Appearance {
                background: Some(Background::Color(Color::from_rgb(0.99, 0.99, 0.99))),
                border: Border {
                    color: Color::from_rgb(0.8, 0.8, 0.8),
                    width: 1.0,
                    radius: 6.0.into(),
                },
                text_color: Some(Color::BLACK),
                ..Default::default()
            }
        }
    }
    
    /// Compact section styling
    fn compact_section_style() -> fn(&iced::Theme) -> iced::widget::container::Appearance {
        |_theme| {
            use iced::{Background, Color, Border};
            iced::widget::container::Appearance {
                background: Some(Background::Color(Color::from_rgb(0.98, 0.98, 0.98))),
                border: Border {
                    color: Color::from_rgb(0.85, 0.85, 0.85),
                    width: 0.5,
                    radius: 4.0.into(),
                },
                text_color: Some(Color::BLACK),
                ..Default::default()
            }
        }
    }
    
    /// Compact panel styling
    fn compact_panel_style() -> fn(&iced::Theme) -> iced::widget::container::Appearance {
        |_theme| {
            use iced::{Background, Color, Border};
            iced::widget::container::Appearance {
                background: Some(Background::Color(Color::from_rgb(0.97, 0.97, 0.97))),
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
    
    /// Status bar styling
    fn status_bar_style() -> fn(&iced::Theme) -> iced::widget::container::Appearance {
        |_theme| {
            use iced::{Background, Color, Border};
            iced::widget::container::Appearance {
                background: Some(Background::Color(Color::from_rgb(0.92, 0.92, 0.92))),
                border: Border {
                    color: Color::from_rgb(0.7, 0.7, 0.7),
                    width: 1.0,
                    radius: 5.0.into(),
                },
                text_color: Some(Color::BLACK),
                ..Default::default()
            }
        }
    }
}
