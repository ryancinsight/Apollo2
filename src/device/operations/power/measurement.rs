//! Power measurement operations for Lumidox II Controller
//!
//! This module provides functions for reading power information
//! from device stages and decoding unit information.

use crate::core::{LumidoxError, Result};
use crate::communication::ProtocolHandler;
use crate::device::models::PowerInfo;

/// Get power information for a specific stage
pub fn get_power_info(protocol: &mut ProtocolHandler, stage_num: u8) -> Result<PowerInfo> {
    if !(1..=5).contains(&stage_num) {
        return Err(LumidoxError::InvalidInput(
            format!("Invalid stage number: {}", stage_num)
        ));
    }
    
    let stage_idx = (stage_num - 1) as usize;
    let base_cmd = match stage_idx {
        0 => 0x7b, // Stage 1: 0x7b-0x7e
        1 => 0x83, // Stage 2: 0x83-0x86
        2 => 0x8b, // Stage 3: 0x8b-0x8e
        3 => 0x93, // Stage 4: 0x93-0x96
        4 => 0x9b, // Stage 5: 0x9b-0x9e
        _ => unreachable!(),
    };
    
    let total_power_cmd = format!("{:02x}", base_cmd);
    let per_power_cmd = format!("{:02x}", base_cmd + 1);
    let total_units_cmd = format!("{:02x}", base_cmd + 2);
    let per_units_cmd = format!("{:02x}", base_cmd + 3);
    
    let total_power = protocol.send_command(total_power_cmd.as_bytes(), 0)? as f32 / 10.0;
    let per_power = protocol.send_command(per_power_cmd.as_bytes(), 0)? as f32 / 10.0;
    let total_units_idx = protocol.send_command(total_units_cmd.as_bytes(), 0)?;
    let per_units_idx = protocol.send_command(per_units_cmd.as_bytes(), 0)?;
    
    Ok(PowerInfo {
        total_power,
        total_units: decode_total_units(total_units_idx),
        per_power,
        per_units: decode_per_units(per_units_idx),
    })
}

/// Decode total units index to human-readable string
pub fn decode_total_units(index: i32) -> String {
    match index {
        0 => "W TOTAL RADIANT POWER".to_string(),
        1 => "mW TOTAL RADIANT POWER".to_string(),
        2 => "W/cm² TOTAL IRRADIANCE".to_string(),
        3 => "mW/cm² TOTAL IRRADIANCE".to_string(),
        4 => "".to_string(),
        5 => "A TOTAL CURRENT".to_string(),
        6 => "mA TOTAL CURRENT".to_string(),
        _ => "UNKNOWN UNITS".to_string(),
    }
}

/// Decode per-unit index to human-readable string
pub fn decode_per_units(index: i32) -> String {
    match index {
        0 => "W PER WELL".to_string(),
        1 => "mW PER WELL".to_string(),
        2 => "W TOTAL RADIANT POWER".to_string(),
        3 => "mW TOTAL RADIANT POWER".to_string(),
        4 => "mW/cm² PER WELL".to_string(),
        5 => "mW/cm²".to_string(),
        6 => "J/s".to_string(),
        7 => "".to_string(),
        8 => "A PER WELL".to_string(),
        9 => "mA PER WELL".to_string(),
        _ => "UNKNOWN UNITS".to_string(),
    }
}
