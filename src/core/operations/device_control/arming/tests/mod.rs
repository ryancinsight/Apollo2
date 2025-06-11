//! Tests for device arming operations
//!
//! This module provides comprehensive test coverage for the unified device arming
//! operations following the established testing patterns from stage operations.
//!
//! Test organization follows the 6+ level hierarchical structure:
//! - `unit_tests` - Core function testing with mock devices
//! - `integration_tests` - Full operation flow testing
//! - `error_scenarios` - Error handling and edge case testing
//! - `mock_device` - Mock device implementations for testing

pub mod unit_tests;
pub mod integration_tests;
pub mod error_scenarios;
pub mod mock_device;

// Re-export test utilities for convenience
pub use mock_device::*;
