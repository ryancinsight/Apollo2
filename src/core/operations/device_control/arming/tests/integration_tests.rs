//! Integration tests for device arming operations
//!
//! This module provides integration tests that verify the complete arming
//! operation flow without requiring actual hardware. Tests focus on the
//! interaction between validation, execution, and response generation.

use super::super::ArmingOperations;
use super::mock_device::*;
use crate::core::operations::result_types::DeviceOperationData;
use crate::device::models::DeviceMode;

/// Tests for complete arming operation flow
mod arming_flow_tests {
    use super::*;

    #[test]
    fn test_complete_arming_flow_success() {
        let mut device = create_ready_mock_device();
        
        // Verify initial state
        assert_eq!(device.current_mode(), Some(DeviceMode::Standby));
        
        // Validate readiness
        let validation_result = ArmingOperations::validate_arming_readiness(&device);
        assert!(validation_result.is_ok(), "Device should be ready for arming");
        
        // Perform arming
        let arm_result = device.arm();
        assert!(arm_result.is_ok(), "Arming should succeed");
        
        // Verify final state
        assert_eq!(device.current_mode(), Some(DeviceMode::Armed));
        assert_eq!(device.get_arm_call_count(), 1, "Arm should be called exactly once");
    }

    #[test]
    fn test_complete_arming_flow_validation_failure() {
        let device = create_local_mode_mock_device();
        
        // Verify initial state
        assert_eq!(device.current_mode(), Some(DeviceMode::Local));
        
        // Validation should fail
        let validation_result = ArmingOperations::validate_arming_readiness(&device);
        assert!(validation_result.is_err(), "Local mode device should fail validation");
        
        // Should not proceed to arming when validation fails
        // (In real implementation, unified function would check validation first)
    }

    #[test]
    fn test_complete_arming_flow_device_failure() {
        let mut device = create_failing_mock_device();
        
        // Verify initial state (ready for arming)
        assert_eq!(device.current_mode(), Some(DeviceMode::Standby));
        
        // Validation should pass
        let validation_result = ArmingOperations::validate_arming_readiness(&device);
        assert!(validation_result.is_ok(), "Device should pass validation");
        
        // But arming should fail
        let arm_result = device.arm();
        assert!(arm_result.is_err(), "Device arming should fail");
        
        // State should remain unchanged on failure
        assert_eq!(device.current_mode(), Some(DeviceMode::Standby));
        assert_eq!(device.get_arm_call_count(), 1, "Arm should be attempted once");
    }
}

/// Tests for state transition validation
mod state_transition_tests {
    use super::*;

    #[test]
    fn test_state_transition_standby_to_armed() {
        let mut device = create_ready_mock_device();
        
        let initial_state = device.current_mode();
        assert_eq!(initial_state, Some(DeviceMode::Standby));
        
        let result = device.arm();
        assert!(result.is_ok(), "Standby to Armed transition should succeed");
        
        let final_state = device.current_mode();
        assert_eq!(final_state, Some(DeviceMode::Armed));
    }

    #[test]
    fn test_invalid_state_transitions() {
        let test_cases = vec![
            (create_local_mode_mock_device(), DeviceMode::Local, "Local mode"),
            (create_already_armed_mock_device(), DeviceMode::Armed, "Already armed"),
        ];

        for (device, expected_mode, description) in test_cases {
            assert_eq!(device.current_mode(), Some(expected_mode));
            
            let validation_result = ArmingOperations::validate_arming_readiness(&device);
            assert!(validation_result.is_err(), "{} should fail validation", description);
        }
    }

    #[test]
    fn test_unknown_state_handling() {
        let device = MockArmingDevice::new_unknown_mode();
        
        assert_eq!(device.current_mode(), None);
        
        let validation_result = ArmingOperations::validate_arming_readiness(&device);
        assert!(validation_result.is_err(), "Unknown state should fail validation");
    }
}

/// Tests for operation consistency and reliability
mod consistency_tests {
    use super::*;

    #[test]
    fn test_multiple_validation_calls_consistent() {
        let device = create_ready_mock_device();
        
        // Multiple validation calls should return consistent results
        for i in 0..10 {
            let result = ArmingOperations::validate_arming_readiness(&device);
            assert!(result.is_ok(), "Validation call {} should succeed", i + 1);
        }
    }

    #[test]
    fn test_validation_does_not_modify_state() {
        let mut device = create_ready_mock_device();
        let initial_state = device.current_mode();
        let initial_call_count = device.get_arm_call_count();
        
        // Validation should not modify device state
        let _ = ArmingOperations::validate_arming_readiness(&device);
        
        assert_eq!(device.current_mode(), initial_state, "Validation should not change device mode");
        assert_eq!(device.get_arm_call_count(), initial_call_count, "Validation should not call arm()");
    }

    #[test]
    fn test_arming_operation_idempotency() {
        let mut device = create_ready_mock_device();
        
        // First arming should succeed
        let result1 = device.arm();
        assert!(result1.is_ok(), "First arming should succeed");
        assert_eq!(device.current_mode(), Some(DeviceMode::Armed));
        
        // Validation should now fail for already armed device
        let validation_result = ArmingOperations::validate_arming_readiness(&device);
        assert!(validation_result.is_err(), "Already armed device should fail validation");
    }
}

/// Tests for error propagation and handling
mod error_propagation_tests {
    use super::*;

    #[test]
    fn test_validation_error_propagation() {
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
                    "Error should contain '{}', got: {}", expected_keyword, error_msg);
            }
        }
    }

    #[test]
    fn test_device_error_propagation() {
        let mut device = create_failing_mock_device();
        
        let result = device.arm();
        assert!(result.is_err(), "Failing device should return error");
        
        if let Err(e) = result {
            let error_msg = format!("{}", e);
            assert!(error_msg.contains("Mock device arming failure"), 
                "Error should contain device failure message");
        }
    }

    #[test]
    fn test_error_type_consistency() {
        let device = create_local_mode_mock_device();
        let result = ArmingOperations::validate_arming_readiness(&device);
        
        assert!(result.is_err(), "Validation should fail");
        if let Err(e) = result {
            match e {
                crate::core::LumidoxError::InvalidInput(_) => {
                    // This is the expected error type for validation failures
                }
                _ => panic!("Expected InvalidInput error for validation failure, got: {:?}", e),
            }
        }
    }
}
