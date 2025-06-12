//! Test module for power control operations
//!
//! This module provides comprehensive testing for power control operations
//! including unit tests, integration tests, and error scenario testing.
//! All tests use mock devices to ensure deterministic behavior without
//! requiring actual hardware.

pub mod unit_tests;
pub mod integration_tests;
pub mod error_scenarios;
pub mod mock_device;

// Re-export commonly used test utilities
pub use mock_device::*;
