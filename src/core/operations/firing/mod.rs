//! Firing operations module for Lumidox II Controller
//!
//! This module provides unified firing operations that eliminate duplication between
//! CLI and GUI implementations. It implements the "Code as a Database" paradigm
//! where firing operations are treated as normalized, structured transactions.
//!
//! The firing operations module includes:
//! - Stage-based firing operations (stages 1-5)
//! - Custom current firing operations
//! - Unified result types with structured firing data
//! - Validation and error handling for firing parameters
//! - Interface-independent firing logic

pub mod stage_operations;

// Re-export commonly used types
pub use stage_operations::StageOperations;
