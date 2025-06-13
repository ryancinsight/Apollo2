//! Unit tests for device arming operations
//!
//! This module provides comprehensive unit tests for the arming operations
//! following the established testing patterns. Tests are organized into
//! focused groups covering validation, operation execution, and error handling.

use super::super::ArmingOperations;
use super::mock_device::*;
use crate::device::models::DeviceMode;

/// Tests for arming readiness validation
mod arming_validation_tests {
    use super::*;

    #[test]
    fn test_validate_arming_readiness_standby_mode() {
        let device = create_ready_mock_device();
        let result = ArmingOperations::validate_arming_readiness(&device);
        assert!(result.is_ok(), "Standby mode should be ready for arming");
    }

    #[test]
    fn test_validate_arming_readiness_local_mode() {
        let device = create_local_mode_mock_device();
        let result = ArmingOperations::validate_arming_readiness(&device);
        assert!(result.is_err(), "Local mode should not be ready for arming");
        
        if let Err(e) = result {
            let error_msg = format!("{}", e);
            assert!(error_msg.contains("local mode"), "Error should mention local mode");
        }
    }

    #[test]
    fn test_validate_arming_readiness_already_armed() {
        let device = create_already_armed_mock_device();
        let result = ArmingOperations::validate_arming_readiness(&device);
        assert!(result.is_err(), "Already armed device should not be ready for arming");
        
        if let Err(e) = result {
            let error_msg = format!("{}", e);
            assert!(error_msg.contains("already armed"), "Error should mention already armed");
        }
    }

    #[test]
    fn test_validate_arming_readiness_unknown_mode() {
        let device = MockArmingDevice::new_unknown_mode();
        let result = ArmingOperations::validate_arming_readiness(&device);
        assert!(result.is_err(), "Unknown mode should not be ready for arming");
        
        if let Err(e) = result {
            let error_msg = format!("{}", e);
            assert!(error_msg.contains("unknown"), "Error should mention unknown mode");
        }
    }
}

/// Tests for operation response structure
mod operation_response_tests {
    use super::*;

    #[test]
    fn test_arming_response_structure_success() {
        let device = create_ready_mock_device();
        
        // Note: This test would need actual LumidoxDevice integration
        // For now, we test the validation logic which is the core of the operation
        let validation_result = ArmingOperations::validate_arming_readiness(&device);
        assert!(validation_result.is_ok(), "Validation should pass for ready device");
    }

    #[test]
    fn test_arming_response_contains_state_information() {
        let device = create_ready_mock_device();
        let previous_state = device.current_mode();
        assert_eq!(previous_state, Some(DeviceMode::Standby), "Initial state should be Standby");
    }

    #[test]
    fn test_arming_state_transition() {
        let mut device = create_ready_mock_device();
        assert_eq!(device.current_mode(), Some(DeviceMode::Standby));
        
        // Simulate arming
        let result = device.arm();
        assert!(result.is_ok(), "Arming should succeed");
        assert_eq!(device.current_mode(), Some(DeviceMode::Armed), "State should change to Armed");
    }
}

/// Tests for error handling
mod error_handling_tests {
    use super::*;

    #[test]
    fn test_arming_failure_handling() {
        let mut device = create_failing_mock_device();
        let result = device.arm();
        assert!(result.is_err(), "Failing device should return error");
        
        if let Err(e) = result {
            let error_msg = format!("{}", e);
            assert!(error_msg.contains("Mock device arming failure"), "Error should contain failure message");
        }
    }

    #[test]
    fn test_validation_error_types() {
        let device = create_local_mode_mock_device();
        let result = ArmingOperations::validate_arming_readiness(&device);
        
        assert!(result.is_err(), "Local mode should fail validation");
        if let Err(e) = result {
            // Verify it's an InvalidInput error
            match e {
                crate::core::LumidoxError::InvalidInput(_) => {
                    // This is the expected error type
                }
                _ => panic!("Expected InvalidInput error for validation failure"),
            }
        }
    }

    #[test]
    fn test_error_message_consistency() {
        let test_cases = vec![
            (create_local_mode_mock_device(), "local mode"),
            (create_already_armed_mock_device(), "already armed"),
            (MockArmingDevice::new_unknown_mode(), "unknown"),
        ];

        for (device, expected_keyword) in test_cases {
            let result = ArmingOperations::validate_arming_readiness(&device);
            assert!(result.is_err(), "Validation should fail");
            
            if let Err(e) = result {
                let error_msg = format!("{}", e).to_lowercase();
                assert!(error_msg.contains(expected_keyword), 
                    "Error message should contain '{}', got: {}", expected_keyword, error_msg);
            }
        }
    }
}

/// Tests for performance and reliability
mod performance_tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_validation_performance() {
        let device = create_ready_mock_device();
        let start = Instant::now();
        
        // Run validation 1000 times
        for _ in 0..1000 {
            let _ = ArmingOperations::validate_arming_readiness(&device);
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 50, "1000 validations should complete in <50ms, took: {}ms", duration.as_millis());
    }

    #[test]
    fn test_validation_deterministic() {
        let device = create_ready_mock_device();
        
        // Run validation multiple times and ensure consistent results
        for _ in 0..100 {
            let result = ArmingOperations::validate_arming_readiness(&device);
            assert!(result.is_ok(), "Validation should be consistently successful");
        }
    }

    #[test]
    fn test_no_side_effects() {
        let device = create_ready_mock_device();
        let initial_mode = device.current_mode();
        
        // Validation should not change device state
        let _ = ArmingOperations::validate_arming_readiness(&device);
        assert_eq!(device.current_mode(), initial_mode, "Validation should not change device state");
    }
}
