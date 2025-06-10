//! Device operations module for Lumidox II Controller
//!
//! This module contains all device operation functions including
//! firing stages, setting modes, and power management.

pub mod control;
pub mod power;

// Re-export commonly used items for convenience
// Note: Individual functions are accessed via module paths to avoid naming conflicts
