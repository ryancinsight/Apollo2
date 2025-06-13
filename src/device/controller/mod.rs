//! Device controller module for Lumidox II Controller
//!
//! This module provides the main LumidoxDevice controller that orchestrates
//! device operations using specialized sub-modules for initialization, state
//! management, control operations, and information retrieval.
//!
//! The controller architecture is organized into focused sub-modules:
//! - `initialization`: Device setup and initialization procedures
//! - `state_management`: Device mode control and state tracking
//! - `operations`: Control and information operations
//!
//! This modular design provides:
//! - Clear separation of concerns for different controller responsibilities
//! - Enhanced maintainability through focused, single-responsibility modules
//! - Improved testability with isolated functional components
//! - Scalable architecture for future feature additions
//! - Comprehensive documentation and usage examples

use crate::core::Result;
use crate::communication::ProtocolHandler;
use crate::device::models::{DeviceMode, DeviceInfo, PowerInfo};
use crate::device::operations as device_operations;

// Sub-module declarations
pub mod initialization;
pub mod state_management;

// Re-export key types and utilities for convenience
pub use initialization::setup::DeviceInitializer;
pub use state_management::mode_control::DeviceStateManager;

/// High-level device controller with modular architecture
/// 
/// The LumidoxDevice provides a unified interface for controlling and monitoring
/// the Lumidox II Controller device. It integrates specialized sub-modules to
/// provide comprehensive device management capabilities while maintaining a
/// clean and intuitive public API.
/// 
/// # Architecture
/// The controller is built on a modular architecture with the following components:
/// - **Initialization**: Device setup and configuration management
/// - **State Management**: Mode control and state tracking
/// - **Control Operations**: Device control functions (firing, arming, shutdown)
/// - **Information Operations**: Device status and information retrieval
/// 
/// # Usage Patterns
/// ```
/// // Create and initialize device
/// let protocol = ProtocolHandler::new(port)?;
/// let mut device = LumidoxDevice::new(protocol);
/// device.initialize()?;
/// 
/// // Control operations
/// device.arm()?;
/// device.fire_stage(1)?;
/// device.turn_off()?;
/// 
/// // Information retrieval
/// let status = device.read_device_state()?;
/// let power_info = device.get_power_info(2)?;
/// ```
pub struct LumidoxDevice {
    /// Protocol handler for device communication
    pub(crate) protocol: ProtocolHandler,
    /// Cached device information (loaded during initialization)
    pub(crate) info: Option<DeviceInfo>,
    /// Current device mode tracking
    pub(crate) current_mode: Option<DeviceMode>,
    /// Whether to use optimized stage transitions (true) or always use full safety sequence (false)
    pub(crate) optimize_transitions: bool,
}

impl LumidoxDevice {
    /// Create a new device controller with optimized transitions enabled by default
    /// 
    /// Creates a new LumidoxDevice instance with default settings optimized for
    /// typical usage patterns. Optimized transitions are enabled by default to
    /// provide better performance while maintaining safety.
    /// 
    /// # Arguments
    /// * `protocol` - The protocol handler for device communication
    /// 
    /// # Returns
    /// * `LumidoxDevice` - A new device controller instance
    /// 
    /// # Example
    /// ```
    /// let protocol = ProtocolHandler::new(port)?;
    /// let device = LumidoxDevice::new(protocol);
    /// ```
    pub fn new(protocol: ProtocolHandler) -> Self {
        DeviceInitializer::create_default(protocol)
    }

    /// Create a new device controller with specified optimization setting
    /// 
    /// Creates a new LumidoxDevice instance with custom optimization settings
    /// for specialized use cases that require specific transition behavior.
    /// 
    /// # Arguments
    /// * `protocol` - The protocol handler for device communication
    /// * `optimize_transitions` - Whether to enable optimized stage transitions
    /// 
    /// # Returns
    /// * `LumidoxDevice` - A new device controller instance with specified settings
    /// 
    /// # Example
    /// ```
    /// let protocol = ProtocolHandler::new(port)?;
    /// let device = LumidoxDevice::new_with_optimization(protocol, false);
    /// ```
    pub fn new_with_optimization(protocol: ProtocolHandler, optimize_transitions: bool) -> Self {
        DeviceInitializer::create_with_optimization(protocol, optimize_transitions)
    }

    /// Enable or disable optimized stage transitions
    /// 
    /// Configures the optimization setting for stage transitions, allowing
    /// runtime adjustment of device behavior based on operational requirements.
    /// 
    /// # Arguments
    /// * `optimize` - Whether to enable optimized transitions
    /// 
    /// # Example
    /// ```
    /// device.set_optimize_transitions(false); // Use full safety sequence
    /// ```
    pub fn set_optimize_transitions(&mut self, optimize: bool) {
        self.optimize_transitions = optimize;
    }

    /// Check if optimized transitions are enabled
    /// 
    /// Returns the current optimization setting for stage transitions.
    /// 
    /// # Returns
    /// * `bool` - True if optimized transitions are enabled
    /// 
    /// # Example
    /// ```
    /// if device.is_optimize_transitions() {
    ///     println!("Using optimized transitions");
    /// }
    /// ```
    pub fn is_optimize_transitions(&self) -> bool {
        self.optimize_transitions
    }

    /// Initialize the device and retrieve basic information
    /// 
    /// Performs the complete device initialization sequence including mode
    /// setting and information retrieval. This method should be called after
    /// device creation to prepare it for operation.
    /// 
    /// # Returns
    /// * `Result<()>` - Success or initialization error
    /// 
    /// # Example
    /// ```
    /// let mut device = LumidoxDevice::new(protocol);
    /// device.initialize()?;
    /// ```
    pub fn initialize(&mut self) -> Result<()> {
        DeviceInitializer::initialize_device(self)
    }

    /// Get device information (cached after initialization)
    ///
    /// Returns the cached device information that was retrieved during
    /// initialization, including firmware version, model number, and other
    /// device characteristics.
    ///
    /// # Returns
    /// * `Option<&DeviceInfo>` - Reference to cached device info, if available
    ///
    /// # Example
    /// ```
    /// if let Some(info) = device.info() {
    ///     println!("Firmware: {}", info.firmware_version);
    /// }
    /// ```
    pub fn info(&self) -> Option<&DeviceInfo> {
        self.info.as_ref()
    }

    /// Set device operating mode
    /// 
    /// Sets the device to the specified operating mode and updates internal
    /// state tracking to maintain consistency.
    /// 
    /// # Arguments
    /// * `mode` - The target operating mode
    /// 
    /// # Returns
    /// * `Result<()>` - Success or mode setting error
    /// 
    /// # Example
    /// ```
    /// device.set_mode(DeviceMode::Standby)?;
    /// ```
    pub fn set_mode(&mut self, mode: DeviceMode) -> Result<()> {
        DeviceStateManager::set_device_mode(self, mode)
    }

    /// Get current device mode
    /// 
    /// Returns the current device mode from internal state tracking.
    /// 
    /// # Returns
    /// * `Option<DeviceMode>` - Current mode if set, None if uninitialized
    /// 
    /// # Example
    /// ```
    /// if let Some(mode) = device.current_mode() {
    ///     println!("Current mode: {:?}", mode);
    /// }
    /// ```
    pub fn current_mode(&self) -> Option<DeviceMode> {
        DeviceStateManager::get_current_mode(self)
    }

    /// Arm the device (prepare for firing)
    ///
    /// Prepares the device for firing operations by setting it to armed mode.
    ///
    /// # Returns
    /// * `Result<()>` - Success or arming error
    ///
    /// # Example
    /// ```
    /// device.arm()?;
    /// ```
    pub fn arm(&mut self) -> Result<()> {
        device_operations::control::arm_device(&mut self.protocol)?;
        self.current_mode = Some(DeviceMode::Remote);
        Ok(())
    }
    
    /// Fire a specific stage
    ///
    /// Fires the specified stage using optimization settings for improved
    /// performance when appropriate.
    ///
    /// # Arguments
    /// * `stage_num` - The stage number to fire (1-5)
    ///
    /// # Returns
    /// * `Result<()>` - Success or firing error
    ///
    /// # Example
    /// ```
    /// device.fire_stage(3)?;
    /// ```
    pub fn fire_stage(&mut self, stage_num: u8) -> Result<()> {
        if self.optimize_transitions {
            device_operations::control::fire_stage_smart(&mut self.protocol, stage_num, self.current_mode)?;
        } else {
            device_operations::control::fire_stage(&mut self.protocol, stage_num)?;
        }
        self.current_mode = Some(DeviceMode::Remote);
        Ok(())
    }

    /// Fire with a specific current value
    ///
    /// Fires the device with a custom current value using optimization
    /// settings for improved performance when appropriate.
    ///
    /// # Arguments
    /// * `current_ma` - The current value in milliamps
    ///
    /// # Returns
    /// * `Result<()>` - Success or firing error
    ///
    /// # Example
    /// ```
    /// device.fire_with_current(2500)?;
    /// ```
    pub fn fire_with_current(&mut self, current_ma: u16) -> Result<()> {
        if self.optimize_transitions {
            device_operations::control::fire_with_current_smart(&mut self.protocol, current_ma, self.current_mode)?;
        } else {
            device_operations::control::fire_with_current(&mut self.protocol, current_ma)?;
        }
        self.current_mode = Some(DeviceMode::Remote);
        Ok(())
    }

    /// Turn off the device
    ///
    /// Safely turns off the device output while maintaining remote control
    /// capability.
    ///
    /// # Returns
    /// * `Result<()>` - Success or turn-off error
    ///
    /// # Example
    /// ```
    /// device.turn_off()?;
    /// ```
    pub fn turn_off(&mut self) -> Result<()> {
        device_operations::control::turn_off(&mut self.protocol)?;
        self.current_mode = Some(DeviceMode::Remote);
        Ok(())
    }

    /// Shutdown and return to local mode
    ///
    /// Completely shuts down the device and returns it to local mode.
    ///
    /// # Returns
    /// * `Result<()>` - Success or shutdown error
    ///
    /// # Example
    /// ```
    /// device.shutdown()?;
    /// ```
    pub fn shutdown(&mut self) -> Result<()> {
        device_operations::control::shutdown(&mut self.protocol)?;
        self.current_mode = None;
        Ok(())
    }

    /// Get maximum current setting
    ///
    /// Queries the device to determine its maximum current capability.
    ///
    /// # Returns
    /// * `Result<u16>` - Maximum current in milliamps or query error
    ///
    /// # Example
    /// ```
    /// let max_current = device.get_max_current()?;
    /// ```
    pub fn get_max_current(&mut self) -> Result<u16> {
        device_operations::control::get_max_current(&mut self.protocol)
    }
    
    /// Get power information for a specific stage
    ///
    /// Retrieves power information for the specified stage.
    ///
    /// # Arguments
    /// * `stage_num` - The stage number to query (1-5)
    ///
    /// # Returns
    /// * `Result<PowerInfo>` - Power information or query error
    ///
    /// # Example
    /// ```
    /// let power_info = device.get_power_info(2)?;
    /// ```
    pub fn get_power_info(&mut self, stage_num: u8) -> Result<PowerInfo> {
        device_operations::power::get_power_info(&mut self.protocol, stage_num)
    }

    /// Read current device state description
    ///
    /// Provides a comprehensive description of the current device state.
    ///
    /// # Returns
    /// * `Result<String>` - Device state description or query error
    ///
    /// # Example
    /// ```
    /// let state = device.read_device_state()?;
    /// ```
    pub fn read_device_state(&mut self) -> Result<String> {
        device_operations::readback::get_device_state_description(&mut self.protocol)
    }

    /// Read current settings summary
    ///
    /// Provides a summary of current device settings.
    ///
    /// # Returns
    /// * `Result<String>` - Current settings summary or query error
    ///
    /// # Example
    /// ```
    /// let settings = device.read_current_settings()?;
    /// ```
    pub fn read_current_settings(&mut self) -> Result<String> {
        device_operations::readback::get_current_settings_summary(&mut self.protocol)
    }

    /// Read remote mode state
    /// 
    /// Queries the device to determine its current remote mode state.
    /// 
    /// # Returns
    /// * `Result<DeviceMode>` - Current remote mode state or query error
    /// 
    /// # Example
    /// ```
    /// let mode = device.read_remote_mode()?;
    /// ```
    pub fn read_remote_mode(&mut self) -> Result<DeviceMode> {
        device_operations::readback::read_remote_mode_state(&mut self.protocol)
    }

    /// Read ARM current setting
    ///
    /// Queries the device to retrieve the current ARM current setting.
    ///
    /// # Returns
    /// * `Result<u16>` - ARM current in milliamps or query error
    ///
    /// # Example
    /// ```
    /// let arm_current = device.read_arm_current()?;
    /// ```
    pub fn read_arm_current(&mut self) -> Result<u16> {
        device_operations::readback::read_arm_current(&mut self.protocol)
    }

    /// Read FIRE current setting
    ///
    /// Queries the device to retrieve the current FIRE current setting.
    ///
    /// # Returns
    /// * `Result<u16>` - FIRE current in milliamps or query error
    ///
    /// # Example
    /// ```
    /// let fire_current = device.read_fire_current()?;
    /// ```
    pub fn read_fire_current(&mut self) -> Result<u16> {
        device_operations::readback::read_fire_current(&mut self.protocol)
    }

    /// Set ARM current value
    /// 
    /// Sets the ARM current value for the device.
    /// 
    /// # Arguments
    /// * `current_ma` - The ARM current value in milliamps
    /// 
    /// # Returns
    /// * `Result<()>` - Success or setting error
    /// 
    /// # Example
    /// ```
    /// device.set_arm_current(1500)?;
    /// ```
    pub fn set_arm_current(&mut self, current_ma: u16) -> Result<()> {
        device_operations::readback::set_arm_current(&mut self.protocol, current_ma)
    }

    /// Get complete stage parameters
    /// 
    /// Retrieves comprehensive parameters for the specified stage.
    /// 
    /// # Arguments
    /// * `stage_num` - The stage number to query (1-5)
    /// 
    /// # Returns
    /// * `Result<operations::power::StageParameters>` - Stage parameters or query error
    /// 
    /// # Example
    /// ```
    /// let params = device.get_stage_parameters(1)?;
    /// ```
    pub fn get_stage_parameters(&mut self, stage_num: u8) -> Result<device_operations::power::StageParameters> {
        device_operations::power::get_stage_parameters(&mut self.protocol, stage_num)
    }

    /// Get ARM current for specific stage
    /// 
    /// Retrieves the ARM current setting for the specified stage.
    /// 
    /// # Arguments
    /// * `stage_num` - The stage number to query (1-5)
    /// 
    /// # Returns
    /// * `Result<u16>` - Stage ARM current in milliamps or query error
    /// 
    /// # Example
    /// ```
    /// let arm_current = device.get_stage_arm_current(2)?;
    /// ```
    pub fn get_stage_arm_current(&mut self, stage_num: u8) -> Result<u16> {
        device_operations::power::get_stage_arm_current(&mut self.protocol, stage_num)
    }

    /// Get voltage limit for specific stage
    /// 
    /// Retrieves the voltage limit setting for the specified stage.
    /// 
    /// # Arguments
    /// * `stage_num` - The stage number to query (1-5)
    /// 
    /// # Returns
    /// * `Result<f32>` - Stage voltage limit in volts or query error
    /// 
    /// # Example
    /// ```
    /// let volt_limit = device.get_stage_volt_limit(3)?;
    /// ```
    pub fn get_stage_volt_limit(&mut self, stage_num: u8) -> Result<f32> {
        device_operations::power::get_stage_volt_limit(&mut self.protocol, stage_num)
    }

    /// Get voltage start for specific stage
    /// 
    /// Retrieves the voltage start setting for the specified stage.
    /// 
    /// # Arguments
    /// * `stage_num` - The stage number to query (1-5)
    /// 
    /// # Returns
    /// * `Result<f32>` - Stage voltage start in volts or query error
    /// 
    /// # Example
    /// ```
    /// let volt_start = device.get_stage_volt_start(4)?;
    /// ```
    pub fn get_stage_volt_start(&mut self, stage_num: u8) -> Result<f32> {
        device_operations::power::get_stage_volt_start(&mut self.protocol, stage_num)
    }
}
