//! Stage parameter operations for Lumidox II Controller
//!
//! This module provides functions for reading complete stage parameters
//! including ARM current, voltage limits, and other stage-specific settings.
//! 
//! Based on LumidoxII.md protocol specification, this module will implement
//! missing protocol commands for complete stage parameter access.

use crate::core::{LumidoxError, Result};
use crate::communication::ProtocolHandler;

/// Stage parameter structure for complete stage information
#[derive(Debug, Clone)]
pub struct StageParameters {
    pub stage_number: u8,
    pub arm_current_ma: u16,
    pub fire_current_ma: u16,
    pub volt_limit_v: f32,
    pub volt_start_v: f32,
    pub power_total: f32,
    pub power_per_led: f32,
    pub total_units: String,
    pub per_led_units: String,
}

/// Get complete stage parameters
///
/// This function implements the complete protocol commands from LumidoxII.md:
/// - ARM Current: 0x77, 0x7f, 0x87, 0x8f, 0x97 (Stages 1-5)
/// - FIRE Current: 0x78, 0x80, 0x88, 0x90, 0x98 (Stages 1-5)
/// - VOLT Limit: 0x79, 0x81, 0x89, 0x91, 0x99 (Stages 1-5)
/// - VOLT Start: 0x7a, 0x82, 0x8a, 0x92, 0x9a (Stages 1-5)
/// - Power measurements: Combined from existing power info functionality
pub fn get_stage_parameters(protocol: &mut ProtocolHandler, stage_num: u8) -> Result<StageParameters> {
    if !(1..=5).contains(&stage_num) {
        return Err(LumidoxError::InvalidInput(
            format!("Invalid stage number: {}. Must be 1-5", stage_num)
        ));
    }

    // Get ARM current for this stage
    let arm_current_ma = get_stage_arm_current(protocol, stage_num)?;

    // Get FIRE current for this stage using existing STAGE_CURRENTS commands
    let stage_idx = (stage_num - 1) as usize;
    let fire_command = crate::communication::protocol::commands::STAGE_CURRENTS[stage_idx];
    let fire_current_ma = protocol.send_command(fire_command, 0)? as u16;

    // Get voltage parameters for this stage
    let volt_limit_v = get_stage_volt_limit(protocol, stage_num)?;
    let volt_start_v = get_stage_volt_start(protocol, stage_num)?;

    // Get power information for this stage using existing power measurement functionality
    let power_info = super::measurement::get_power_info(protocol, stage_num)?;

    Ok(StageParameters {
        stage_number: stage_num,
        arm_current_ma,
        fire_current_ma,
        volt_limit_v,
        volt_start_v,
        power_total: power_info.total_power,
        power_per_led: power_info.per_power,
        total_units: power_info.total_units,
        per_led_units: power_info.per_units,
    })
}

/// Get ARM current for a specific stage
///
/// Protocol commands: 0x77 (Stage 1), 0x7f (Stage 2), 0x87 (Stage 3), 0x8f (Stage 4), 0x97 (Stage 5)
pub fn get_stage_arm_current(protocol: &mut ProtocolHandler, stage_num: u8) -> Result<u16> {
    if !(1..=5).contains(&stage_num) {
        return Err(LumidoxError::InvalidInput(
            format!("Invalid stage number: {}. Must be 1-5", stage_num)
        ));
    }

    // Get the appropriate command for this stage (stages are 1-indexed, array is 0-indexed)
    let stage_idx = (stage_num - 1) as usize;
    let command = crate::communication::protocol::commands::STAGE_ARM_CURRENTS[stage_idx];

    // Send command and get ARM current value
    let arm_current = protocol.send_command(command, 0)? as u16;

    Ok(arm_current)
}

/// Get FIRE current for a specific stage
///
/// Protocol commands: 0x78 (Stage 1), 0x80 (Stage 2), 0x88 (Stage 3), 0x90 (Stage 4), 0x98 (Stage 5)
pub fn get_stage_fire_current(protocol: &mut ProtocolHandler, stage_num: u8) -> Result<u16> {
    if !(1..=5).contains(&stage_num) {
        return Err(LumidoxError::InvalidInput(
            format!("Invalid stage number: {}. Must be 1-5", stage_num)
        ));
    }

    // Get the appropriate command for this stage (stages are 1-indexed, array is 0-indexed)
    let stage_idx = (stage_num - 1) as usize;
    let fire_command = crate::communication::protocol::commands::STAGE_CURRENTS[stage_idx];

    // Send command and get FIRE current value
    let fire_current = protocol.send_command(fire_command, 0)? as u16;

    Ok(fire_current)
}

/// Get voltage limit for a specific stage
///
/// Protocol commands: 0x79 (Stage 1), 0x81 (Stage 2), 0x89 (Stage 3), 0x91 (Stage 4), 0x99 (Stage 5)
pub fn get_stage_volt_limit(protocol: &mut ProtocolHandler, stage_num: u8) -> Result<f32> {
    if !(1..=5).contains(&stage_num) {
        return Err(LumidoxError::InvalidInput(
            format!("Invalid stage number: {}. Must be 1-5", stage_num)
        ));
    }

    // Get the appropriate command for this stage (stages are 1-indexed, array is 0-indexed)
    let stage_idx = (stage_num - 1) as usize;
    let command = crate::communication::protocol::commands::STAGE_VOLT_LIMITS[stage_idx];

    // Send command and get voltage limit value
    // Convert from device units to volts (assuming device returns in appropriate units)
    let volt_limit = protocol.send_command(command, 0)? as f32 / 10.0;

    Ok(volt_limit)
}

/// Get voltage start for a specific stage
///
/// Protocol commands: 0x7a (Stage 1), 0x82 (Stage 2), 0x8a (Stage 3), 0x92 (Stage 4), 0x9a (Stage 5)
pub fn get_stage_volt_start(protocol: &mut ProtocolHandler, stage_num: u8) -> Result<f32> {
    if !(1..=5).contains(&stage_num) {
        return Err(LumidoxError::InvalidInput(
            format!("Invalid stage number: {}. Must be 1-5", stage_num)
        ));
    }

    // Get the appropriate command for this stage (stages are 1-indexed, array is 0-indexed)
    let stage_idx = (stage_num - 1) as usize;
    let command = crate::communication::protocol::commands::STAGE_VOLT_STARTS[stage_idx];

    // Send command and get voltage start value
    // Convert from device units to volts (assuming device returns in appropriate units)
    let volt_start = protocol.send_command(command, 0)? as f32 / 10.0;

    Ok(volt_start)
}
