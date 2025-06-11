//! Unit tests for stage operations core functionality
//!
//! This module provides comprehensive unit tests for the StageOperations implementation,
//! focusing on isolated testing without external dependencies.

use super::super::StageOperations;
use crate::core::operations::result_types::DeviceOperationData;
use crate::core::LumidoxError;

#[cfg(test)]
mod stage_validation_tests {
    use super::*;

    #[test]
    fn test_validate_stage_number_valid_stages() {
        // Test all valid stage numbers (1-5)
        for stage in 1..=5 {
            let result = StageOperations::validate_stage_number(stage);
            assert!(result.is_ok(), "Stage {} should be valid", stage);
        }
    }

    #[test]
    fn test_validate_stage_number_invalid_zero() {
        let result = StageOperations::validate_stage_number(0);
        assert!(result.is_err(), "Stage 0 should be invalid");
        
        if let Err(LumidoxError::InvalidInput(msg)) = result {
            assert!(msg.contains("Invalid stage number: 0"));
            assert!(msg.contains("Must be 1-5"));
        } else {
            panic!("Expected InvalidInput error for stage 0");
        }
    }

    #[test]
    fn test_validate_stage_number_invalid_high() {
        for invalid_stage in [6, 7, 10, 255] {
            let result = StageOperations::validate_stage_number(invalid_stage);
            assert!(result.is_err(), "Stage {} should be invalid", invalid_stage);
            
            if let Err(LumidoxError::InvalidInput(msg)) = result {
                assert!(msg.contains(&format!("Invalid stage number: {}", invalid_stage)));
                assert!(msg.contains("Must be 1-5"));
            } else {
                panic!("Expected InvalidInput error for stage {}", invalid_stage);
            }
        }
    }

    #[test]
    fn test_validate_stage_number_boundary_conditions() {
        // Test boundary conditions
        assert!(StageOperations::validate_stage_number(1).is_ok(), "Stage 1 (lower bound) should be valid");
        assert!(StageOperations::validate_stage_number(5).is_ok(), "Stage 5 (upper bound) should be valid");
        assert!(StageOperations::validate_stage_number(0).is_err(), "Stage 0 (below lower bound) should be invalid");
        assert!(StageOperations::validate_stage_number(6).is_err(), "Stage 6 (above upper bound) should be invalid");
    }
}

#[cfg(test)]
mod operation_response_tests {
    use super::*;

    #[test]
    fn test_stage_firing_data_structure() {
        // Test DeviceOperationData::StageFiring structure
        let stage_data = DeviceOperationData::StageFiring {
            stage: 3,
            current_ma: Some(100),
            success: true,
        };

        match stage_data {
            DeviceOperationData::StageFiring { stage, current_ma, success } => {
                assert_eq!(stage, 3);
                assert_eq!(current_ma, Some(100));
                assert_eq!(success, true);
            }
            _ => panic!("Expected StageFiring variant"),
        }
    }

    #[test]
    fn test_stage_firing_data_with_none_current() {
        // Test DeviceOperationData::StageFiring with None current
        let stage_data = DeviceOperationData::StageFiring {
            stage: 2,
            current_ma: None,
            success: false,
        };

        match stage_data {
            DeviceOperationData::StageFiring { stage, current_ma, success } => {
                assert_eq!(stage, 2);
                assert_eq!(current_ma, None);
                assert_eq!(success, false);
            }
            _ => panic!("Expected StageFiring variant"),
        }
    }

    #[test]
    fn test_stage_firing_data_boundary_values() {
        // Test with boundary stage values
        let stage_data_min = DeviceOperationData::StageFiring {
            stage: 1,
            current_ma: Some(0),
            success: true,
        };

        let stage_data_max = DeviceOperationData::StageFiring {
            stage: 5,
            current_ma: Some(65535),
            success: true,
        };

        match stage_data_min {
            DeviceOperationData::StageFiring { stage, current_ma, success } => {
                assert_eq!(stage, 1);
                assert_eq!(current_ma, Some(0));
                assert_eq!(success, true);
            }
            _ => panic!("Expected StageFiring variant for min values"),
        }

        match stage_data_max {
            DeviceOperationData::StageFiring { stage, current_ma, success } => {
                assert_eq!(stage, 5);
                assert_eq!(current_ma, Some(65535));
                assert_eq!(success, true);
            }
            _ => panic!("Expected StageFiring variant for max values"),
        }
    }
}

#[cfg(test)]
mod error_handling_tests {
    use super::*;

    #[test]
    fn test_invalid_stage_error_message_format() {
        let result = StageOperations::validate_stage_number(0);
        
        match result {
            Err(LumidoxError::InvalidInput(msg)) => {
                assert!(msg.contains("Invalid stage number: 0"));
                assert!(msg.contains("Must be 1-5"));
                assert!(msg.len() > 10, "Error message should be descriptive");
            }
            _ => panic!("Expected InvalidInput error"),
        }
    }

    #[test]
    fn test_error_message_consistency() {
        // Test that error messages are consistent across different invalid values
        let invalid_stages = [0, 6, 10, 255];
        
        for &stage in &invalid_stages {
            let result = StageOperations::validate_stage_number(stage);
            
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

#[cfg(test)]
mod performance_tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_validation_performance() {
        // Test that validation is fast (should complete in microseconds)
        let start = Instant::now();
        
        for _ in 0..1000 {
            for stage in 1..=5 {
                let _ = StageOperations::validate_stage_number(stage);
            }
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 10, "Validation should be very fast");
    }

    #[test]
    fn test_validation_deterministic() {
        // Test that validation is deterministic (same input = same output)
        for stage in 0..=10 {
            let result1 = StageOperations::validate_stage_number(stage);
            let result2 = StageOperations::validate_stage_number(stage);
            
            match (result1, result2) {
                (Ok(()), Ok(())) => {
                    assert!((1..=5).contains(&stage), "Valid stages should be 1-5");
                }
                (Err(_), Err(_)) => {
                    assert!(!(1..=5).contains(&stage), "Invalid stages should not be 1-5");
                }
                _ => panic!("Validation should be deterministic for stage {}", stage),
            }
        }
    }
}
