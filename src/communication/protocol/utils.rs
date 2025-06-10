//! Protocol utility functions for Lumidox II Controller
//!
//! This module provides utility functions for protocol data processing,
//! including string data reading and other protocol-specific operations.

use crate::core::Result;
use super::handler::ProtocolHandler;

/// Read string data from device using multiple commands
pub fn read_string_data(
    handler: &mut ProtocolHandler, 
    commands: &[&[u8]]
) -> Result<String> {
    let mut result = String::new();
    
    for &cmd in commands {
        let val = handler.send_command(cmd, 0)?;
        if val > 0 && val < 256 {
            result.push(val as u8 as char);
        }
    }
    
    Ok(result.trim_end_matches('\0').to_string())
}
