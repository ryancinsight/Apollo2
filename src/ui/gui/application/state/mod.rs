//! State management module for Lumidox II Controller GUI
//!
//! This module organizes and exports all state-related components for the
//! Lumidox II Controller GUI application. It provides a centralized access
//! point for application state, device state, and unified state management
//! with proper module organization and re-exports.
//!
//! The state module includes:
//! - Application UI state management with validation and notifications
//! - Device connection and status state tracking
//! - Unified state coordination and synchronization
//! - Proper state transitions and validation
//! - Integration with Iced application state patterns

// Import state management modules
pub mod app_state;
pub mod device_state;

// Re-export state components for easy access
pub use app_state::{
    AppState, AppView, ValidationState, NotificationState, NotificationType,
    InputFieldState, AppSettings
};
pub use device_state::{
    DeviceState, ConnectionState, OperationState, CachedParameters
};

use crate::device::LumidoxDevice;
use crate::core::{LumidoxError, Result};

/// Unified application state
/// 
/// Combines application UI state and device state into a unified
/// state management structure for comprehensive GUI state tracking.
#[derive(Debug, Clone)]
pub struct UnifiedState {
    /// Application UI state
    pub app_state: AppState,
    /// Device connection and status state
    pub device_state: DeviceState,
}

impl Default for UnifiedState {
    fn default() -> Self {
        Self {
            app_state: AppState::new(),
            device_state: DeviceState::new(),
        }
    }
}

impl UnifiedState {
    /// Create new unified state
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Create unified state with device configuration
    pub fn with_device_config(
        port_name: Option<String>,
        auto_detect: bool,
        verbose: bool,
        optimize_transitions: bool,
    ) -> Self {
        Self {
            app_state: AppState::new(),
            device_state: DeviceState::with_config(
                port_name,
                auto_detect,
                verbose,
                optimize_transitions,
            ),
        }
    }
    
    /// Check if device is connected
    pub fn is_device_connected(&self) -> bool {
        self.device_state.is_connected()
    }
    
    /// Check if application is busy
    pub fn is_busy(&self) -> bool {
        self.app_state.busy || self.device_state.is_busy()
    }
    
    /// Set busy state
    pub fn set_busy(&mut self, busy: bool, operation: Option<String>) {
        self.app_state.set_busy(busy);
        
        if busy {
            if let Some(op) = operation {
                self.device_state.set_operation_state(OperationState::Busy(op));
            }
        } else {
            self.device_state.set_operation_state(OperationState::Idle);
        }
    }
    
    /// Set device connection state
    pub fn set_connection_state(&mut self, state: ConnectionState) {
        self.device_state.set_connection_state(state);
        
        // Update app state based on connection
        match &self.device_state.connection_state {
            ConnectionState::Connected => {
                self.app_state.show_notification(
                    "Device connected successfully".to_string(),
                    NotificationType::Success,
                    Some(3),
                );
            }
            ConnectionState::Failed(error) => {
                self.app_state.show_notification(
                    format!("Connection failed: {}", error),
                    NotificationType::Error,
                    Some(5),
                );
            }
            ConnectionState::Disconnected => {
                self.app_state.show_notification(
                    "Device disconnected".to_string(),
                    NotificationType::Warning,
                    Some(3),
                );
            }
            _ => {}
        }
    }
    
    /// Set operation result
    pub fn set_operation_result(&mut self, success: bool, message: String) {
        self.app_state.set_last_operation(success);
        
        if success {
            self.device_state.set_operation_state(OperationState::Success(message.clone()));
            self.app_state.show_notification(
                message,
                NotificationType::Success,
                Some(3),
            );
        } else {
            self.device_state.set_operation_state(OperationState::Failed(message.clone()));
            self.app_state.show_notification(
                format!("Operation failed: {}", message),
                NotificationType::Error,
                Some(5),
            );
        }
    }
    
    /// Update input field with validation
    pub fn update_input_field(&mut self, field_name: &str, value: String) -> Result<()> {
        self.app_state.update_input_field(field_name, value.clone());
        
        // Perform validation based on field type
        match field_name {
            "arm_current" => {
                self.validate_arm_current(&value)?;
            }
            "stage_current" => {
                self.validate_stage_current(&value)?;
            }
            _ => {}
        }
        
        Ok(())
    }
    
    /// Validate ARM current input
    fn validate_arm_current(&mut self, value: &str) -> Result<()> {
        if value.is_empty() {
            self.app_state.validation.set_arm_current_validation(true, None);
            return Ok(());
        }
        
        match value.parse::<u16>() {
            Ok(current) => {
                if current > 0 && current <= 5000 {
                    self.app_state.validation.set_arm_current_validation(true, None);
                } else {
                    self.app_state.validation.set_arm_current_validation(
                        false,
                        Some("ARM current must be between 1 and 5000 mA".to_string()),
                    );
                }
            }
            Err(_) => {
                self.app_state.validation.set_arm_current_validation(
                    false,
                    Some("Invalid number format".to_string()),
                );
            }
        }
        
        Ok(())
    }
    
    /// Validate stage current input
    fn validate_stage_current(&mut self, value: &str) -> Result<()> {
        if value.is_empty() {
            self.app_state.validation.set_stage_current_validation(true, None);
            return Ok(());
        }
        
        match value.parse::<u16>() {
            Ok(current) => {
                if current > 0 && current <= 5000 {
                    self.app_state.validation.set_stage_current_validation(true, None);
                } else {
                    self.app_state.validation.set_stage_current_validation(
                        false,
                        Some("Stage current must be between 1 and 5000 mA".to_string()),
                    );
                }
            }
            Err(_) => {
                self.app_state.validation.set_stage_current_validation(
                    false,
                    Some("Invalid number format".to_string()),
                );
            }
        }
        
        Ok(())
    }
    
    /// Get ARM current input value
    pub fn get_arm_current_input(&self) -> String {
        self.app_state
            .get_input_field("arm_current")
            .map(|field| field.value.clone())
            .unwrap_or_default()
    }
    
    /// Get stage current input value
    pub fn get_stage_current_input(&self) -> String {
        self.app_state
            .get_input_field("stage_current")
            .map(|field| field.value.clone())
            .unwrap_or_default()
    }
    
    /// Get ARM current validation message
    pub fn get_arm_current_validation(&self) -> Option<&str> {
        self.app_state.validation.arm_current_message.as_deref()
    }
    
    /// Get stage current validation message
    pub fn get_stage_current_validation(&self) -> Option<&str> {
        self.app_state.validation.stage_current_message.as_deref()
    }
    
    /// Check if inputs are valid
    pub fn are_inputs_valid(&self) -> bool {
        !self.app_state.validation.has_errors()
    }
    
    /// Update state (called periodically)
    pub fn update(&mut self) {
        self.app_state.update();
    }
    
    /// Clear all state
    pub fn clear(&mut self) {
        self.app_state.clear_all_inputs();
        self.app_state.validation.clear_all();
        self.app_state.hide_notification();
        self.device_state.cached_parameters.clear_cache();
        self.device_state.clear_error();
    }
    
    /// Get device info for display
    pub fn get_device_info_display(&self) -> Option<String> {
        self.device_state.get_device_info().map(|info| {
            format!("Model: {} | Serial: {} | Firmware: {}", 
                info.model_number, 
                info.serial_number, 
                info.firmware_version)
        })
    }
    
    /// Get connection status for display
    pub fn get_connection_status_display(&self) -> String {
        self.device_state.get_connection_description()
    }
    
    /// Get operation status for display
    pub fn get_operation_status_display(&self) -> String {
        self.device_state.get_operation_description()
    }
}

/// State management utilities
/// 
/// Provides utility functions for state management and coordination
/// across the GUI application.
pub struct StateManager;

impl StateManager {
    /// Create initial state from CLI arguments
    pub fn create_initial_state(
        port_name: Option<String>,
        auto_detect: bool,
        verbose: bool,
        optimize_transitions: bool,
    ) -> UnifiedState {
        UnifiedState::with_device_config(
            port_name,
            auto_detect,
            verbose,
            optimize_transitions,
        )
    }
    
    /// Validate state consistency
    pub fn validate_state_consistency(state: &UnifiedState) -> Result<()> {
        // Check for state inconsistencies
        if state.device_state.is_connected() && 
           matches!(state.device_state.connection_state, ConnectionState::Disconnected) {
            return Err(LumidoxError::InvalidState(
                "Device state inconsistency: connected but state is disconnected".to_string()
            ));
        }
        
        Ok(())
    }
    
    /// Synchronize state with device
    pub fn synchronize_with_device(
        state: &mut UnifiedState,
        device: Option<&LumidoxDevice>,
    ) -> Result<()> {
        if let Some(device) = device {
            // Update cached parameters if needed
            if state.device_state.needs_cache_refresh() {
                // This would be called from async handlers to update cache
                state.device_state.invalidate_cache();
            }
        }
        
        Ok(())
    }
}
