//! Result types for unified operations
//!
//! This module defines interface-independent result types that can be used by both
//! CLI and GUI interfaces. These types contain structured data that each interface
//! can format and present according to its own requirements.

use crate::core::LumidoxError;

/// Unified operation result type
pub type OperationResult<T> = std::result::Result<OperationResponse<T>, LumidoxError>;

/// Interface-independent operation response
#[derive(Debug, Clone)]
pub struct OperationResponse<T> {
    /// The operation data payload
    pub data: T,
    /// Human-readable success message
    pub message: String,
    /// Operation metadata
    pub metadata: OperationMetadata,
}

/// Operation metadata for tracking and logging
#[derive(Debug, Clone)]
pub struct OperationMetadata {
    /// Operation type identifier
    pub operation_type: String,
    /// Timestamp of operation completion
    pub timestamp: std::time::SystemTime,
    /// Operation duration in milliseconds
    pub duration_ms: Option<u64>,
    /// Additional context information
    pub context: std::collections::HashMap<String, String>,
}

/// Device operation data types
#[derive(Debug, Clone)]
pub enum DeviceOperationData {
    /// Device control operation result
    DeviceControl {
        /// Previous device state
        previous_state: Option<String>,
        /// New device state
        new_state: Option<String>,
        /// Operation success flag
        success: bool,
    },
    /// Stage firing operation result
    StageFiring {
        /// Stage number that was fired
        stage: u8,
        /// Current used for firing (if applicable)
        current_ma: Option<u16>,
        /// Success flag
        success: bool,
    },
    /// Custom current firing result
    CurrentFiring {
        /// Current value used
        current_ma: u16,
        /// Success flag
        success: bool,
    },
    /// Device status information
    StatusInfo {
        /// Device information string
        device_info: String,
        /// Connection status
        connected: bool,
        /// Current device mode
        mode: Option<String>,
    },
    /// Device status information (unified)
    DeviceStatus {
        /// Current device mode
        current_mode: Option<String>,
        /// ARM current setting in mA
        arm_current: Option<u16>,
        /// FIRE current setting in mA
        fire_current: Option<u16>,
        /// Remote mode state
        remote_mode_state: Option<u16>,
        /// Connection health status
        connection_healthy: bool,
        /// Device readiness for operations
        ready_for_operations: bool,
    },
    /// Parameter information
    ParameterInfo {
        /// Parameter name
        parameter_name: String,
        /// Parameter value
        value: Option<String>,
        /// Parameter units (if applicable)
        units: Option<String>,
        /// Whether parameter is within valid range
        valid_range: bool,
        /// Additional parameter metadata
        metadata: Option<String>,
    },
    /// Stage information
    StageInfo {
        /// Stage number (1-5)
        stage_number: u8,
        /// Stage current in mA
        current_ma: Option<u16>,
        /// Stage voltage (if available)
        voltage_v: Option<f32>,
        /// Power information
        power_info: Option<String>,
        /// Stage readiness for firing
        ready_for_firing: bool,
    },
    /// Connection operation result
    Connection {
        /// Connection success flag
        connected: bool,
        /// Port name used
        port_name: Option<String>,
        /// Device information if connected
        device_info: Option<String>,
    },
    /// Power measurement operation results
    PowerMeasurement {
        /// Stage number measured
        stage_number: u8,
        /// Comprehensive power measurement data
        power_data: crate::core::operations::power::PowerMeasurementData,
        /// Validation result for the measurement
        validation_result: crate::core::operations::power::PowerValidationResult,
    },
    /// All stages power measurement results
    AllStagesPower {
        /// Power measurement data for all stages
        stages_data: Vec<crate::core::operations::power::PowerMeasurementData>,
        /// Target unit for conversion (if applied)
        target_unit: Option<crate::core::operations::power::PowerUnit>,
        /// Timestamp when measurements were taken
        measurement_timestamp: std::time::Instant,
    },
}

impl<T> OperationResponse<T> {
    /// Create a new successful operation response
    pub fn success(data: T, message: String, operation_type: String) -> Self {
        Self {
            data,
            message,
            metadata: OperationMetadata {
                operation_type,
                timestamp: std::time::SystemTime::now(),
                duration_ms: None,
                context: std::collections::HashMap::new(),
            },
        }
    }

    /// Create a new operation response with duration
    pub fn success_with_duration(
        data: T,
        message: String,
        operation_type: String,
        duration_ms: u64,
    ) -> Self {
        Self {
            data,
            message,
            metadata: OperationMetadata {
                operation_type,
                timestamp: std::time::SystemTime::now(),
                duration_ms: Some(duration_ms),
                context: std::collections::HashMap::new(),
            },
        }
    }

    /// Add context information to the operation response
    pub fn with_context(mut self, key: String, value: String) -> Self {
        self.metadata.context.insert(key, value);
        self
    }
}

impl OperationMetadata {
    /// Create new operation metadata
    pub fn new(operation_type: String) -> Self {
        Self {
            operation_type,
            timestamp: std::time::SystemTime::now(),
            duration_ms: None,
            context: std::collections::HashMap::new(),
        }
    }
}
