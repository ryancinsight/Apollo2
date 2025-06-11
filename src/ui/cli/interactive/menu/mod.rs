//! Menu system for interactive CLI
//!
//! This module organizes menu functionality into specialized components:
//! - display: Menu display logic and formatting
//! - validation: Menu option validation and processing
//! - organization: Menu structure and categorization

pub mod display;
pub mod validation;
pub mod organization;

// Re-export commonly used items for convenience
pub use display::MenuDisplay;
pub use validation::MenuValidation;
pub use organization::MenuOrganization;
