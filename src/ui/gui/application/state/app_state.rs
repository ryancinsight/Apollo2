//! Application state management for Lumidox II Controller GUI
//!
//! This module provides UI state management including current view, input
//! validation, notifications, and user interface state tracking. It implements
//! proper Iced state management patterns with comprehensive state tracking
//! for optimal user experience and interface responsiveness.
//!
//! The app state system provides:
//! - Current view and navigation state management
//! - Input validation state and error tracking
//! - Notification and message display state
//! - User interface interaction state
//! - Proper state transitions and validation

use std::collections::HashMap;

/// Application view states
/// 
/// Defines the different views available in the GUI application
/// for navigation and state management.
#[derive(Debug, Clone, PartialEq)]
pub enum AppView {
    /// Main control view with device controls and information
    Main,
    /// Device information and status view
    DeviceInfo,
    /// Settings and configuration view
    Settings,
    /// Help and documentation view
    Help,
    /// Error display view
    Error,
}

impl Default for AppView {
    fn default() -> Self {
        AppView::Main
    }
}

/// Input validation state
/// 
/// Tracks validation state for various input fields in the GUI
/// with error messages and validation status.
#[derive(Debug, Clone, Default)]
pub struct ValidationState {
    /// ARM current input validation
    pub arm_current_valid: bool,
    /// ARM current validation message
    pub arm_current_message: Option<String>,
    /// Stage current input validation
    pub stage_current_valid: bool,
    /// Stage current validation message
    pub stage_current_message: Option<String>,
    /// General validation errors
    pub general_errors: Vec<String>,
}

impl ValidationState {
    /// Create new validation state
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Clear all validation errors
    pub fn clear_all(&mut self) {
        self.arm_current_valid = true;
        self.arm_current_message = None;
        self.stage_current_valid = true;
        self.stage_current_message = None;
        self.general_errors.clear();
    }
    
    /// Set ARM current validation
    pub fn set_arm_current_validation(&mut self, valid: bool, message: Option<String>) {
        self.arm_current_valid = valid;
        self.arm_current_message = message;
    }
    
    /// Set stage current validation
    pub fn set_stage_current_validation(&mut self, valid: bool, message: Option<String>) {
        self.stage_current_valid = valid;
        self.stage_current_message = message;
    }
    
    /// Add general error
    pub fn add_general_error(&mut self, error: String) {
        self.general_errors.push(error);
    }
    
    /// Check if any validation errors exist
    pub fn has_errors(&self) -> bool {
        !self.arm_current_valid || !self.stage_current_valid || !self.general_errors.is_empty()
    }
}

/// Notification state
/// 
/// Manages notification display including success messages, warnings,
/// and error notifications with proper timing and display control.
#[derive(Debug, Clone)]
pub struct NotificationState {
    /// Current notification message
    pub message: Option<String>,
    /// Notification type
    pub notification_type: NotificationType,
    /// Whether notification is visible
    pub visible: bool,
    /// Auto-dismiss timer (in seconds)
    pub auto_dismiss_timer: Option<u32>,
}

impl Default for NotificationState {
    fn default() -> Self {
        Self {
            message: None,
            notification_type: NotificationType::Info,
            visible: false,
            auto_dismiss_timer: None,
        }
    }
}

impl NotificationState {
    /// Create new notification state
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Show notification
    pub fn show(&mut self, message: String, notification_type: NotificationType, auto_dismiss: Option<u32>) {
        self.message = Some(message);
        self.notification_type = notification_type;
        self.visible = true;
        self.auto_dismiss_timer = auto_dismiss;
    }
    
    /// Hide notification
    pub fn hide(&mut self) {
        self.visible = false;
        self.message = None;
        self.auto_dismiss_timer = None;
    }
    
    /// Update auto-dismiss timer
    pub fn update_timer(&mut self) {
        if let Some(timer) = &mut self.auto_dismiss_timer {
            if *timer > 0 {
                *timer -= 1;
            } else {
                self.hide();
            }
        }
    }
}

/// Notification types
/// 
/// Defines different types of notifications for proper styling
/// and user feedback.
#[derive(Debug, Clone, PartialEq)]
pub enum NotificationType {
    /// Information notification
    Info,
    /// Success notification
    Success,
    /// Warning notification
    Warning,
    /// Error notification
    Error,
}

/// Input field state
/// 
/// Tracks the state of individual input fields including current
/// values, focus state, and validation status.
#[derive(Debug, Clone, Default)]
pub struct InputFieldState {
    /// Current input value
    pub value: String,
    /// Whether field is focused
    pub focused: bool,
    /// Whether field has been modified
    pub modified: bool,
    /// Validation state
    pub valid: bool,
    /// Validation message
    pub validation_message: Option<String>,
}

impl InputFieldState {
    /// Create new input field state
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Set input value
    pub fn set_value(&mut self, value: String) {
        self.value = value;
        self.modified = true;
    }
    
    /// Set focus state
    pub fn set_focused(&mut self, focused: bool) {
        self.focused = focused;
    }
    
    /// Set validation state
    pub fn set_validation(&mut self, valid: bool, message: Option<String>) {
        self.valid = valid;
        self.validation_message = message;
    }
    
    /// Clear field
    pub fn clear(&mut self) {
        self.value.clear();
        self.modified = false;
        self.valid = true;
        self.validation_message = None;
    }
}

/// Main application state
/// 
/// Comprehensive application state management including view state,
/// input tracking, notifications, and user interface state.
#[derive(Debug, Clone)]
pub struct AppState {
    /// Current application view
    pub current_view: AppView,
    /// Validation state
    pub validation: ValidationState,
    /// Notification state
    pub notifications: NotificationState,
    /// Input field states
    pub input_fields: HashMap<String, InputFieldState>,
    /// Whether application is busy (loading/processing)
    pub busy: bool,
    /// Last operation status
    pub last_operation_success: Option<bool>,
    /// Application settings
    pub settings: AppSettings,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            current_view: AppView::default(),
            validation: ValidationState::new(),
            notifications: NotificationState::new(),
            input_fields: HashMap::new(),
            busy: false,
            last_operation_success: None,
            settings: AppSettings::default(),
        }
    }
}

impl AppState {
    /// Create new application state
    pub fn new() -> Self {
        let mut state = Self::default();
        
        // Initialize common input fields
        state.input_fields.insert("arm_current".to_string(), InputFieldState::new());
        state.input_fields.insert("stage_current".to_string(), InputFieldState::new());
        
        state
    }
    
    /// Set current view
    pub fn set_view(&mut self, view: AppView) {
        self.current_view = view;
    }
    
    /// Set busy state
    pub fn set_busy(&mut self, busy: bool) {
        self.busy = busy;
    }
    
    /// Set last operation result
    pub fn set_last_operation(&mut self, success: bool) {
        self.last_operation_success = Some(success);
    }
    
    /// Get input field state
    pub fn get_input_field(&self, field_name: &str) -> Option<&InputFieldState> {
        self.input_fields.get(field_name)
    }
    
    /// Get mutable input field state
    pub fn get_input_field_mut(&mut self, field_name: &str) -> Option<&mut InputFieldState> {
        self.input_fields.get_mut(field_name)
    }
    
    /// Update input field value
    pub fn update_input_field(&mut self, field_name: &str, value: String) {
        if let Some(field) = self.input_fields.get_mut(field_name) {
            field.set_value(value);
        } else {
            let mut field = InputFieldState::new();
            field.set_value(value);
            self.input_fields.insert(field_name.to_string(), field);
        }
    }
    
    /// Clear all input fields
    pub fn clear_all_inputs(&mut self) {
        for field in self.input_fields.values_mut() {
            field.clear();
        }
    }
    
    /// Show notification
    pub fn show_notification(&mut self, message: String, notification_type: NotificationType, auto_dismiss: Option<u32>) {
        self.notifications.show(message, notification_type, auto_dismiss);
    }
    
    /// Hide notification
    pub fn hide_notification(&mut self) {
        self.notifications.hide();
    }
    
    /// Update application state (called periodically)
    pub fn update(&mut self) {
        self.notifications.update_timer();
    }
}

/// Application settings
/// 
/// User preferences and configuration settings for the GUI application.
#[derive(Debug, Clone)]
pub struct AppSettings {
    /// Auto-refresh interval for device status (in seconds)
    pub auto_refresh_interval: u32,
    /// Whether to show confirmation dialogs
    pub show_confirmations: bool,
    /// Whether to use compact layout
    pub compact_layout: bool,
    /// Theme preference
    pub theme: String,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            auto_refresh_interval: 5,
            show_confirmations: true,
            compact_layout: false,
            theme: "default".to_string(),
        }
    }
}
