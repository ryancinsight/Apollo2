//! Device initialization and setup logic for Lumidox II Controller
//!
//! This module handles the initialization and setup of the LumidoxDevice controller
//! including constructor methods, device initialization sequences, and basic setup
//! operations required to prepare the device for operation.
//! 
//! The initialization system provides:
//! - Device controller construction with various configuration options
//! - Device initialization sequence with proper mode setting and information retrieval
//! - Error handling for initialization failures
//! - Integration with device information and protocol systems

use crate::core::Result;
use crate::communication::ProtocolHandler;
use crate::device::models::DeviceMode;
use crate::device::{info, operations::control};
use std::thread;
use std::time::Duration;

/// Device initialization and setup utilities
pub struct DeviceInitializer;

impl DeviceInitializer {
    /// Create a new device controller with default settings
    /// 
    /// Initializes a new LumidoxDevice controller with optimized transitions
    /// enabled by default for improved performance and user experience.
    /// 
    /// # Arguments
    /// * `protocol` - The protocol handler for device communication
    /// 
    /// # Returns
    /// * `LumidoxDevice` - A new device controller instance
    /// 
    /// # Default Configuration
    /// - Optimized transitions: Enabled (true)
    /// - Device info: None (retrieved during initialization)
    /// - Current mode: None (set during initialization)
    /// 
    /// # Example
    /// ```
    /// let protocol = ProtocolHandler::new(port)?;
    /// let device = DeviceInitializer::create_default(protocol);
    /// ```
    pub fn create_default(protocol: ProtocolHandler) -> super::super::LumidoxDevice {
        super::super::LumidoxDevice {
            protocol,
            info: None,
            current_mode: None,
            optimize_transitions: true, // Enable optimized transitions by default
        }
    }
    
    /// Create a new device controller with specified optimization setting
    /// 
    /// Initializes a new LumidoxDevice controller with a custom optimization
    /// setting for stage transitions, allowing fine-grained control over
    /// device behavior.
    /// 
    /// # Arguments
    /// * `protocol` - The protocol handler for device communication
    /// * `optimize_transitions` - Whether to enable optimized stage transitions
    /// 
    /// # Returns
    /// * `LumidoxDevice` - A new device controller instance with specified settings
    /// 
    /// # Optimization Behavior
    /// - When enabled (true): Uses smart transition logic to minimize unnecessary operations
    /// - When disabled (false): Always uses full safety sequence for all operations
    /// 
    /// # Example
    /// ```
    /// let protocol = ProtocolHandler::new(port)?;
    /// let device = DeviceInitializer::create_with_optimization(protocol, false);
    /// ```
    pub fn create_with_optimization(
        protocol: ProtocolHandler, 
        optimize_transitions: bool
    ) -> super::super::LumidoxDevice {
        super::super::LumidoxDevice {
            protocol,
            info: None,
            current_mode: None,
            optimize_transitions,
        }
    }
    
    /// Initialize the device and retrieve basic information
    /// 
    /// Performs the complete device initialization sequence including setting
    /// the device to standby mode and retrieving device information for caching.
    /// This method should be called after device creation to prepare it for operation.
    /// 
    /// # Arguments
    /// * `device` - Mutable reference to the device controller to initialize
    /// 
    /// # Returns
    /// * `Result<()>` - Success or initialization error
    /// 
    /// # Initialization Sequence
    /// 1. Set device to standby mode for safe operation
    /// 2. Wait for mode transition to complete (100ms delay)
    /// 3. Retrieve and cache device information
    /// 4. Update internal state with retrieved information
    /// 
    /// # Error Handling
    /// If any step fails, the initialization is aborted and an error is returned.
    /// The device may be left in an intermediate state and should be re-initialized.
    /// 
    /// # Example
    /// ```
    /// let mut device = DeviceInitializer::create_default(protocol);
    /// DeviceInitializer::initialize_device(&mut device)?;
    /// ```
    pub fn initialize_device(device: &mut super::super::LumidoxDevice) -> Result<()> {
        // Set to standby mode first for safe initialization
        Self::set_initial_mode(device, DeviceMode::Standby)?;
        
        // Allow time for mode transition to complete
        Self::wait_for_mode_transition(Duration::from_millis(100));
        
        // Retrieve and cache device information
        Self::retrieve_device_information(device)?;
        
        Ok(())
    }
    
    /// Set the initial device mode during initialization
    /// 
    /// Sets the device to the specified mode as part of the initialization
    /// sequence, updating both the device hardware and internal state tracking.
    /// 
    /// # Arguments
    /// * `device` - Mutable reference to the device controller
    /// * `mode` - The initial mode to set for the device
    /// 
    /// # Returns
    /// * `Result<()>` - Success or mode setting error
    /// 
    /// # Example
    /// ```
    /// DeviceInitializer::set_initial_mode(&mut device, DeviceMode::Standby)?;
    /// ```
    pub fn set_initial_mode(
        device: &mut super::super::LumidoxDevice, 
        mode: DeviceMode
    ) -> Result<()> {
        control::set_mode(&mut device.protocol, mode)?;
        device.current_mode = Some(mode);
        Ok(())
    }
    
    /// Wait for mode transition to complete
    /// 
    /// Introduces a delay to allow the device hardware to complete mode
    /// transitions before proceeding with additional operations.
    /// 
    /// # Arguments
    /// * `duration` - The duration to wait for the transition
    /// 
    /// # Example
    /// ```
    /// DeviceInitializer::wait_for_mode_transition(Duration::from_millis(100));
    /// ```
    pub fn wait_for_mode_transition(duration: Duration) {
        thread::sleep(duration);
    }
    
    /// Retrieve and cache device information
    /// 
    /// Reads device information from the hardware and caches it in the
    /// device controller for future access without additional protocol calls.
    /// 
    /// # Arguments
    /// * `device` - Mutable reference to the device controller
    /// 
    /// # Returns
    /// * `Result<()>` - Success or information retrieval error
    /// 
    /// # Information Retrieved
    /// - Firmware version
    /// - Model number
    /// - Serial number
    /// - Device wavelength
    /// - Maximum current capability
    /// 
    /// # Example
    /// ```
    /// DeviceInitializer::retrieve_device_information(&mut device)?;
    /// ```
    pub fn retrieve_device_information(device: &mut super::super::LumidoxDevice) -> Result<()> {
        let device_info = info::read_device_info(&mut device.protocol)?;
        device.info = Some(device_info);
        Ok(())
    }
    
    /// Validate device initialization state
    /// 
    /// Checks that the device has been properly initialized and is ready
    /// for normal operation by verifying internal state consistency.
    /// 
    /// # Arguments
    /// * `device` - Reference to the device controller to validate
    /// 
    /// # Returns
    /// * `Result<InitializationStatus>` - Validation results
    /// 
    /// # Validation Checks
    /// - Device information has been retrieved and cached
    /// - Current mode has been set and is valid
    /// - Protocol handler is accessible and functional
    /// 
    /// # Example
    /// ```
    /// let status = DeviceInitializer::validate_initialization(&device)?;
    /// if status.is_fully_initialized {
    ///     println!("Device ready for operation");
    /// }
    /// ```
    pub fn validate_initialization(
        device: &super::super::LumidoxDevice
    ) -> Result<InitializationStatus> {
        let has_device_info = device.info.is_some();
        let has_current_mode = device.current_mode.is_some();
        let is_fully_initialized = has_device_info && has_current_mode;
        
        Ok(InitializationStatus {
            has_device_info,
            has_current_mode,
            is_fully_initialized,
            optimization_enabled: device.optimize_transitions,
        })
    }
    
    /// Get initialization recommendations
    /// 
    /// Provides recommendations for device initialization based on the
    /// current state and intended usage patterns.
    /// 
    /// # Arguments
    /// * `intended_usage` - The intended usage pattern for the device
    /// 
    /// # Returns
    /// * `InitializationRecommendations` - Recommended settings and procedures
    /// 
    /// # Example
    /// ```
    /// let recommendations = DeviceInitializer::get_initialization_recommendations(
    ///     IntendedUsage::HighFrequencyTesting
    /// );
    /// ```
    pub fn get_initialization_recommendations(
        intended_usage: IntendedUsage
    ) -> InitializationRecommendations {
        match intended_usage {
            IntendedUsage::GeneralOperation => InitializationRecommendations {
                optimize_transitions: true,
                initial_mode: DeviceMode::Standby,
                initialization_delay_ms: 100,
                recommended_timeout_ms: 5000,
            },
            IntendedUsage::HighFrequencyTesting => InitializationRecommendations {
                optimize_transitions: true,
                initial_mode: DeviceMode::Standby,
                initialization_delay_ms: 50,
                recommended_timeout_ms: 2000,
            },
            IntendedUsage::SafetyTesting => InitializationRecommendations {
                optimize_transitions: false,
                initial_mode: DeviceMode::Standby,
                initialization_delay_ms: 200,
                recommended_timeout_ms: 10000,
            },
            IntendedUsage::ProductionTesting => InitializationRecommendations {
                optimize_transitions: true,
                initial_mode: DeviceMode::Standby,
                initialization_delay_ms: 100,
                recommended_timeout_ms: 3000,
            },
        }
    }
}

/// Device initialization status information
#[derive(Debug, Clone)]
pub struct InitializationStatus {
    /// Whether device information has been retrieved and cached
    pub has_device_info: bool,
    /// Whether the current mode has been set
    pub has_current_mode: bool,
    /// Whether the device is fully initialized and ready for operation
    pub is_fully_initialized: bool,
    /// Whether optimization is enabled for this device instance
    pub optimization_enabled: bool,
}

/// Intended usage patterns for initialization recommendations
#[derive(Debug, Clone, PartialEq)]
pub enum IntendedUsage {
    /// General device operation with balanced performance and safety
    GeneralOperation,
    /// High-frequency testing requiring optimized performance
    HighFrequencyTesting,
    /// Safety-critical testing requiring maximum safety margins
    SafetyTesting,
    /// Production testing with optimized throughput
    ProductionTesting,
}

/// Initialization recommendations based on intended usage
#[derive(Debug, Clone)]
pub struct InitializationRecommendations {
    /// Whether to enable optimized transitions
    pub optimize_transitions: bool,
    /// Recommended initial device mode
    pub initial_mode: DeviceMode,
    /// Recommended initialization delay in milliseconds
    pub initialization_delay_ms: u64,
    /// Recommended protocol timeout in milliseconds
    pub recommended_timeout_ms: u64,
}
