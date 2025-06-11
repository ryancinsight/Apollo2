//! Parameter and configuration model definitions
//!
//! This module contains types and structures related to device configuration
//! parameters including stage definitions and operational settings.

use crate::core::{LumidoxError, Result};
use crate::communication::protocol::commands;

/// Device stage configuration
/// 
/// Represents a single stage of the Lumidox II device with its associated
/// configuration parameters. Each stage has a unique number (1-5) and
/// current setting that determines its operational characteristics.
/// 
/// Stages are the fundamental operational units of the device, and each
/// can be configured and controlled independently.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Stage {
    /// Stage number (1-5)
    /// 
    /// Unique identifier for this stage. Must be in the range 1-5
    /// as the device supports exactly 5 configurable stages.
    pub number: u8,
    
    /// Current setting in milliamps
    /// 
    /// The current level configured for this stage. This determines
    /// the output intensity when the stage is activated.
    pub current_ma: u16,
}

impl Stage {
    /// Create a new stage configuration
    /// 
    /// Creates a new Stage instance with the specified stage number.
    /// The current is initialized to 0 and should be set separately.
    /// 
    /// # Arguments
    /// * `number` - Stage number (must be 1-5)
    /// 
    /// # Returns
    /// * `Ok(Stage)` - Successfully created stage
    /// * `Err(LumidoxError::InvalidInput)` - Invalid stage number
    /// 
    /// # Example
    /// ```
    /// let stage = Stage::new(1)?;
    /// assert_eq!(stage.number, 1);
    /// assert_eq!(stage.current_ma, 0);
    /// ```
    pub fn new(number: u8) -> Result<Self> {
        if !(1..=5).contains(&number) {
            return Err(LumidoxError::InvalidInput(
                format!("Invalid stage number: {}. Must be 1-5", number)
            ));
        }
        Ok(Stage { number, current_ma: 0 })
    }
    
    /// Get the command for reading this stage's current
    /// 
    /// Returns the protocol command bytes needed to read the current
    /// setting for this specific stage from the device.
    /// 
    /// # Returns
    /// * `&'static [u8]` - Command bytes for reading stage current
    /// 
    /// # Example
    /// ```
    /// let stage = Stage::new(1)?;
    /// let command = stage.current_command();
    /// // command will be b"78" for stage 1
    /// ```
    pub fn current_command(&self) -> &'static [u8] {
        commands::STAGE_CURRENTS[(self.number - 1) as usize]
    }
}
