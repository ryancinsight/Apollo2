//! Device state model definitions
//!
//! This module contains types and enums related to device operational state
//! including operating modes and state transitions.

/// Device operating modes
/// 
/// Represents the different operational states that the Lumidox II device
/// can be in. These modes control the device's behavior and determine
/// what operations are available.
/// 
/// The numeric values correspond to the protocol values sent to the device
/// via the SET_MODE command (0x15).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceMode {
    /// Local mode (device controlled locally) - 0x0000
    /// 
    /// In this mode, the device operates under local control and does not
    /// accept remote commands. This is the default power-on state.
    Local = 0,
    
    /// Standby mode (On, Output Off) - 0x0001
    /// 
    /// Device is powered on and ready to receive commands, but output
    /// is disabled. This is a safe state for configuration changes.
    Standby = 1,
    
    /// Armed mode (On, Arm) - 0x0002
    /// 
    /// Device is armed and ready for firing operations. Output is enabled
    /// and the device will respond to firing commands.
    Armed = 2,
    
    /// Remote firing mode (On, Fire) - 0x0003
    /// 
    /// Device is actively firing or has completed a firing sequence.
    /// This mode indicates active output operation.
    Remote = 3,
}
