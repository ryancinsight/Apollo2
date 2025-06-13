//! Data integrity error handling utilities
//!
//! This module provides specialized error handling for data integrity-related
//! errors in the Lumidox II Controller. It handles various integrity scenarios
//! including checksum validation, corruption detection, and data verification.

use crate::core::error::types::LumidoxError;

/// Integrity error categories for better error classification
#[derive(Debug, Clone, PartialEq)]
pub enum IntegrityErrorCategory {
    /// Checksum validation errors
    Checksum,
    /// Data corruption errors
    Corruption,
    /// Verification errors
    Verification,
    /// Consistency errors
    Consistency,
}

/// Integrity error utilities and helper functions
pub struct IntegrityErrorUtils;

impl IntegrityErrorUtils {
    /// Create a data integrity error
    /// 
    /// Used when data corruption or checksum failures are detected.
    /// 
    /// # Arguments
    /// * `data_description` - Description of the corrupted data
    /// * `integrity_check` - The type of integrity check that failed
    /// 
    /// # Returns
    /// * `LumidoxError::ProtocolError` - Formatted data integrity error
    pub fn checksum_error(data_description: &str, expected: &str, actual: &str) -> LumidoxError {
        LumidoxError::ProtocolError(format!(
            "Checksum error in {}: expected '{}', got '{}'", 
            data_description, expected, actual
        ))
    }
    
    /// Create a data corruption error
    pub fn corruption_error(data_type: &str, corruption_details: &str) -> LumidoxError {
        LumidoxError::ProtocolError(format!(
            "Data corruption detected in {}: {}", 
            data_type, corruption_details
        ))
    }
    
    /// Create a verification error
    pub fn verification_error(verification_type: &str, details: &str) -> LumidoxError {
        LumidoxError::ProtocolError(format!(
            "Verification failed ({}): {}", 
            verification_type, details
        ))
    }
    
    /// Create a consistency error
    pub fn consistency_error(data_type: &str, inconsistency: &str) -> LumidoxError {
        LumidoxError::ProtocolError(format!(
            "Data consistency error in {}: {}", 
            data_type, inconsistency
        ))
    }
    
    /// Categorize an integrity error
    pub fn categorize_error(error_message: &str) -> IntegrityErrorCategory {
        let message_lower = error_message.to_lowercase();
        
        if message_lower.contains("checksum") {
            IntegrityErrorCategory::Checksum
        } else if message_lower.contains("corruption") || message_lower.contains("corrupt") {
            IntegrityErrorCategory::Corruption
        } else if message_lower.contains("verification") || message_lower.contains("verify") {
            IntegrityErrorCategory::Verification
        } else {
            IntegrityErrorCategory::Consistency
        }
    }
}
