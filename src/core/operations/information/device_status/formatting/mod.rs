//! Formatting operations for device status
//!
//! This module provides specialized formatting operations for device status
//! in the Lumidox II Controller. It handles various formatting scenarios
//! including message generation, display formatting, and status presentation.

use crate::core::operations::result_types::DeviceOperationData;

/// Formatting categories for better classification
#[derive(Debug, Clone, PartialEq)]
pub enum FormattingCategory {
    /// Message generation formatting
    Message,
    /// Display formatting
    Display,
    /// Status presentation formatting
    Status,
    /// Summary formatting
    Summary,
}

/// Formatting operations for device status functionality
pub struct FormattingOperations;

impl FormattingOperations {
    /// Format status message based on device status data
    ///
    /// Creates a human-readable status message from the device status data
    /// for consistent messaging across interfaces.
    ///
    /// # Arguments
    /// * `data` - Device status data to format
    ///
    /// # Returns
    /// * `String` - Formatted status message
    pub fn format_status_message(data: &DeviceOperationData) -> String {
        if let DeviceOperationData::DeviceStatus { 
            current_mode, 
            arm_current, 
            fire_current, 
            connection_healthy,
            ready_for_operations,
            .. 
        } = data {
            let mode_str = current_mode.as_deref().unwrap_or("Unknown");
            let arm_str = arm_current.map(|c| c.to_string()).unwrap_or_else(|| "N/A".to_string());
            let fire_str = fire_current.map(|c| c.to_string()).unwrap_or_else(|| "N/A".to_string());
            let health_str = if *connection_healthy { "Healthy" } else { "Issues" };
            let ready_str = if *ready_for_operations { "Ready" } else { "Not Ready" };
            
            format!(
                "Device Status: Mode={}, ARM={}mA, FIRE={}mA, Connection={}, Operations={}",
                mode_str, arm_str, fire_str, health_str, ready_str
            )
        } else {
            "Device status retrieved successfully".to_string()
        }
    }

    /// Format display message for UI presentation
    pub fn format_display_message(data: &DeviceOperationData) -> String {
        if let DeviceOperationData::DeviceStatus { 
            current_mode, 
            arm_current, 
            fire_current, 
            connection_healthy,
            ready_for_operations,
            .. 
        } = data {
            let mode_str = current_mode.as_deref().unwrap_or("Unknown");
            let health_indicator = if *connection_healthy { "✓" } else { "✗" };
            let ready_indicator = if *ready_for_operations { "✓" } else { "✗" };
            
            format!(
                "Mode: {} | ARM: {}mA | FIRE: {}mA | Health: {} | Ready: {}",
                mode_str,
                arm_current.map(|c| c.to_string()).unwrap_or_else(|| "N/A".to_string()),
                fire_current.map(|c| c.to_string()).unwrap_or_else(|| "N/A".to_string()),
                health_indicator,
                ready_indicator
            )
        } else {
            "Status: Available".to_string()
        }
    }

    /// Format summary message for quick overview
    pub fn format_summary_message(data: &DeviceOperationData) -> String {
        if let DeviceOperationData::DeviceStatus { 
            current_mode, 
            connection_healthy,
            ready_for_operations,
            .. 
        } = data {
            let mode_str = current_mode.as_deref().unwrap_or("Unknown");
            let status = if *connection_healthy && *ready_for_operations {
                "Operational"
            } else if *connection_healthy {
                "Connected"
            } else {
                "Disconnected"
            };
            
            format!("{} - {}", mode_str, status)
        } else {
            "Status Available".to_string()
        }
    }

    /// Format detailed status report
    pub fn format_detailed_report(data: &DeviceOperationData) -> String {
        if let DeviceOperationData::DeviceStatus { 
            current_mode, 
            arm_current, 
            fire_current, 
            remote_mode_state,
            connection_healthy,
            ready_for_operations,
        } = data {
            let mut report = String::new();
            
            report.push_str(&format!("Device Mode: {}\n", 
                current_mode.as_deref().unwrap_or("Unknown")));
            
            report.push_str(&format!("ARM Current: {}mA\n", 
                arm_current.map(|c| c.to_string()).unwrap_or_else(|| "N/A".to_string())));
            
            report.push_str(&format!("FIRE Current: {}mA\n", 
                fire_current.map(|c| c.to_string()).unwrap_or_else(|| "N/A".to_string())));
            
            if let Some(remote_state) = remote_mode_state {
                report.push_str(&format!("Remote Mode State: {}\n", remote_state));
            }
            
            report.push_str(&format!("Connection Health: {}\n", 
                if *connection_healthy { "Healthy" } else { "Issues Detected" }));
            
            report.push_str(&format!("Operational Readiness: {}", 
                if *ready_for_operations { "Ready" } else { "Not Ready" }));
            
            report
        } else {
            "Device status information is available".to_string()
        }
    }

    /// Categorize a formatting operation
    pub fn categorize_formatting(format_type: &str) -> FormattingCategory {
        let format_lower = format_type.to_lowercase();
        
        if format_lower.contains("message") {
            FormattingCategory::Message
        } else if format_lower.contains("display") {
            FormattingCategory::Display
        } else if format_lower.contains("summary") {
            FormattingCategory::Summary
        } else {
            FormattingCategory::Status
        }
    }
}
