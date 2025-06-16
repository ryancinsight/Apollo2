//! Calculation utilities for Lumidox II Controller
//!
//! This module provides calculation utilities for device operations including
//! power, irradiance, and geometry calculations.
//!
//! Modules:
//! - `irradiance`: mW/cm² and plate geometry calculations

pub mod irradiance;

// Re-export for convenience
pub use irradiance::*;
