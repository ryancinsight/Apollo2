//! Error scenario tests for stage operations
//!
//! This module tests various error conditions and edge cases for stage operations,
//! ensuring robust error handling and proper error propagation.

use super::super::StageOperations;
use crate::core::LumidoxError;

#[cfg(test)]
mod validation_error_tests {
    use super::*;

    #[test]
    fn test_stage_zero_validation_error() {
        let result = StageOperations::validate_stage_number(0);
        
        assert!(result.is_err(), "Stage 0 should fail validation");
        
        match result {
            Err(LumidoxError::InvalidInput(msg)) => {
                assert!(msg.contains("Invalid stage number: 0"));
                assert!(msg.contains("Must be 1-5"));
            }
            _ => panic!("Expected InvalidInput error for stage 0"),
        }
    }

    #[test]
    fn test_stage_six_validation_error() {
        let result = StageOperations::validate_stage_number(6);
        
        assert!(result.is_err(), "Stage 6 should fail validation");
        
        match result {
            Err(LumidoxError::InvalidInput(msg)) => {
                assert!(msg.contains("Invalid stage number: 6"));
                assert!(msg.contains("Must be 1-5"));
            }
            _ => panic!("Expected InvalidInput error for stage 6"),
        }
    }

    #[test]
    fn test_extreme_stage_values() {
        let extreme_values = [u8::MIN, u8::MAX, 100, 200];
        
        for &stage in &extreme_values {
            let result = StageOperations::validate_stage_number(stage);
            
            if (1..=5).contains(&stage) {
                assert!(result.is_ok(), "Stage {} should be valid", stage);
            } else {
                assert!(result.is_err(), "Stage {} should be invalid", stage);
                
                match result {
                    Err(LumidoxError::InvalidInput(msg)) => {
                        assert!(msg.contains(&format!("Invalid stage number: {}", stage)));
                        assert!(msg.contains("Must be 1-5"));
                    }
                    _ => panic!("Expected InvalidInput error for stage {}", stage),
                }
            }
        }
    }
}

#[cfg(test)]
mod error_message_tests {
    use super::*;

    #[test]
    fn test_error_message_contains_stage_number() {
        let invalid_stages = [0, 6, 7, 10, 255];
        
        for &stage in &invalid_stages {
            let result = StageOperations::validate_stage_number(stage);
            
            match result {
                Err(LumidoxError::InvalidInput(msg)) => {
                    assert!(
                        msg.contains(&stage.to_string()),
                        "Error message should contain stage number {} in message: {}",
                        stage,
                        msg
                    );
                }
                _ => panic!("Expected InvalidInput error for stage {}", stage),
            }
        }
    }

    #[test]
    fn test_error_message_provides_valid_range() {
        let result = StageOperations::validate_stage_number(10);
        
        match result {
            Err(LumidoxError::InvalidInput(msg)) => {
                assert!(
                    msg.contains("1-5") || (msg.contains("1") && msg.contains("5")),
                    "Error message should indicate valid range: {}",
                    msg
                );
            }
            _ => panic!("Expected InvalidInput error"),
        }
    }

    #[test]
    fn test_error_message_is_actionable() {
        let result = StageOperations::validate_stage_number(0);
        
        match result {
            Err(LumidoxError::InvalidInput(msg)) => {
                // Error message should be actionable (tell user what to do)
                assert!(
                    msg.to_lowercase().contains("must") || msg.to_lowercase().contains("should"),
                    "Error message should be actionable: {}",
                    msg
                );
            }
            _ => panic!("Expected InvalidInput error"),
        }
    }

    #[test]
    fn test_error_message_length_reasonable() {
        let result = StageOperations::validate_stage_number(99);
        
        match result {
            Err(LumidoxError::InvalidInput(msg)) => {
                assert!(
                    msg.len() >= 10 && msg.len() <= 200,
                    "Error message should be reasonable length (10-200 chars): {} chars",
                    msg.len()
                );
            }
            _ => panic!("Expected InvalidInput error"),
        }
    }
}

#[cfg(test)]
mod error_type_tests {
    use super::*;

    #[test]
    fn test_validation_returns_correct_error_type() {
        let result = StageOperations::validate_stage_number(0);
        
        match result {
            Err(LumidoxError::InvalidInput(_)) => {
                // This is the expected error type
            }
            Err(other_error) => {
                panic!("Expected InvalidInput error, got: {:?}", other_error);
            }
            Ok(_) => {
                panic!("Expected error for invalid stage 0");
            }
        }
    }

    #[test]
    fn test_validation_error_is_not_device_error() {
        let result = StageOperations::validate_stage_number(10);
        
        match result {
            Err(LumidoxError::DeviceError(_)) => {
                panic!("Validation errors should not be DeviceError type");
            }
            Err(LumidoxError::InvalidInput(_)) => {
                // This is correct
            }
            Err(other_error) => {
                panic!("Unexpected error type: {:?}", other_error);
            }
            Ok(_) => {
                panic!("Expected error for invalid stage 10");
            }
        }
    }

    #[test]
    fn test_validation_error_is_not_protocol_error() {
        let result = StageOperations::validate_stage_number(255);

        match result {
            Err(LumidoxError::ProtocolError(_)) => {
                panic!("Validation errors should not be ProtocolError type");
            }
            Err(LumidoxError::InvalidInput(_)) => {
                // This is correct
            }
            Err(other_error) => {
                panic!("Unexpected error type: {:?}", other_error);
            }
            Ok(_) => {
                panic!("Expected error for invalid stage 255");
            }
        }
    }
}

#[cfg(test)]
mod edge_case_tests {
    use super::*;

    #[test]
    fn test_boundary_conditions() {
        // Test exact boundaries
        assert!(StageOperations::validate_stage_number(1).is_ok(), "Stage 1 should be valid (lower boundary)");
        assert!(StageOperations::validate_stage_number(5).is_ok(), "Stage 5 should be valid (upper boundary)");
        assert!(StageOperations::validate_stage_number(0).is_err(), "Stage 0 should be invalid (below lower boundary)");
        assert!(StageOperations::validate_stage_number(6).is_err(), "Stage 6 should be invalid (above upper boundary)");
    }

    #[test]
    fn test_all_invalid_single_digit_stages() {
        let invalid_single_digits = [0, 6, 7, 8, 9];
        
        for &stage in &invalid_single_digits {
            let result = StageOperations::validate_stage_number(stage);
            assert!(result.is_err(), "Single digit stage {} should be invalid", stage);
        }
    }

    #[test]
    fn test_all_valid_stages_individually() {
        let valid_stages = [1, 2, 3, 4, 5];
        
        for &stage in &valid_stages {
            let result = StageOperations::validate_stage_number(stage);
            assert!(result.is_ok(), "Stage {} should be valid", stage);
        }
    }

    #[test]
    fn test_validation_consistency_across_calls() {
        // Test that multiple calls with same input produce same result
        for stage in 0..=10 {
            let result1 = StageOperations::validate_stage_number(stage);
            let result2 = StageOperations::validate_stage_number(stage);
            let result3 = StageOperations::validate_stage_number(stage);
            
            // All results should be the same type (Ok or Err)
            match (result1.is_ok(), result2.is_ok(), result3.is_ok()) {
                (true, true, true) => {
                    assert!((1..=5).contains(&stage), "Valid results should only occur for stages 1-5");
                }
                (false, false, false) => {
                    assert!(!(1..=5).contains(&stage), "Invalid results should only occur for stages outside 1-5");
                }
                _ => {
                    panic!("Validation should be consistent across calls for stage {}", stage);
                }
            }
        }
    }
}
