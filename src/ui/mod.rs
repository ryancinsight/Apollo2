//! User Interface module for Lumidox II Controller
//!
//! This module contains all user interface components organized into sub-modules:
//! - `cli`: Command-line interface with organized sub-components
//! - `gui`: Graphical user interface with Iced-based components
//!
//! The module supports dual-mode operation where the application can run in either
//! CLI mode (command-line interface) or GUI mode (graphical interface) based on
//! user preference and system capabilities.

pub mod cli;

// Conditional compilation for GUI module
#[cfg(feature = "gui")]
pub mod gui;

// Re-export GUI functionality when available
#[cfg(feature = "gui")]
pub use gui::run_gui;


// Placeholder GUI module when GUI feature is not enabled
#[cfg(not(feature = "gui"))]
pub mod gui {
    use crate::core::{LumidoxError, Result};

    /// Placeholder GUI function when GUI feature is not enabled
    ///
    /// This function provides a graceful fallback when the GUI dependencies
    /// are not available or the GUI feature is not enabled.
    ///
    /// # Arguments
    ///
    /// * `port_name` - Optional port name for device connection
    /// * `auto_detect` - Whether to use automatic port detection
    /// * `verbose` - Whether to use verbose output
    /// * `optimize_transitions` - Whether to use optimized stage transitions
    ///
    /// # Returns
    ///
    /// * `Result<()>` - Always returns an error to trigger CLI fallback
    pub fn run_gui(
        _port_name: Option<String>,
        _auto_detect: bool,
        _verbose: bool,
        _optimize_transitions: bool,
    ) -> Result<()> {
        Err(LumidoxError::ConfigError(
            "GUI mode is not available. GUI dependencies not installed or feature not enabled.".to_string()
        ))
    }
}

// Re-export GUI functionality when not available (placeholder)
#[cfg(not(feature = "gui"))]
pub use gui::run_gui;

// Re-export commonly used items for convenience
pub use cli::{Cli, Commands, run_interactive_mode, run_command_mode,
              run_interactive_mode_with_optimization, run_command_mode_with_optimization,
              list_serial_ports};

// Re-export GUI functionality for dual-mode integration
// (Already re-exported above based on feature flags)
