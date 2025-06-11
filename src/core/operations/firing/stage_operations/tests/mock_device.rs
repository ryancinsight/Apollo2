//! Mock device implementations for testing stage operations
//!
//! This module provides mock implementations of LumidoxDevice for isolated unit testing
//! without requiring real hardware connections or external dependencies.

use crate::core::{Result, LumidoxError};
use crate::device::LumidoxDevice;
use std::collections::HashMap;

/// Mock device for testing stage operations
///
/// This mock implementation allows for predictable testing scenarios by controlling
/// device responses and simulating various success/failure conditions.
pub struct MockLumidoxDevice {
    /// Simulated stage current values (stage -> current_ma)
    pub stage_currents: HashMap<u8, u16>,
    /// Whether fire_stage operations should succeed
    pub fire_stage_success: bool,
    /// Whether get_stage_arm_current operations should succeed
    pub get_current_success: bool,
    /// Custom error message for failures
    pub error_message: String,
    /// Track which stages have been fired
    pub fired_stages: Vec<u8>,
}

impl MockLumidoxDevice {
    /// Create a new mock device with default success behavior
    pub fn new_success() -> Self {
        let mut stage_currents = HashMap::new();
        stage_currents.insert(1, 50);
        stage_currents.insert(2, 75);
        stage_currents.insert(3, 100);
        stage_currents.insert(4, 125);
        stage_currents.insert(5, 150);

        Self {
            stage_currents,
            fire_stage_success: true,
            get_current_success: true,
            error_message: "Mock error".to_string(),
            fired_stages: Vec::new(),
        }
    }

    /// Create a mock device that fails fire_stage operations
    pub fn new_fire_failure() -> Self {
        let mut device = Self::new_success();
        device.fire_stage_success = false;
        device.error_message = "Mock fire stage failure".to_string();
        device
    }

    /// Create a mock device that fails get_stage_arm_current operations
    pub fn new_current_failure() -> Self {
        let mut device = Self::new_success();
        device.get_current_success = false;
        device.error_message = "Mock get current failure".to_string();
        device
    }

    /// Create a mock device with custom stage currents
    pub fn with_stage_currents(currents: HashMap<u8, u16>) -> Self {
        Self {
            stage_currents: currents,
            fire_stage_success: true,
            get_current_success: true,
            error_message: "Mock error".to_string(),
            fired_stages: Vec::new(),
        }
    }

    /// Check if a stage has been fired
    pub fn was_stage_fired(&self, stage: u8) -> bool {
        self.fired_stages.contains(&stage)
    }

    /// Get the number of stages fired
    pub fn fired_stage_count(&self) -> usize {
        self.fired_stages.len()
    }
}

impl LumidoxDevice {
    /// Create a mock device for testing (test-only constructor)
    #[cfg(test)]
    pub fn mock_success() -> MockLumidoxDevice {
        MockLumidoxDevice::new_success()
    }

    /// Create a mock device that fails fire operations (test-only constructor)
    #[cfg(test)]
    pub fn mock_fire_failure() -> MockLumidoxDevice {
        MockLumidoxDevice::new_fire_failure()
    }

    /// Create a mock device that fails current operations (test-only constructor)
    #[cfg(test)]
    pub fn mock_current_failure() -> MockLumidoxDevice {
        MockLumidoxDevice::new_current_failure()
    }
}

// Note: Since LumidoxDevice is a concrete struct, we need to implement the mock behavior
// through trait methods or by extending the existing implementation for testing.
// For now, we'll create helper functions that can be used in tests.

/// Helper function to simulate fire_stage operation for testing
pub fn mock_fire_stage(mock: &mut MockLumidoxDevice, stage: u8) -> Result<()> {
    if mock.fire_stage_success {
        mock.fired_stages.push(stage);
        Ok(())
    } else {
        Err(LumidoxError::DeviceError(mock.error_message.clone()))
    }
}

/// Helper function to simulate get_stage_arm_current operation for testing
pub fn mock_get_stage_arm_current(mock: &MockLumidoxDevice, stage: u8) -> Result<u16> {
    if mock.get_current_success {
        mock.stage_currents.get(&stage)
            .copied()
            .ok_or_else(|| LumidoxError::DeviceError(format!("No current data for stage {}", stage)))
    } else {
        Err(LumidoxError::DeviceError(mock.error_message.clone()))
    }
}
