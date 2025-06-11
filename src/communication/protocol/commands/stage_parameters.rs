//! Stage parameter command definitions
//!
//! This module contains protocol commands for reading and configuring
//! stage-specific parameters including currents and voltage settings.

/// Stage FIRE current commands
/// 
/// Command sequence to read FIRE current settings for each stage.
/// Commands: 0x78 (Stage 1), 0x80 (Stage 2), 0x88 (Stage 3), 0x90 (Stage 4), 0x98 (Stage 5)
pub const STAGE_CURRENTS: [&[u8]; 5] = [
    b"78", // Stage 1
    b"80", // Stage 2
    b"88", // Stage 3
    b"90", // Stage 4
    b"98", // Stage 5
];

/// Stage ARM current commands
/// 
/// Command sequence to read ARM current settings for each stage.
/// Commands: 0x77 (Stage 1), 0x7f (Stage 2), 0x87 (Stage 3), 0x8f (Stage 4), 0x97 (Stage 5)
pub const STAGE_ARM_CURRENTS: [&[u8]; 5] = [
    b"77", // Stage 1
    b"7f", // Stage 2
    b"87", // Stage 3
    b"8f", // Stage 4
    b"97", // Stage 5
];

/// Stage voltage limit commands
/// 
/// Command sequence to read voltage limit settings for each stage.
/// Commands: 0x79 (Stage 1), 0x81 (Stage 2), 0x89 (Stage 3), 0x91 (Stage 4), 0x99 (Stage 5)
pub const STAGE_VOLT_LIMITS: [&[u8]; 5] = [
    b"79", // Stage 1
    b"81", // Stage 2
    b"89", // Stage 3
    b"91", // Stage 4
    b"99", // Stage 5
];

/// Stage voltage start commands
/// 
/// Command sequence to read voltage start settings for each stage.
/// Commands: 0x7a (Stage 1), 0x82 (Stage 2), 0x8a (Stage 3), 0x92 (Stage 4), 0x9a (Stage 5)
pub const STAGE_VOLT_STARTS: [&[u8]; 5] = [
    b"7a", // Stage 1
    b"82", // Stage 2
    b"8a", // Stage 3
    b"92", // Stage 4
    b"9a", // Stage 5
];
