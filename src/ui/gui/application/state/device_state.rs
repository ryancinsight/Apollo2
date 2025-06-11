//! Device state management for Lumidox II Controller GUI
//!
//! This module provides device connection and status state management including
//! connected device tracking, current stage monitoring, ARM state management,
//! and parameter caching. It mirrors the CLI device state with proper state
//! synchronization and update mechanisms for optimal GUI responsiveness.
//!
//! The device state system provides:
//! - Device connection status and information tracking
//! - Current stage and ARM state monitoring
//! - Parameter caching and synchronization
//! - Device operation status tracking
//! - Proper state transitions and validation

use crate::device::{LumidoxDevice, models::{DeviceMode, DeviceInfo}};
use crate::core::{LumidoxError, Result};
use std::time::{Duration, Instant};

/// Device connection state
/// 
/// Tracks the current connection status and device information
/// for proper GUI state management and user feedback.
#[derive(Debug, Clone, PartialEq)]
pub enum ConnectionState {
    /// Device is disconnected
    Disconnected,
    /// Device is connecting
    Connecting,
    /// Device is connected and operational
    Connected,
    /// Device connection failed
    Failed(String),
    /// Device is reconnecting after failure
    Reconnecting,
}

impl Default for ConnectionState {
    fn default() -> Self {
        ConnectionState::Disconnected
    }
}

/// Device operation state
/// 
/// Tracks the current device operation status for proper
/// GUI feedback and state management.
#[derive(Debug, Clone, PartialEq)]
pub enum OperationState {
    /// Device is idle and ready for operations
    Idle,
    /// Device is performing an operation
    Busy(String),
    /// Last operation completed successfully
    Success(String),
    /// Last operation failed
    Failed(String),
}

impl Default for OperationState {
    fn default() -> Self {
        OperationState::Idle
    }
}

/// Cached device parameters
/// 
/// Stores frequently accessed device parameters to reduce
/// communication overhead and improve GUI responsiveness.
#[derive(Debug, Clone, Default)]
pub struct CachedParameters {
    /// ARM current setting (mA)
    pub arm_current: Option<u16>,
    /// FIRE current setting (mA)
    pub fire_current: Option<u16>,
    /// Maximum current limit (mA)
    pub max_current: Option<u16>,
    /// Device remote mode
    pub remote_mode: Option<DeviceMode>,
    /// Device information
    pub device_info: Option<DeviceInfo>,
    /// Last update timestamp
    pub last_update: Option<Instant>,
    /// Cache validity duration
    pub cache_duration: Duration,
}

impl CachedParameters {
    /// Create new cached parameters
    pub fn new() -> Self {
        Self {
            cache_duration: Duration::from_secs(5), // 5 second cache
            ..Default::default()
        }
    }
    
    /// Check if cache is valid
    pub fn is_cache_valid(&self) -> bool {
        if let Some(last_update) = self.last_update {
            last_update.elapsed() < self.cache_duration
        } else {
            false
        }
    }
    
    /// Update ARM current
    pub fn update_arm_current(&mut self, current: u16) {
        self.arm_current = Some(current);
        self.last_update = Some(Instant::now());
    }
    
    /// Update FIRE current
    pub fn update_fire_current(&mut self, current: u16) {
        self.fire_current = Some(current);
        self.last_update = Some(Instant::now());
    }
    
    /// Update remote mode
    pub fn update_remote_mode(&mut self, mode: DeviceMode) {
        self.remote_mode = Some(mode);
        self.last_update = Some(Instant::now());
    }
    
    /// Update device info
    pub fn update_device_info(&mut self, info: DeviceInfo) {
        self.device_info = Some(info);
        self.last_update = Some(Instant::now());
    }
    
    /// Clear cache
    pub fn clear_cache(&mut self) {
        self.arm_current = None;
        self.fire_current = None;
        self.max_current = None;
        self.remote_mode = None;
        self.device_info = None;
        self.last_update = None;
    }
    
    /// Invalidate cache
    pub fn invalidate(&mut self) {
        self.last_update = None;
    }
}

/// Device state management
/// 
/// Comprehensive device state management including connection status,
/// operation tracking, parameter caching, and state synchronization.
#[derive(Debug, Clone)]
pub struct DeviceState {
    /// Current connection state
    pub connection_state: ConnectionState,
    /// Current operation state
    pub operation_state: OperationState,
    /// Cached device parameters
    pub cached_parameters: CachedParameters,
    /// Port name for connection
    pub port_name: Option<String>,
    /// Auto-detection enabled
    pub auto_detect: bool,
    /// Verbose mode enabled
    pub verbose: bool,
    /// Optimize transitions enabled
    pub optimize_transitions: bool,
    /// Last error message
    pub last_error: Option<String>,
    /// Connection retry count
    pub retry_count: u32,
    /// Maximum retry attempts
    pub max_retries: u32,
}

impl Default for DeviceState {
    fn default() -> Self {
        Self {
            connection_state: ConnectionState::default(),
            operation_state: OperationState::default(),
            cached_parameters: CachedParameters::new(),
            port_name: None,
            auto_detect: false,
            verbose: false,
            optimize_transitions: true,
            last_error: None,
            retry_count: 0,
            max_retries: 3,
        }
    }
}

impl DeviceState {
    /// Create new device state
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Create device state with configuration
    pub fn with_config(
        port_name: Option<String>,
        auto_detect: bool,
        verbose: bool,
        optimize_transitions: bool,
    ) -> Self {
        Self {
            port_name,
            auto_detect,
            verbose,
            optimize_transitions,
            ..Self::default()
        }
    }
    
    /// Set connection state
    pub fn set_connection_state(&mut self, state: ConnectionState) {
        self.connection_state = state;
        
        // Clear cache when disconnected
        if matches!(self.connection_state, ConnectionState::Disconnected | ConnectionState::Failed(_)) {
            self.cached_parameters.clear_cache();
        }
    }
    
    /// Set operation state
    pub fn set_operation_state(&mut self, state: OperationState) {
        self.operation_state = state;
    }
    
    /// Check if device is connected
    pub fn is_connected(&self) -> bool {
        matches!(self.connection_state, ConnectionState::Connected)
    }
    
    /// Check if device is busy
    pub fn is_busy(&self) -> bool {
        matches!(self.operation_state, OperationState::Busy(_))
    }
    
    /// Set last error
    pub fn set_error(&mut self, error: String) {
        self.last_error = Some(error.clone());
        self.set_connection_state(ConnectionState::Failed(error));
    }
    
    /// Clear last error
    pub fn clear_error(&mut self) {
        self.last_error = None;
    }
    
    /// Increment retry count
    pub fn increment_retry(&mut self) -> bool {
        self.retry_count += 1;
        self.retry_count <= self.max_retries
    }
    
    /// Reset retry count
    pub fn reset_retry(&mut self) {
        self.retry_count = 0;
    }
    
    /// Update cached ARM current
    pub fn update_arm_current(&mut self, current: u16) {
        self.cached_parameters.update_arm_current(current);
    }
    
    /// Update cached FIRE current
    pub fn update_fire_current(&mut self, current: u16) {
        self.cached_parameters.update_fire_current(current);
    }
    
    /// Update cached remote mode
    pub fn update_remote_mode(&mut self, mode: DeviceMode) {
        self.cached_parameters.update_remote_mode(mode);
    }
    
    /// Update cached device info
    pub fn update_device_info(&mut self, info: DeviceInfo) {
        self.cached_parameters.update_device_info(info);
    }
    
    /// Get cached ARM current
    pub fn get_arm_current(&self) -> Option<u16> {
        if self.cached_parameters.is_cache_valid() {
            self.cached_parameters.arm_current
        } else {
            None
        }
    }
    
    /// Get cached FIRE current
    pub fn get_fire_current(&self) -> Option<u16> {
        if self.cached_parameters.is_cache_valid() {
            self.cached_parameters.fire_current
        } else {
            None
        }
    }
    
    /// Get cached remote mode
    pub fn get_remote_mode(&self) -> Option<DeviceMode> {
        if self.cached_parameters.is_cache_valid() {
            self.cached_parameters.remote_mode
        } else {
            None
        }
    }
    
    /// Get cached device info
    pub fn get_device_info(&self) -> Option<&DeviceInfo> {
        if self.cached_parameters.is_cache_valid() {
            self.cached_parameters.device_info.as_ref()
        } else {
            None
        }
    }
    
    /// Invalidate parameter cache
    pub fn invalidate_cache(&mut self) {
        self.cached_parameters.invalidate();
    }
    
    /// Get connection status description
    pub fn get_connection_description(&self) -> String {
        match &self.connection_state {
            ConnectionState::Disconnected => "Disconnected".to_string(),
            ConnectionState::Connecting => "Connecting...".to_string(),
            ConnectionState::Connected => "Connected".to_string(),
            ConnectionState::Failed(error) => format!("Failed: {}", error),
            ConnectionState::Reconnecting => "Reconnecting...".to_string(),
        }
    }
    
    /// Get operation status description
    pub fn get_operation_description(&self) -> String {
        match &self.operation_state {
            OperationState::Idle => "Ready".to_string(),
            OperationState::Busy(operation) => format!("Busy: {}", operation),
            OperationState::Success(operation) => format!("Success: {}", operation),
            OperationState::Failed(error) => format!("Failed: {}", error),
        }
    }
    
    /// Check if cache needs refresh
    pub fn needs_cache_refresh(&self) -> bool {
        !self.cached_parameters.is_cache_valid() && self.is_connected()
    }
}
