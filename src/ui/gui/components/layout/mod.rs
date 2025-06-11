//! Layout components module for Lumidox II Controller GUI
//!
//! This module organizes and exports all layout-related UI components
//! for the Lumidox II Controller GUI application. It provides a centralized
//! access point for main layout, control panel layout, and responsive design
//! components with proper module organization and re-exports.
//!
//! The layout module includes:
//! - Main application layout with responsive design
//! - Control panel layout organization matching CLI menu structure
//! - Consistent styling and layout patterns across components
//! - Proper container management and element positioning
//! - Integration with existing control and display components

// Import layout component modules
pub mod main_layout;
pub mod control_panel;

// Re-export layout components for easy access
pub use main_layout::MainLayout;
pub use control_panel::ControlPanelLayout;

use iced::{
    widget::{column, row, container, Space},
    Element, Length, Alignment,
};
use crate::device::LumidoxDevice;

/// Layout components coordinator
/// 
/// Provides high-level coordination and organization of all layout
/// components with consistent styling and responsive design patterns.
pub struct LayoutComponents;

impl LayoutComponents {
    /// Create complete application layout
    /// 
    /// Creates the complete application layout combining main layout structure
    /// with organized control panels and information displays.
    /// 
    /// # Arguments
    /// * `stage_controls` - Stage firing controls element
    /// * `device_controls` - Device control buttons element
    /// * `current_controls` - ARM current setting controls element
    /// * `status_display` - Status display element
    /// * `info_display` - Information display element
    /// * `is_connected` - Whether device is currently connected
    /// * `device_info` - Optional device information for header
    /// 
    /// # Returns
    /// * `Element<Message>` - Complete application layout
    /// 
    /// # Example
    /// ```
    /// let app_layout = LayoutComponents::create_application_layout(
    ///     stage_controls, device_controls, current_controls,
    ///     status_display, info_display, true, Some("Lumidox II")
    /// );
    /// ```
    pub fn create_application_layout<Message>(
        stage_controls: Element<'static, Message>,
        device_controls: Element<'static, Message>,
        current_controls: Element<'static, Message>,
        status_display: Element<'static, Message>,
        info_display: Element<'static, Message>,
        is_connected: bool,
        device_info: Option<&str>,
    ) -> Element<'static, Message>
    where
        Message: Clone + 'static,
    {
        // Create organized control panel
        let controls_panel = ControlPanelLayout::create_control_panel(
            stage_controls,
            device_controls,
            current_controls,
        );
        
        // Create information panel
        let info_panel = Self::create_info_panel(status_display, info_display);
        
        // Create complete layout with header
        MainLayout::create_complete_layout(
            controls_panel,
            info_panel,
            is_connected,
            device_info,
        )
    }
    
    /// Create compact application layout
    /// 
    /// Creates a compact application layout suitable for smaller screens
    /// or when space is limited.
    /// 
    /// # Arguments
    /// * `stage_controls` - Stage firing controls element
    /// * `device_controls` - Device control buttons element
    /// * `current_controls` - ARM current setting controls element
    /// * `status_display` - Status display element
    /// * `info_display` - Information display element
    /// * `is_connected` - Whether device is currently connected
    /// * `device_info` - Optional device information for header
    /// 
    /// # Returns
    /// * `Element<Message>` - Compact application layout
    /// 
    /// # Example
    /// ```
    /// let compact_layout = LayoutComponents::create_compact_layout(
    ///     stage_controls, device_controls, current_controls,
    ///     status_display, info_display, true, Some("Lumidox II")
    /// );
    /// ```
    pub fn create_compact_layout<Message>(
        stage_controls: Element<'static, Message>,
        device_controls: Element<'static, Message>,
        current_controls: Element<'static, Message>,
        status_display: Element<'static, Message>,
        info_display: Element<'static, Message>,
        is_connected: bool,
        device_info: Option<&str>,
    ) -> Element<'static, Message>
    where
        Message: Clone + 'static,
    {
        // Create compact control panel
        let controls_panel = ControlPanelLayout::create_compact_panel(
            stage_controls,
            device_controls,
            current_controls,
        );
        
        // Create compact information panel
        let info_panel = Self::create_compact_info_panel(status_display, info_display);
        
        // Create complete layout with header
        MainLayout::create_complete_layout(
            controls_panel,
            info_panel,
            is_connected,
            device_info,
        )
    }
    
    /// Create horizontal application layout
    /// 
    /// Creates a horizontal application layout suitable for wide screens
    /// where horizontal space is abundant.
    /// 
    /// # Arguments
    /// * `stage_controls` - Stage firing controls element
    /// * `device_controls` - Device control buttons element
    /// * `current_controls` - ARM current setting controls element
    /// * `status_display` - Status display element
    /// * `info_display` - Information display element
    /// * `is_connected` - Whether device is currently connected
    /// * `device_info` - Optional device information for header
    /// 
    /// # Returns
    /// * `Element<Message>` - Horizontal application layout
    /// 
    /// # Example
    /// ```
    /// let horizontal_layout = LayoutComponents::create_horizontal_layout(
    ///     stage_controls, device_controls, current_controls,
    ///     status_display, info_display, true, Some("Lumidox II")
    /// );
    /// ```
    pub fn create_horizontal_layout<Message>(
        stage_controls: Element<'static, Message>,
        device_controls: Element<'static, Message>,
        current_controls: Element<'static, Message>,
        status_display: Element<'static, Message>,
        info_display: Element<'static, Message>,
        is_connected: bool,
        device_info: Option<&str>,
    ) -> Element<'static, Message>
    where
        Message: Clone + 'static,
    {
        // Create horizontal control panel
        let controls_panel = ControlPanelLayout::create_horizontal_panel(
            stage_controls,
            device_controls,
            current_controls,
        );
        
        // Create information panel
        let info_panel = Self::create_info_panel(status_display, info_display);
        
        // Create complete layout with header
        MainLayout::create_complete_layout(
            controls_panel,
            info_panel,
            is_connected,
            device_info,
        )
    }
    
    /// Create information panel
    /// 
    /// Creates an organized information panel combining status display
    /// and device information with proper spacing.
    /// 
    /// # Arguments
    /// * `status_display` - Status display element
    /// * `info_display` - Information display element
    /// 
    /// # Returns
    /// * `Element<Message>` - Organized information panel
    fn create_info_panel<Message>(
        status_display: Element<'static, Message>,
        info_display: Element<'static, Message>,
    ) -> Element<'static, Message>
    where
        Message: Clone + 'static,
    {
        column![
            status_display,
            Space::with_height(15),
            info_display,
        ]
        .spacing(0)
        .align_items(Alignment::Fill)
        .into()
    }
    
    /// Create compact information panel
    /// 
    /// Creates a compact information panel with reduced spacing
    /// suitable for smaller screens.
    /// 
    /// # Arguments
    /// * `status_display` - Status display element
    /// * `info_display` - Information display element
    /// 
    /// # Returns
    /// * `Element<Message>` - Compact information panel
    fn create_compact_info_panel<Message>(
        status_display: Element<'static, Message>,
        info_display: Element<'static, Message>,
    ) -> Element<'static, Message>
    where
        Message: Clone + 'static,
    {
        column![
            status_display,
            Space::with_height(8),
            info_display,
        ]
        .spacing(0)
        .align_items(Alignment::Fill)
        .into()
    }
    
    /// Create responsive layout
    /// 
    /// Creates a responsive layout that adapts to different screen sizes
    /// and orientations automatically.
    /// 
    /// # Arguments
    /// * `stage_controls` - Stage firing controls element
    /// * `device_controls` - Device control buttons element
    /// * `current_controls` - ARM current setting controls element
    /// * `status_display` - Status display element
    /// * `info_display` - Information display element
    /// * `is_connected` - Whether device is currently connected
    /// * `device_info` - Optional device information for header
    /// * `screen_width` - Current screen width for responsive decisions
    /// 
    /// # Returns
    /// * `Element<Message>` - Responsive application layout
    /// 
    /// # Example
    /// ```
    /// let responsive_layout = LayoutComponents::create_responsive_layout(
    ///     stage_controls, device_controls, current_controls,
    ///     status_display, info_display, true, Some("Lumidox II"), 1200
    /// );
    /// ```
    pub fn create_responsive_layout<Message>(
        stage_controls: Element<'static, Message>,
        device_controls: Element<'static, Message>,
        current_controls: Element<'static, Message>,
        status_display: Element<'static, Message>,
        info_display: Element<'static, Message>,
        is_connected: bool,
        device_info: Option<&str>,
        screen_width: u32,
    ) -> Element<'static, Message>
    where
        Message: Clone + 'static,
    {
        // Choose layout based on screen width
        if screen_width >= 1200 {
            // Wide screen: use horizontal layout
            Self::create_horizontal_layout(
                stage_controls, device_controls, current_controls,
                status_display, info_display, is_connected, device_info
            )
        } else if screen_width >= 800 {
            // Medium screen: use standard layout
            Self::create_application_layout(
                stage_controls, device_controls, current_controls,
                status_display, info_display, is_connected, device_info
            )
        } else {
            // Small screen: use compact layout
            Self::create_compact_layout(
                stage_controls, device_controls, current_controls,
                status_display, info_display, is_connected, device_info
            )
        }
    }
}
