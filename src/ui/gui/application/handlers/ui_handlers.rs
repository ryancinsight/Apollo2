//! UI state update handlers for Lumidox II Controller GUI
//!
//! This module provides UI state update handlers for synchronous UiMessage
//! processing that directly modify UnifiedState without async operations.
//! These handlers perform immediate state updates and return Command::none()
//! for synchronous operations that don't require async processing.
//!
//! The UI handlers system provides:
//! - Immediate state updates for UI interactions
//! - Input validation and field management
//! - View navigation and state transitions
//! - Notification and error handling
//! - Settings and configuration management
//! - Synchronous operations with Command::none() returns

use iced::Command;
use crate::ui::gui::application::messages::{Message, UiMessage};
use crate::ui::gui::application::state::{UnifiedState, AppView, NotificationType};
use crate::core::{LumidoxError, Result};

/// UI state update handlers
/// 
/// Provides synchronous handlers for UI message processing that perform
/// immediate state updates without requiring async operations.
pub struct UiHandlers;

impl UiHandlers {
    /// Handle view change operation
    /// 
    /// Updates the current application view and performs any necessary
    /// state transitions for the new view.
    /// 
    /// # Arguments
    /// * `state` - Mutable reference to unified application state
    /// * `view` - New application view to display
    /// 
    /// # Returns
    /// * `Command<Message>` - Command::none() for synchronous operation
    /// 
    /// # Example
    /// ```
    /// let cmd = UiHandlers::handle_view_changed(&mut state, AppView::Settings);
    /// ```
    pub fn handle_view_changed(
        state: &mut UnifiedState,
        view: AppView,
    ) -> Command<Message> {
        // Update current view
        state.app_state.set_view(view.clone());
        
        // Clear any existing notifications when changing views
        state.app_state.hide_notification();
        
        // Perform view-specific initialization
        match view {
            AppView::Main => {
                // Main view - no special initialization needed
            }
            AppView::DeviceInfo => {
                // Device info view - refresh device information if connected
                if state.is_device_connected() {
                    state.device_state.invalidate_cache();
                }
            }
            AppView::Settings => {
                // Settings view - no special initialization needed
            }
            AppView::Help => {
                // Help view - no special initialization needed
            }
            AppView::Error => {
                // Error view - typically set by error conditions
            }
        }
        
        Command::none()
    }
    
    /// Handle input field update operation
    /// 
    /// Updates input field values and performs real-time validation
    /// with immediate feedback to the user.
    /// 
    /// # Arguments
    /// * `state` - Mutable reference to unified application state
    /// * `field_name` - Name of the input field being updated
    /// * `value` - New input field value
    /// 
    /// # Returns
    /// * `Command<Message>` - Command::none() for synchronous operation
    /// 
    /// # Example
    /// ```
    /// let cmd = UiHandlers::handle_input_updated(&mut state, "arm_current", "1000");
    /// ```
    pub fn handle_input_updated(
        state: &mut UnifiedState,
        field_name: String,
        value: String,
    ) -> Command<Message> {
        // Update input field value with validation
        if let Err(error) = state.update_input_field(&field_name, value) {
            // Show validation error as notification
            state.app_state.show_notification(
                format!("Validation error: {}", error),
                NotificationType::Error,
                Some(5),
            );
        }
        
        Command::none()
    }
    
    /// Handle input field focus change operation
    /// 
    /// Updates input field focus state for proper UI feedback
    /// and user experience.
    /// 
    /// # Arguments
    /// * `state` - Mutable reference to unified application state
    /// * `field_name` - Name of the input field
    /// * `focused` - Whether field is now focused
    /// 
    /// # Returns
    /// * `Command<Message>` - Command::none() for synchronous operation
    /// 
    /// # Example
    /// ```
    /// let cmd = UiHandlers::handle_input_focus_changed(&mut state, "arm_current", true);
    /// ```
    pub fn handle_input_focus_changed(
        state: &mut UnifiedState,
        field_name: String,
        focused: bool,
    ) -> Command<Message> {
        // Update input field focus state
        if let Some(field) = state.app_state.get_input_field_mut(&field_name) {
            field.set_focused(focused);
        }
        
        Command::none()
    }
    
    /// Handle error dismissal operation
    /// 
    /// Clears error state and returns to normal operation mode.
    /// 
    /// # Arguments
    /// * `state` - Mutable reference to unified application state
    /// 
    /// # Returns
    /// * `Command<Message>` - Command::none() for synchronous operation
    /// 
    /// # Example
    /// ```
    /// let cmd = UiHandlers::handle_error_dismissed(&mut state);
    /// ```
    pub fn handle_error_dismissed(state: &mut UnifiedState) -> Command<Message> {
        // Clear error state
        state.device_state.clear_error();
        state.app_state.validation.clear_all();
        
        // Return to main view if currently in error view
        if matches!(state.app_state.current_view, AppView::Error) {
            state.app_state.set_view(AppView::Main);
        }
        
        Command::none()
    }
    
    /// Handle notification dismissal operation
    /// 
    /// Hides current notification and clears notification state.
    /// 
    /// # Arguments
    /// * `state` - Mutable reference to unified application state
    /// 
    /// # Returns
    /// * `Command<Message>` - Command::none() for synchronous operation
    /// 
    /// # Example
    /// ```
    /// let cmd = UiHandlers::handle_notification_dismissed(&mut state);
    /// ```
    pub fn handle_notification_dismissed(state: &mut UnifiedState) -> Command<Message> {
        // Hide current notification
        state.app_state.hide_notification();
        
        Command::none()
    }
    
    /// Handle show notification operation
    /// 
    /// Displays a notification to the user with specified type and timing.
    /// 
    /// # Arguments
    /// * `state` - Mutable reference to unified application state
    /// * `message` - Notification message text
    /// * `notification_type` - Type of notification for styling
    /// * `auto_dismiss` - Optional auto-dismiss timer in seconds
    /// 
    /// # Returns
    /// * `Command<Message>` - Command::none() for synchronous operation
    /// 
    /// # Example
    /// ```
    /// let cmd = UiHandlers::handle_show_notification(
    ///     &mut state, 
    ///     "Operation completed", 
    ///     NotificationType::Success, 
    ///     Some(3)
    /// );
    /// ```
    pub fn handle_show_notification(
        state: &mut UnifiedState,
        message: String,
        notification_type: NotificationType,
        auto_dismiss: Option<u32>,
    ) -> Command<Message> {
        // Show notification
        state.app_state.show_notification(message, notification_type, auto_dismiss);
        
        Command::none()
    }
    
    /// Handle refresh request operation
    /// 
    /// Initiates refresh of device status and cached parameters.
    /// This is one of the few UI operations that may trigger async work.
    /// 
    /// # Arguments
    /// * `state` - Mutable reference to unified application state
    /// 
    /// # Returns
    /// * `Command<Message>` - Command for refresh operation
    /// 
    /// # Example
    /// ```
    /// let cmd = UiHandlers::handle_refresh_requested(&mut state);
    /// ```
    pub fn handle_refresh_requested(state: &mut UnifiedState) -> Command<Message> {
        // Invalidate cached parameters to force refresh
        state.device_state.invalidate_cache();
        
        // If device is connected, trigger status read
        if state.is_device_connected() {
            // This would typically trigger a device status read command
            // For now, we'll just invalidate cache and let the UI refresh
            state.app_state.show_notification(
                "Refreshing device status...".to_string(),
                NotificationType::Info,
                Some(2),
            );
        } else {
            state.app_state.show_notification(
                "No device connected to refresh".to_string(),
                NotificationType::Warning,
                Some(3),
            );
        }
        
        Command::none()
    }
    
    /// Handle setting change operation
    /// 
    /// Updates application settings and applies changes immediately.
    /// 
    /// # Arguments
    /// * `state` - Mutable reference to unified application state
    /// * `setting_name` - Name of the setting being changed
    /// * `value` - New setting value
    /// 
    /// # Returns
    /// * `Command<Message>` - Command::none() for synchronous operation
    /// 
    /// # Example
    /// ```
    /// let cmd = UiHandlers::handle_setting_changed(&mut state, "theme", "dark");
    /// ```
    pub fn handle_setting_changed(
        state: &mut UnifiedState,
        setting_name: String,
        value: String,
    ) -> Command<Message> {
        // Update application settings
        match setting_name.as_str() {
            "auto_refresh_interval" => {
                if let Ok(interval) = value.parse::<u32>() {
                    state.app_state.settings.auto_refresh_interval = interval;
                    state.app_state.show_notification(
                        format!("Auto-refresh interval set to {} seconds", interval),
                        NotificationType::Success,
                        Some(3),
                    );
                } else {
                    state.app_state.show_notification(
                        "Invalid auto-refresh interval value".to_string(),
                        NotificationType::Error,
                        Some(3),
                    );
                }
            }
            "show_confirmations" => {
                state.app_state.settings.show_confirmations = value.to_lowercase() == "true";
                state.app_state.show_notification(
                    format!("Confirmation dialogs {}", 
                        if state.app_state.settings.show_confirmations { "enabled" } else { "disabled" }),
                    NotificationType::Success,
                    Some(3),
                );
            }
            "theme" => {
                state.app_state.settings.theme = value.clone();
                state.app_state.show_notification(
                    format!("Theme changed to {}", value),
                    NotificationType::Success,
                    Some(3),
                );
            }
            _ => {
                state.app_state.show_notification(
                    format!("Unknown setting: {}", setting_name),
                    NotificationType::Warning,
                    Some(3),
                );
            }
        }
        
        Command::none()
    }
    
    /// Handle layout mode change operation
    /// 
    /// Updates layout mode between compact and normal layouts.
    /// 
    /// # Arguments
    /// * `state` - Mutable reference to unified application state
    /// * `compact` - Whether to use compact layout
    /// 
    /// # Returns
    /// * `Command<Message>` - Command::none() for synchronous operation
    /// 
    /// # Example
    /// ```
    /// let cmd = UiHandlers::handle_layout_mode_changed(&mut state, true);
    /// ```
    pub fn handle_layout_mode_changed(
        state: &mut UnifiedState,
        compact: bool,
    ) -> Command<Message> {
        // Update layout mode setting
        state.app_state.settings.compact_layout = compact;
        
        // Show confirmation
        state.app_state.show_notification(
            format!("Layout changed to {}", if compact { "compact" } else { "normal" }),
            NotificationType::Info,
            Some(2),
        );
        
        Command::none()
    }
    
    /// Handle application tick operation
    /// 
    /// Performs periodic updates and maintenance tasks.
    /// 
    /// # Arguments
    /// * `state` - Mutable reference to unified application state
    /// 
    /// # Returns
    /// * `Command<Message>` - Command::none() for synchronous operation
    /// 
    /// # Example
    /// ```
    /// let cmd = UiHandlers::handle_tick(&mut state);
    /// ```
    pub fn handle_tick(state: &mut UnifiedState) -> Command<Message> {
        // Update application state (handles notification timers, etc.)
        state.update();
        
        Command::none()
    }
    
    /// Handle clear all inputs operation
    /// 
    /// Clears all input fields and resets validation state.
    /// 
    /// # Arguments
    /// * `state` - Mutable reference to unified application state
    /// 
    /// # Returns
    /// * `Command<Message>` - Command::none() for synchronous operation
    /// 
    /// # Example
    /// ```
    /// let cmd = UiHandlers::handle_clear_all_inputs(&mut state);
    /// ```
    pub fn handle_clear_all_inputs(state: &mut UnifiedState) -> Command<Message> {
        // Clear all input fields
        state.app_state.clear_all_inputs();
        
        // Clear validation state
        state.app_state.validation.clear_all();
        
        // Show confirmation
        state.app_state.show_notification(
            "All input fields cleared".to_string(),
            NotificationType::Info,
            Some(2),
        );
        
        Command::none()
    }
    
    /// Handle reset state operation
    /// 
    /// Resets application state to initial values.
    /// 
    /// # Arguments
    /// * `state` - Mutable reference to unified application state
    /// 
    /// # Returns
    /// * `Command<Message>` - Command::none() for synchronous operation
    /// 
    /// # Example
    /// ```
    /// let cmd = UiHandlers::handle_reset_state(&mut state);
    /// ```
    pub fn handle_reset_state(state: &mut UnifiedState) -> Command<Message> {
        // Clear all state
        state.clear();
        
        // Return to main view
        state.app_state.set_view(AppView::Main);
        
        // Show confirmation
        state.app_state.show_notification(
            "Application state reset".to_string(),
            NotificationType::Info,
            Some(3),
        );
        
        Command::none()
    }
}
