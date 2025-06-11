//! Error scenario tests for device arming operations
//!
//! This module provides comprehensive error testing for arming operations,
//! covering edge cases, boundary conditions, and failure scenarios.

use super::super::ArmingOperations;
use super::mock_device::*;
use crate::core::LumidoxError;
use crate::device::models::DeviceMode;

/// Tests for validation error scenarios
mod validation_error_tests {
    use super::*;

    #[test]
    fn test_local_mode_validation_error() {
        let device = create_local_mode_mock_device();
        let result = ArmingOperations::validate_arming_readiness(&device);
        
        assert!(result.is_err(), "Local mode should fail validation");
        
        if let Err(LumidoxError::InvalidInput(msg)) = result {
            assert!(msg.contains("local mode"), "Error should mention local mode");
            assert!(msg.contains("Cannot arm device remotely"), "Error should explain the issue");
        } else {
            panic!("Expected InvalidInput error for local mode");
        }
    }

    #[test]
    fn test_already_armed_validation_error() {
        let device = create_already_armed_mock_device();
        let result = ArmingOperations::validate_arming_readiness(&device);
        
        assert!(result.is_err(), "Already armed device should fail validation");
        
        if let Err(LumidoxError::InvalidInput(msg)) = result {
            assert!(msg.contains("already armed"), "Error should mention already armed");
        } else {
            panic!("Expected InvalidInput error for already armed device");
        }
    }

    #[test]
    fn test_remote_mode_validation_error() {
        let mut device = MockArmingDevice::new_ready_for_arming();
        device.current_mode = Some(DeviceMode::Remote);
        
        let result = ArmingOperations::validate_arming_readiness(&device);
        
        assert!(result.is_err(), "Remote mode should fail validation");
        
        if let Err(LumidoxError::InvalidInput(msg)) = result {
            assert!(msg.contains("remote mode"), "Error should mention remote mode");
            assert!(msg.contains("Turn off device before arming"), "Error should provide guidance");
        } else {
            panic!("Expected InvalidInput error for remote mode");
        }
    }

    #[test]
    fn test_unknown_mode_validation_error() {
        let device = MockArmingDevice::new_unknown_mode();
        let result = ArmingOperations::validate_arming_readiness(&device);
        
        assert!(result.is_err(), "Unknown mode should fail validation");
        
        if let Err(LumidoxError::InvalidInput(msg)) = result {
            assert!(msg.contains("unknown"), "Error should mention unknown mode");
            assert!(msg.contains("Cannot determine arming readiness"), "Error should explain the issue");
        } else {
            panic!("Expected InvalidInput error for unknown mode");
        }
    }
}

/// Tests for device operation error scenarios
mod device_error_tests {
    use super::*;

    #[test]
    fn test_device_arming_failure() {
        let mut device = create_failing_mock_device();
        
        // Validation should pass
        let validation_result = ArmingOperations::validate_arming_readiness(&device);
        assert!(validation_result.is_ok(), "Validation should pass for failing device");
        
        // But arming should fail
        let arm_result = device.arm();
        assert!(arm_result.is_err(), "Device arming should fail");
        
        if let Err(LumidoxError::DeviceError(msg)) = arm_result {
            assert!(msg.contains("Mock device arming failure"), "Error should contain failure message");
        } else {
            panic!("Expected DeviceError for arming failure");
        }
    }

    #[test]
    fn test_device_state_after_failure() {
        let mut device = create_failing_mock_device();
        let initial_state = device.current_mode();
        
        // Attempt arming (should fail)
        let _ = device.arm();
        
        // State should remain unchanged after failure
        assert_eq!(device.current_mode(), initial_state, "State should not change on failure");
    }

    #[test]
    fn test_multiple_failure_attempts() {
        let mut device = create_failing_mock_device();
        
        // Multiple failure attempts should be consistent
        for i in 0..5 {
            let result = device.arm();
            assert!(result.is_err(), "Attempt {} should fail", i + 1);
            
            if let Err(LumidoxError::DeviceError(msg)) = result {
                assert!(msg.contains("Mock device arming failure"), 
                    "Error message should be consistent on attempt {}", i + 1);
            }
        }
        
        assert_eq!(device.get_arm_call_count(), 5, "All attempts should be counted");
    }
}

/// Tests for edge cases and boundary conditions
mod edge_case_tests {
    use super::*;

    #[test]
    fn test_rapid_validation_calls() {
        let device = create_ready_mock_device();
        
        // Rapid validation calls should not cause issues
        for _ in 0..1000 {
            let result = ArmingOperations::validate_arming_readiness(&device);
            assert!(result.is_ok(), "Rapid validation calls should succeed");
        }
    }

    #[test]
    fn test_validation_after_state_change() {
        let mut device = create_ready_mock_device();
        
        // Initial validation should pass
        let result1 = ArmingOperations::validate_arming_readiness(&device);
        assert!(result1.is_ok(), "Initial validation should pass");
        
        // Change state to armed
        let _ = device.arm();
        
        // Validation should now fail
        let result2 = ArmingOperations::validate_arming_readiness(&device);
        assert!(result2.is_err(), "Validation should fail after arming");
    }

    #[test]
    fn test_device_reset_and_revalidation() {
        let mut device = create_ready_mock_device();
        
        // Arm the device
        let _ = device.arm();
        assert_eq!(device.current_mode(), Some(DeviceMode::Armed));
        
        // Reset device
        device.reset();
        assert_eq!(device.current_mode(), Some(DeviceMode::Standby));
        assert_eq!(device.get_arm_call_count(), 0);
        
        // Validation should pass again
        let result = ArmingOperations::validate_arming_readiness(&device);
        assert!(result.is_ok(), "Validation should pass after reset");
    }
}

/// Tests for error message quality and consistency
mod error_message_tests {
    use super::*;

    #[test]
    fn test_error_message_descriptiveness() {
        let test_cases = vec![
            (create_local_mode_mock_device(), "local mode", "remotely"),
            (create_already_armed_mock_device(), "already armed", "armed"),
            (MockArmingDevice::new_unknown_mode(), "unknown", "determine"),
        ];

        for (device, primary_keyword, secondary_keyword) in test_cases {
            let result = ArmingOperations::validate_arming_readiness(&device);
            assert!(result.is_err(), "Validation should fail");
            
            if let Err(e) = result {
                let error_msg = format!("{}", e).to_lowercase();
                assert!(error_msg.contains(primary_keyword), 
                    "Error should contain primary keyword '{}', got: {}", primary_keyword, error_msg);
                assert!(error_msg.contains(secondary_keyword), 
                    "Error should contain secondary keyword '{}', got: {}", secondary_keyword, error_msg);
            }
        }
    }

    #[test]
    fn test_error_message_actionability() {
        let device = MockArmingDevice::new_in_local_mode();
        let result = ArmingOperations::validate_arming_readiness(&device);
        
        if let Err(e) = result {
            let error_msg = format!("{}", e);
            // Error should provide actionable guidance
            assert!(error_msg.contains("Cannot arm device remotely") || 
                   error_msg.contains("local mode"), 
                "Error should explain why arming failed");
        }
    }

    #[test]
    fn test_error_message_consistency_across_calls() {
        let device = create_local_mode_mock_device();
        
        let mut error_messages = Vec::new();
        for _ in 0..10 {
            if let Err(e) = ArmingOperations::validate_arming_readiness(&device) {
                error_messages.push(format!("{}", e));
            }
        }
        
        // All error messages should be identical
        assert_eq!(error_messages.len(), 10, "All calls should return errors");
        for (i, msg) in error_messages.iter().enumerate() {
            assert_eq!(msg, &error_messages[0], 
                "Error message {} should match the first message", i + 1);
        }
    }
}

/// Tests for error recovery and resilience
mod error_recovery_tests {
    use super::*;

    #[test]
    fn test_recovery_from_validation_failure() {
        let mut device = create_local_mode_mock_device();
        
        // Initial validation should fail
        let result1 = ArmingOperations::validate_arming_readiness(&device);
        assert!(result1.is_err(), "Local mode validation should fail");
        
        // Change to valid state
        device.current_mode = Some(DeviceMode::Standby);
        
        // Validation should now pass
        let result2 = ArmingOperations::validate_arming_readiness(&device);
        assert!(result2.is_ok(), "Standby mode validation should pass");
    }

    #[test]
    fn test_recovery_from_device_failure() {
        let mut device = create_failing_mock_device();
        
        // Initial arming should fail
        let result1 = device.arm();
        assert!(result1.is_err(), "Initial arming should fail");
        
        // Fix the device
        device.set_arm_failure(false);
        
        // Arming should now succeed
        let result2 = device.arm();
        assert!(result2.is_ok(), "Arming should succeed after fix");
        assert_eq!(device.current_mode(), Some(DeviceMode::Armed));
    }

    #[test]
    fn test_error_state_isolation() {
        let device1 = create_local_mode_mock_device();
        let device2 = create_ready_mock_device();
        
        // Error in device1 should not affect device2
        let result1 = ArmingOperations::validate_arming_readiness(&device1);
        assert!(result1.is_err(), "Device1 validation should fail");
        
        let result2 = ArmingOperations::validate_arming_readiness(&device2);
        assert!(result2.is_ok(), "Device2 validation should pass");
    }
}
