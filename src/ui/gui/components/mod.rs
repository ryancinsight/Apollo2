//! UI components coordination module for Lumidox II Controller GUI
//!
//! This module organizes and exports all UI components for the Lumidox II
//! Controller GUI application. It provides a centralized access point for
//! control components, layout components, and component factory functions
//! with proper module organization and re-exports.
//!
//! The components module includes:
//! - Re-exports of all existing control components
//! - Re-exports of all layout components and utilities
//! - Component factory functions for standardized component creation
//! - Proper visibility controls and module organization
//! - Integration with unified state management and message system

// Import component modules
pub mod controls;
pub mod layout;

// Re-export control components for easy access
pub use controls::current_controls::CurrentControls;

// Re-export layout components for easy access
pub use layout::{
    MainLayout, ControlPanelLayout, LayoutComponents
};

use iced::{widget::{column, row, container, text, button, Space}, Element, Length, Alignment};
use crate::ui::gui::application::messages::{Message, MessageFactory};
use crate::ui::gui::application::state::{UnifiedState, AppView, NotificationType};
use crate::device::LumidoxDevice;

/// Component factory for creating standardized UI components
/// 
/// Provides factory functions for creating consistent UI components
/// with proper styling, message handling, and state integration.
pub struct ComponentFactory;

impl ComponentFactory {
    /// Create stage firing controls
    /// 
    /// Creates a complete stage firing control panel with buttons for
    /// stages 1-5 and custom current firing functionality.
    /// 
    /// # Arguments
    /// * `state` - Current unified application state
    /// 
    /// # Returns
    /// * `Element<Message>` - Stage firing controls element
    /// 
    /// # Example
    /// ```
    /// let stage_controls = ComponentFactory::create_stage_controls(&state);
    /// ```
    pub fn create_stage_controls(state: &UnifiedState) -> Element<'static, Message> {
        let title = text("Stage Controls")
            .size(18)
            .horizontal_alignment(iced::alignment::Horizontal::Center);
        
        // Create stage buttons (1-5)
        let stage_buttons = row![
            button("Stage 1")
                .on_press(MessageFactory::fire_stage(1))
                .style(Self::stage_button_style()),
            Space::with_width(5),
            button("Stage 2")
                .on_press(MessageFactory::fire_stage(2))
                .style(Self::stage_button_style()),
            Space::with_width(5),
            button("Stage 3")
                .on_press(MessageFactory::fire_stage(3))
                .style(Self::stage_button_style()),
            Space::with_width(5),
            button("Stage 4")
                .on_press(MessageFactory::fire_stage(4))
                .style(Self::stage_button_style()),
            Space::with_width(5),
            button("Stage 5")
                .on_press(MessageFactory::fire_stage(5))
                .style(Self::stage_button_style()),
        ]
        .spacing(0)
        .align_items(Alignment::Center);
        
        // Custom current firing section
        let custom_section = column![
            text("Custom Current Firing")
                .size(14)
                .horizontal_alignment(iced::alignment::Horizontal::Center),
            Space::with_height(5),
            text("Enter current value and use ARM current controls below")
                .size(12)
                .horizontal_alignment(iced::alignment::Horizontal::Center),
        ]
        .spacing(0)
        .align_items(Alignment::Center);
        
        let content = column![
            title,
            Space::with_height(10),
            stage_buttons,
            Space::with_height(15),
            custom_section,
        ]
        .spacing(0)
        .align_items(Alignment::Center);
        
        container(content)
            .width(Length::Fill)
            .padding(10)
            .style(Self::section_container_style())
            .into()
    }
    
    /// Create device control buttons
    /// 
    /// Creates device control buttons for connection, disconnection,
    /// arming, and shutdown operations.
    /// 
    /// # Arguments
    /// * `state` - Current unified application state
    /// 
    /// # Returns
    /// * `Element<Message>` - Device control buttons element
    /// 
    /// # Example
    /// ```
    /// let device_controls = ComponentFactory::create_device_controls(&state);
    /// ```
    pub fn create_device_controls(state: &UnifiedState) -> Element<'static, Message> {
        let title = text("Device Controls")
            .size(18)
            .horizontal_alignment(iced::alignment::Horizontal::Center);
        
        // Connection controls
        let connection_controls = if state.is_device_connected() {
            row![
                button("Disconnect")
                    .on_press(MessageFactory::disconnect_device())
                    .style(Self::warning_button_style()),
                Space::with_width(10),
                button("Arm Device")
                    .on_press(Message::Device(crate::ui::gui::application::messages::DeviceMessage::ArmDevice))
                    .style(Self::primary_button_style()),
            ]
            .spacing(0)
            .align_items(Alignment::Center)
        } else {
            row![
                button("Connect")
                    .on_press(MessageFactory::connect_device(
                        state.device_state.port_name.clone(),
                        state.device_state.auto_detect,
                        state.device_state.verbose,
                        state.device_state.optimize_transitions,
                    ))
                    .style(Self::primary_button_style()),
            ]
            .spacing(0)
            .align_items(Alignment::Center)
        };
        
        // Device operation controls (only show when connected)
        let operation_controls = if state.is_device_connected() {
            column![
                Space::with_height(10),
                row![
                    button("Turn Off")
                        .on_press(Message::Device(crate::ui::gui::application::messages::DeviceMessage::TurnOffDevice))
                        .style(Self::warning_button_style()),
                    Space::with_width(10),
                    button("Shutdown")
                        .on_press(Message::Device(crate::ui::gui::application::messages::DeviceMessage::ShutdownDevice))
                        .style(Self::danger_button_style()),
                ]
                .spacing(0)
                .align_items(Alignment::Center),
            ]
            .spacing(0)
            .align_items(Alignment::Center)
        } else {
            column![]
        };
        
        let content = column![
            title,
            Space::with_height(10),
            connection_controls,
            operation_controls,
        ]
        .spacing(0)
        .align_items(Alignment::Center);
        
        container(content)
            .width(Length::Fill)
            .padding(10)
            .style(Self::section_container_style())
            .into()
    }
    
    /// Create ARM current controls
    /// 
    /// Creates ARM current setting controls using the existing CurrentControls
    /// component with proper integration.
    /// 
    /// # Arguments
    /// * `state` - Current unified application state
    /// 
    /// # Returns
    /// * `Element<Message>` - ARM current controls element
    /// 
    /// # Example
    /// ```
    /// let current_controls = ComponentFactory::create_current_controls(&state);
    /// ```
    pub fn create_current_controls(state: &UnifiedState) -> Element<'static, Message> {
        // Use existing CurrentControls component
        CurrentControls::create_arm_current_controls(
            state.get_arm_current_input(),
            state.get_arm_current_validation(),
            state.is_device_connected(),
        )
    }
    
    /// Create device status display
    /// 
    /// Creates a status display showing current device connection status,
    /// operation state, and key parameters.
    /// 
    /// # Arguments
    /// * `state` - Current unified application state
    /// 
    /// # Returns
    /// * `Element<Message>` - Device status display element
    /// 
    /// # Example
    /// ```
    /// let status_display = ComponentFactory::create_status_display(&state);
    /// ```
    pub fn create_status_display(state: &UnifiedState) -> Element<'static, Message> {
        let title = text("Device Status")
            .size(18)
            .horizontal_alignment(iced::alignment::Horizontal::Center);
        
        // Connection status
        let connection_status = text(state.get_connection_status_display())
            .size(14)
            .style(if state.is_device_connected() {
                Self::success_text_style()
            } else {
                Self::error_text_style()
            });
        
        // Operation status
        let operation_status = text(state.get_operation_status_display())
            .size(14)
            .style(if state.is_busy() {
                Self::warning_text_style()
            } else {
                Self::normal_text_style()
            });
        
        // Current parameters (if available)
        let parameters = if let (Some(arm), Some(fire)) = (
            state.device_state.get_arm_current(),
            state.device_state.get_fire_current(),
        ) {
            column![
                text(format!("ARM Current: {}mA", arm)).size(12),
                text(format!("FIRE Current: {}mA", fire)).size(12),
            ]
            .spacing(2)
            .align_items(Alignment::Start)
        } else {
            column![
                text("Parameters not available").size(12),
            ]
            .spacing(2)
            .align_items(Alignment::Start)
        };
        
        let content = column![
            title,
            Space::with_height(10),
            connection_status,
            Space::with_height(5),
            operation_status,
            Space::with_height(10),
            parameters,
        ]
        .spacing(0)
        .align_items(Alignment::Center);
        
        container(content)
            .width(Length::Fill)
            .padding(10)
            .style(Self::info_container_style())
            .into()
    }
    
    /// Create device information display
    /// 
    /// Creates an information display showing device details, firmware
    /// version, and other device-specific information.
    /// 
    /// # Arguments
    /// * `state` - Current unified application state
    /// 
    /// # Returns
    /// * `Element<Message>` - Device information display element
    /// 
    /// # Example
    /// ```
    /// let info_display = ComponentFactory::create_info_display(&state);
    /// ```
    pub fn create_info_display(state: &UnifiedState) -> Element<'static, Message> {
        let title = text("Device Information")
            .size(18)
            .horizontal_alignment(iced::alignment::Horizontal::Center);
        
        let info_content = if let Some(info_text) = state.get_device_info_display() {
            column![
                text(info_text).size(12),
                Space::with_height(10),
                button("Refresh Info")
                    .on_press(Message::Device(crate::ui::gui::application::messages::DeviceMessage::ReadDeviceInfo))
                    .style(Self::secondary_button_style()),
            ]
            .spacing(0)
            .align_items(Alignment::Center)
        } else {
            column![
                text("No device information available").size(12),
                Space::with_height(10),
                if state.is_device_connected() {
                    button("Read Device Info")
                        .on_press(Message::Device(crate::ui::gui::application::messages::DeviceMessage::ReadDeviceInfo))
                        .style(Self::primary_button_style())
                        .into()
                } else {
                    text("Connect device to read information").size(10).into()
                },
            ]
            .spacing(0)
            .align_items(Alignment::Center)
        };
        
        let content = column![
            title,
            Space::with_height(10),
            info_content,
        ]
        .spacing(0)
        .align_items(Alignment::Center);
        
        container(content)
            .width(Length::Fill)
            .padding(10)
            .style(Self::info_container_style())
            .into()
    }
    
    // Styling functions
    fn stage_button_style() -> fn(&iced::Theme) -> iced::widget::button::Appearance {
        |_theme| {
            iced::widget::button::Appearance {
                background: Some(iced::Background::Color(iced::Color::from_rgb(0.2, 0.6, 0.9))),
                text_color: iced::Color::WHITE,
                border: iced::Border {
                    color: iced::Color::from_rgb(0.1, 0.4, 0.7),
                    width: 1.0,
                    radius: 4.0.into(),
                },
                ..Default::default()
            }
        }
    }
    
    fn primary_button_style() -> fn(&iced::Theme) -> iced::widget::button::Appearance {
        |_theme| {
            iced::widget::button::Appearance {
                background: Some(iced::Background::Color(iced::Color::from_rgb(0.1, 0.7, 0.1))),
                text_color: iced::Color::WHITE,
                border: iced::Border {
                    color: iced::Color::from_rgb(0.0, 0.5, 0.0),
                    width: 1.0,
                    radius: 4.0.into(),
                },
                ..Default::default()
            }
        }
    }
    
    fn secondary_button_style() -> fn(&iced::Theme) -> iced::widget::button::Appearance {
        |_theme| {
            iced::widget::button::Appearance {
                background: Some(iced::Background::Color(iced::Color::from_rgb(0.6, 0.6, 0.6))),
                text_color: iced::Color::WHITE,
                border: iced::Border {
                    color: iced::Color::from_rgb(0.4, 0.4, 0.4),
                    width: 1.0,
                    radius: 4.0.into(),
                },
                ..Default::default()
            }
        }
    }
    
    fn warning_button_style() -> fn(&iced::Theme) -> iced::widget::button::Appearance {
        |_theme| {
            iced::widget::button::Appearance {
                background: Some(iced::Background::Color(iced::Color::from_rgb(0.9, 0.6, 0.1))),
                text_color: iced::Color::WHITE,
                border: iced::Border {
                    color: iced::Color::from_rgb(0.7, 0.4, 0.0),
                    width: 1.0,
                    radius: 4.0.into(),
                },
                ..Default::default()
            }
        }
    }
    
    fn danger_button_style() -> fn(&iced::Theme) -> iced::widget::button::Appearance {
        |_theme| {
            iced::widget::button::Appearance {
                background: Some(iced::Background::Color(iced::Color::from_rgb(0.9, 0.1, 0.1))),
                text_color: iced::Color::WHITE,
                border: iced::Border {
                    color: iced::Color::from_rgb(0.7, 0.0, 0.0),
                    width: 1.0,
                    radius: 4.0.into(),
                },
                ..Default::default()
            }
        }
    }
    
    fn section_container_style() -> fn(&iced::Theme) -> iced::widget::container::Appearance {
        |_theme| {
            iced::widget::container::Appearance {
                background: Some(iced::Background::Color(iced::Color::from_rgb(0.98, 0.98, 0.98))),
                border: iced::Border {
                    color: iced::Color::from_rgb(0.8, 0.8, 0.8),
                    width: 1.0,
                    radius: 6.0.into(),
                },
                text_color: Some(iced::Color::BLACK),
                ..Default::default()
            }
        }
    }
    
    fn info_container_style() -> fn(&iced::Theme) -> iced::widget::container::Appearance {
        |_theme| {
            iced::widget::container::Appearance {
                background: Some(iced::Background::Color(iced::Color::from_rgb(0.96, 0.96, 0.98))),
                border: iced::Border {
                    color: iced::Color::from_rgb(0.7, 0.7, 0.8),
                    width: 1.0,
                    radius: 6.0.into(),
                },
                text_color: Some(iced::Color::BLACK),
                ..Default::default()
            }
        }
    }
    
    fn success_text_style() -> fn(&iced::Theme) -> iced::widget::text::Appearance {
        |_theme| {
            iced::widget::text::Appearance {
                color: Some(iced::Color::from_rgb(0.1, 0.7, 0.1)),
            }
        }
    }
    
    fn error_text_style() -> fn(&iced::Theme) -> iced::widget::text::Appearance {
        |_theme| {
            iced::widget::text::Appearance {
                color: Some(iced::Color::from_rgb(0.9, 0.1, 0.1)),
            }
        }
    }
    
    fn warning_text_style() -> fn(&iced::Theme) -> iced::widget::text::Appearance {
        |_theme| {
            iced::widget::text::Appearance {
                color: Some(iced::Color::from_rgb(0.9, 0.6, 0.1)),
            }
        }
    }
    
    fn normal_text_style() -> fn(&iced::Theme) -> iced::widget::text::Appearance {
        |_theme| {
            iced::widget::text::Appearance {
                color: Some(iced::Color::BLACK),
            }
        }
    }
}
