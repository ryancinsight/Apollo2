//! Device traits for arming operations
//!
//! This module defines traits that allow both real devices and mock devices
//! to be used interchangeably in arming operations, enabling comprehensive
//! testing without requiring actual hardware.

use crate::device::models::DeviceMode;
use crate::core::Result;

/// Trait for devices that can provide state information for arming operations
///
/// This trait abstracts the device state interface needed for arming operations,
/// allowing both real LumidoxDevice instances and mock devices to be used
/// interchangeably in testing and production code.
pub trait DeviceStateProvider {
    /// Get the current device mode
    ///
    /// # Returns
    /// * `Option<DeviceMode>` - Current device mode if known, None if unknown
    fn current_mode(&self) -> Option<DeviceMode>;
}

/// Trait for devices that can perform arming operations
///
/// This trait abstracts the arming interface needed for arming operations,
/// allowing both real LumidoxDevice instances and mock devices to be used
/// interchangeably in testing and production code.
pub trait ArmingCapable: DeviceStateProvider {
    /// Arm the device for firing operations
    ///
    /// # Returns
    /// * `Result<()>` - Success if arming succeeded, error if failed
    fn arm(&mut self) -> Result<()>;
}

// Implement DeviceStateProvider for LumidoxDevice
impl DeviceStateProvider for crate::device::LumidoxDevice {
    fn current_mode(&self) -> Option<DeviceMode> {
        self.current_mode()
    }
}

// Implement ArmingCapable for LumidoxDevice
impl ArmingCapable for crate::device::LumidoxDevice {
    fn arm(&mut self) -> Result<()> {
        self.arm()
    }
}

// Implement DeviceStateProvider for MockArmingDevice (test-only)
#[cfg(test)]
impl DeviceStateProvider for super::tests::mock_device::MockArmingDevice {
    fn current_mode(&self) -> Option<DeviceMode> {
        self.current_mode()
    }
}

// Implement ArmingCapable for MockArmingDevice (test-only)
#[cfg(test)]
impl ArmingCapable for super::tests::mock_device::MockArmingDevice {
    fn arm(&mut self) -> Result<()> {
        self.arm()
    }
}
