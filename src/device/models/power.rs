//! Power measurement model definitions
//!
//! This module contains types and structures related to power measurements
//! and energy calculations for the Lumidox II device.

/// Power measurement data
/// 
/// Contains power measurement information for a specific stage or operation.
/// Power measurements are calculated based on current and voltage readings
/// from the device and provide insight into energy consumption and output.
/// 
/// The power values are provided in floating-point format with associated
/// unit strings to maintain precision and clarity in measurements.
#[derive(Debug, Clone)]
pub struct PowerInfo {
    /// Total power measurement value
    /// 
    /// The calculated total power for the measurement context.
    /// This represents the aggregate power consumption or output.
    pub total_power: f32,
    
    /// Units for the total power measurement
    /// 
    /// String representation of the units (e.g., "mW", "W", "µW").
    /// This provides context for interpreting the total_power value.
    pub total_units: String,
    
    /// Per-LED power measurement value
    /// 
    /// The calculated power per individual LED or output element.
    /// This provides granular insight into individual component power.
    pub per_power: f32,
    
    /// Units for the per-LED power measurement
    /// 
    /// String representation of the units (e.g., "mW", "W", "µW").
    /// This provides context for interpreting the per_power value.
    pub per_units: String,
}
