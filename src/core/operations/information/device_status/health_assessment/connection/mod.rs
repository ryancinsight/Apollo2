//! Connection health assessment operations
//!
//! This module provides specialized connection health assessment operations
//! in the Lumidox II Controller. It handles various connection health scenarios
//! including diagnostic operations and connection validation.

// Import specialized sub-modules
pub mod diagnostic;

// Re-export commonly used items for convenience
// Note: Utilities are available but not currently used in the codebase
// pub use diagnostic::{ConnectionDiagnosticOperations, ConnectionDiagnosticCategory};

use crate::device::LumidoxDevice;

/// Connection health assessment categories for better classification
#[derive(Debug, Clone, PartialEq)]
pub enum ConnectionHealthCategory {
    /// Basic connection health
    Basic,
    /// Advanced connection diagnostics
    Advanced,
    /// Connection stability assessment
    Stability,
    /// Connection performance assessment
    Performance,
}

/// Connection health assessment operations
pub struct ConnectionHealthOperations;

impl ConnectionHealthOperations {
    /// Assess basic connection health
    ///
    /// Provides fundamental connection health assessment using basic communication tests.
    ///
    /// # Arguments
    /// * `device` - Reference to the device to assess
    ///
    /// # Returns
    /// * `bool` - True if basic connection is healthy
    pub fn assess_basic_health(device: &LumidoxDevice) -> bool {
        diagnostic::ConnectionDiagnosticOperations::test_basic_communication(device)
    }

    /// Assess advanced connection health
    ///
    /// Provides comprehensive connection health assessment using advanced diagnostics.
    ///
    /// # Arguments
    /// * `device` - Reference to the device to assess
    ///
    /// # Returns
    /// * `bool` - True if advanced connection health is good
    pub fn assess_advanced_health(device: &LumidoxDevice) -> bool {
        diagnostic::ConnectionDiagnosticOperations::perform_comprehensive_diagnostics(device)
    }

    /// Assess connection stability
    ///
    /// Evaluates connection stability over time and under various conditions.
    ///
    /// # Arguments
    /// * `device` - Reference to the device to assess
    ///
    /// # Returns
    /// * `bool` - True if connection is stable
    pub fn assess_stability(device: &LumidoxDevice) -> bool {
        diagnostic::ConnectionDiagnosticOperations::assess_connection_stability(device)
    }

    /// Assess connection performance
    ///
    /// Evaluates connection performance metrics and responsiveness.
    ///
    /// # Arguments
    /// * `device` - Reference to the device to assess
    ///
    /// # Returns
    /// * `bool` - True if connection performance is acceptable
    pub fn assess_performance(device: &LumidoxDevice) -> bool {
        // For now, use basic communication as performance indicator
        // In a full implementation, this would measure response times and throughput
        diagnostic::ConnectionDiagnosticOperations::test_basic_communication(device)
    }

    /// Get comprehensive connection health report
    ///
    /// Provides a detailed report of all connection health aspects.
    ///
    /// # Arguments
    /// * `device` - Reference to the device to report on
    ///
    /// # Returns
    /// * `String` - Comprehensive health report
    pub fn get_health_report(device: &LumidoxDevice) -> String {
        let basic = Self::assess_basic_health(device);
        let advanced = Self::assess_advanced_health(device);
        let stability = Self::assess_stability(device);
        let performance = Self::assess_performance(device);
        
        format!(
            "Connection Health Report:\n- Basic: {}\n- Advanced: {}\n- Stability: {}\n- Performance: {}",
            if basic { "Healthy" } else { "Issues" },
            if advanced { "Healthy" } else { "Issues" },
            if stability { "Stable" } else { "Unstable" },
            if performance { "Good" } else { "Poor" }
        )
    }

    /// Categorize a connection health assessment
    pub fn categorize_health_assessment(assessment_type: &str) -> ConnectionHealthCategory {
        let assessment_lower = assessment_type.to_lowercase();
        
        if assessment_lower.contains("basic") {
            ConnectionHealthCategory::Basic
        } else if assessment_lower.contains("advanced") || assessment_lower.contains("diagnostic") {
            ConnectionHealthCategory::Advanced
        } else if assessment_lower.contains("stability") {
            ConnectionHealthCategory::Stability
        } else {
            ConnectionHealthCategory::Performance
        }
    }
}
