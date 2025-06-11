//! Mock device implementations for arming operation testing
//!
//! This module provides mock device implementations that allow testing
//! arming operations without requiring actual hardware. The mocks provide
//! predictable behavior for both success and failure scenarios.

use crate::core::Result;
use crate::device::models::DeviceMode;

/// Mock device for arming operation testing
/// 
/// Provides controllable behavior for testing different arming scenarios
/// without requiring actual hardware connections.
pub struct MockArmingDevice {
    pub current_mode: Option<DeviceMode>,
    pub should_fail_arm: bool,
    pub arm_call_count: u32,
}

impl MockArmingDevice {
    /// Create a new mock device in standby mode (ready for arming)
    pub fn new_ready_for_arming() -> Self {
        Self {
            current_mode: Some(DeviceMode::Standby),
            should_fail_arm: false,
            arm_call_count: 0,
        }
    }

    /// Create a new mock device in local mode (not ready for arming)
    pub fn new_in_local_mode() -> Self {
        Self {
            current_mode: Some(DeviceMode::Local),
            should_fail_arm: false,
            arm_call_count: 0,
        }
    }

    /// Create a new mock device already armed
    pub fn new_already_armed() -> Self {
        Self {
            current_mode: Some(DeviceMode::Armed),
            should_fail_arm: false,
            arm_call_count: 0,
        }
    }

    /// Create a new mock device with unknown mode
    pub fn new_unknown_mode() -> Self {
        Self {
            current_mode: None,
            should_fail_arm: false,
            arm_call_count: 0,
        }
    }

    /// Create a mock device that will fail arming operations
    pub fn new_failing_device() -> Self {
        Self {
            current_mode: Some(DeviceMode::Standby),
            should_fail_arm: true,
            arm_call_count: 0,
        }
    }

    /// Mock implementation of device arming
    pub fn arm(&mut self) -> Result<()> {
        self.arm_call_count += 1;
        
        if self.should_fail_arm {
            return Err(crate::core::LumidoxError::DeviceError(
                "Mock device arming failure".to_string()
            ));
        }

        // Simulate successful arming by changing mode
        self.current_mode = Some(DeviceMode::Armed);
        Ok(())
    }

    /// Mock implementation of current mode retrieval
    pub fn current_mode(&self) -> Option<DeviceMode> {
        self.current_mode
    }

    /// Reset the mock device state for reuse in tests
    pub fn reset(&mut self) {
        self.current_mode = Some(DeviceMode::Standby);
        self.should_fail_arm = false;
        self.arm_call_count = 0;
    }

    /// Set the device to fail on next arm operation
    pub fn set_arm_failure(&mut self, should_fail: bool) {
        self.should_fail_arm = should_fail;
    }

    /// Get the number of times arm() was called
    pub fn get_arm_call_count(&self) -> u32 {
        self.arm_call_count
    }
}

/// Helper function to create a mock device ready for arming
pub fn create_ready_mock_device() -> MockArmingDevice {
    MockArmingDevice::new_ready_for_arming()
}

/// Helper function to create a mock device that will fail arming
pub fn create_failing_mock_device() -> MockArmingDevice {
    MockArmingDevice::new_failing_device()
}

/// Helper function to create a mock device in local mode
pub fn create_local_mode_mock_device() -> MockArmingDevice {
    MockArmingDevice::new_in_local_mode()
}

/// Helper function to create a mock device already armed
pub fn create_already_armed_mock_device() -> MockArmingDevice {
    MockArmingDevice::new_already_armed()
}
