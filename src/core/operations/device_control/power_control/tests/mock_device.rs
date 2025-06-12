//! Mock device implementations for power control operation testing
//!
//! This module provides mock device implementations that allow testing
//! power control operations without requiring actual hardware. The mocks provide
//! predictable behavior for both success and failure scenarios.

use crate::core::Result;
use crate::device::models::DeviceMode;

/// Mock device for power control operation testing
/// 
/// Provides controllable behavior for testing different power control scenarios
/// without requiring actual hardware connections.
pub struct MockPowerControlDevice {
    pub current_mode: Option<DeviceMode>,
    pub should_fail_turn_off: bool,
    pub turn_off_call_count: u32,
}

impl MockPowerControlDevice {
    /// Create a new mock device in armed mode (ready for turn off)
    pub fn new_ready_for_turn_off() -> Self {
        Self {
            current_mode: Some(DeviceMode::Armed),
            should_fail_turn_off: false,
            turn_off_call_count: 0,
        }
    }

    /// Create a new mock device in standby mode
    pub fn new_in_standby_mode() -> Self {
        Self {
            current_mode: Some(DeviceMode::Standby),
            should_fail_turn_off: false,
            turn_off_call_count: 0,
        }
    }

    /// Create a new mock device in local mode
    pub fn new_in_local_mode() -> Self {
        Self {
            current_mode: Some(DeviceMode::Local),
            should_fail_turn_off: false,
            turn_off_call_count: 0,
        }
    }

    /// Create a new mock device with unknown mode
    pub fn new_unknown_mode() -> Self {
        Self {
            current_mode: None,
            should_fail_turn_off: false,
            turn_off_call_count: 0,
        }
    }

    /// Create a mock device that will fail turn off operations
    pub fn new_failing_device() -> Self {
        Self {
            current_mode: Some(DeviceMode::Armed),
            should_fail_turn_off: true,
            turn_off_call_count: 0,
        }
    }

    /// Mock implementation of device turn off
    pub fn turn_off(&mut self) -> Result<()> {
        self.turn_off_call_count += 1;
        
        if self.should_fail_turn_off {
            return Err(crate::core::LumidoxError::DeviceError(
                "Mock device turn off failure".to_string()
            ));
        }

        // Simulate successful turn off by changing mode to standby
        self.current_mode = Some(DeviceMode::Standby);
        Ok(())
    }

    /// Mock implementation of current mode retrieval
    pub fn current_mode(&self) -> Option<DeviceMode> {
        self.current_mode
    }

    /// Reset the mock device state for reuse in tests
    pub fn reset(&mut self) {
        self.current_mode = Some(DeviceMode::Armed);
        self.should_fail_turn_off = false;
        self.turn_off_call_count = 0;
    }

    /// Set the device to fail on next turn off operation
    pub fn set_turn_off_failure(&mut self, should_fail: bool) {
        self.should_fail_turn_off = should_fail;
    }

    /// Get the number of times turn_off() was called
    pub fn get_turn_off_call_count(&self) -> u32 {
        self.turn_off_call_count
    }
}

/// Helper function to create a mock device ready for turn off
pub fn create_ready_mock_device() -> MockPowerControlDevice {
    MockPowerControlDevice::new_ready_for_turn_off()
}

/// Helper function to create a mock device that will fail turn off
pub fn create_failing_mock_device() -> MockPowerControlDevice {
    MockPowerControlDevice::new_failing_device()
}

/// Helper function to create a mock device in standby mode
pub fn create_standby_mode_mock_device() -> MockPowerControlDevice {
    MockPowerControlDevice::new_in_standby_mode()
}

/// Helper function to create a mock device in local mode
pub fn create_local_mode_mock_device() -> MockPowerControlDevice {
    MockPowerControlDevice::new_in_local_mode()
}
