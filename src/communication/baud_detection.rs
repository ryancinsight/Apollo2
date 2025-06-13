//! Automated baud rate detection for Lumidox II Controller
//!
//! This module provides automated detection of the correct baud rate for
//! communicating with Lumidox II Controller devices. It tests common baud
//! rates and validates communication by sending device identification commands.
//!
//! The baud rate detection system provides:
//! - Testing of common serial communication baud rates
//! - Device identification validation at each baud rate
//! - Ranking of successful baud rates by response quality
//! - Fallback to default baud rate if detection fails

use crate::core::{LumidoxError, Result};

use std::time::Duration;

/// Baud rate detection configuration and settings
#[derive(Debug, Clone)]
pub struct BaudDetectionConfig {
    /// Timeout for each baud rate test
    pub test_timeout: Duration,
    /// List of baud rates to test in order of preference
    pub test_baud_rates: Vec<u32>,
    /// Number of identification attempts per baud rate
    pub attempts_per_rate: u8,
    /// Whether to test with multiple commands for validation
    pub comprehensive_testing: bool,
}

impl Default for BaudDetectionConfig {
    fn default() -> Self {
        Self {
            test_timeout: Duration::from_millis(1500),
            test_baud_rates: vec![
                19200,  // Default for Lumidox II
                9600,   // Common fallback
                38400,  // Double default
                57600,  // High speed option
                115200, // Very high speed
                4800,   // Low speed fallback
                2400,   // Very low speed
            ],
            attempts_per_rate: 2,
            comprehensive_testing: true,
        }
    }
}

/// Result of baud rate testing
#[derive(Debug, Clone)]
pub struct BaudTestResult {
    /// The tested baud rate
    pub baud_rate: u32,
    /// Whether communication was successful
    pub success: bool,
    /// Response quality score (0-100, higher is better)
    pub quality_score: u8,
    /// Number of successful responses out of attempts
    pub successful_responses: u8,
    /// Total number of attempts made
    pub total_attempts: u8,
    /// Details about the test results
    pub test_details: String,
    /// Device information if successfully retrieved
    pub device_info: Option<BaudTestDeviceInfo>,
}

/// Device information retrieved during baud rate testing
#[derive(Debug, Clone)]
pub struct BaudTestDeviceInfo {
    /// Firmware version if retrieved
    pub firmware_version: Option<String>,
    /// Model number if retrieved
    pub model_number: Option<String>,
    /// Whether the device responds consistently
    pub consistent_responses: bool,
}

/// Baud rate detection utilities and functionality
pub struct BaudDetector;

impl BaudDetector {
    /// Detect the correct baud rate for a port automatically
    /// 
    /// Tests multiple baud rates on the given port and returns the best
    /// working baud rate based on successful device communication.
    /// 
    /// # Arguments
    /// * `port_name` - Name of the serial port to test
    /// * `config` - Detection configuration settings
    /// 
    /// # Returns
    /// * `Result<Option<u32>>` - Best baud rate or None if none work
    /// 
    /// # Example
    /// ```
    /// let config = BaudDetectionConfig::default();
    /// if let Some(baud_rate) = BaudDetector::detect_baud_rate("COM3", &config)? {
    ///     println!("Detected baud rate: {}", baud_rate);
    /// }
    /// ```
    pub fn detect_baud_rate(port_name: &str, config: &BaudDetectionConfig) -> Result<Option<u32>> {
        let test_results = Self::test_all_baud_rates(port_name, config)?;
        
        // Find the best working baud rate
        let best_result = test_results
            .into_iter()
            .filter(|result| result.success)
            .max_by_key(|result| result.quality_score);
        
        Ok(best_result.map(|result| result.baud_rate))
    }
    
    /// Test all configured baud rates and return detailed results
    /// 
    /// Performs comprehensive testing of all baud rates in the configuration
    /// and returns detailed results for each rate tested.
    /// 
    /// # Arguments
    /// * `port_name` - Name of the serial port to test
    /// * `config` - Detection configuration settings
    /// 
    /// # Returns
    /// * `Result<Vec<BaudTestResult>>` - Results for all tested baud rates
    /// 
    /// # Example
    /// ```
    /// let config = BaudDetectionConfig::default();
    /// let results = BaudDetector::test_all_baud_rates("COM3", &config)?;
    /// for result in results {
    ///     println!("Baud {}: {} (score: {})", 
    ///         result.baud_rate, 
    ///         if result.success { "OK" } else { "FAIL" },
    ///         result.quality_score);
    /// }
    /// ```
    pub fn test_all_baud_rates(port_name: &str, config: &BaudDetectionConfig) -> Result<Vec<BaudTestResult>> {
        let mut results = Vec::new();
        
        for &baud_rate in &config.test_baud_rates {
            let result = Self::test_single_baud_rate(port_name, baud_rate, config)?;

            // If we found a high-quality match and not doing comprehensive testing, stop early
            let should_break = !config.comprehensive_testing && result.success && result.quality_score >= 80;

            results.push(result);

            if should_break {
                break;
            }
        }
        
        Ok(results)
    }
    
    /// Test a single baud rate for communication
    /// 
    /// Tests communication at a specific baud rate by attempting to send
    /// device identification commands and validating responses.
    /// 
    /// # Arguments
    /// * `port_name` - Name of the serial port to test
    /// * `baud_rate` - Baud rate to test
    /// * `config` - Detection configuration settings
    /// 
    /// # Returns
    /// * `Result<BaudTestResult>` - Test results for the baud rate
    fn test_single_baud_rate(
        port_name: &str, 
        baud_rate: u32, 
        config: &BaudDetectionConfig
    ) -> Result<BaudTestResult> {
        let mut successful_responses = 0u8;
        let mut device_info = None;
        let mut test_details = Vec::new();
        
        for attempt in 1..=config.attempts_per_rate {
            match Self::attempt_communication(port_name, baud_rate, config) {
                Ok(info) => {
                    successful_responses += 1;
                    device_info = Some(info);
                    test_details.push(format!("Attempt {}: Success", attempt));
                }
                Err(e) => {
                    test_details.push(format!("Attempt {}: Failed ({})", attempt, e));
                }
            }
        }
        
        let success = successful_responses > 0;
        let quality_score = Self::calculate_quality_score(
            successful_responses, 
            config.attempts_per_rate, 
            &device_info
        );
        
        Ok(BaudTestResult {
            baud_rate,
            success,
            quality_score,
            successful_responses,
            total_attempts: config.attempts_per_rate,
            test_details: test_details.join("; "),
            device_info,
        })
    }
    
    /// Attempt communication at a specific baud rate
    /// 
    /// Opens the port at the specified baud rate and attempts to communicate
    /// with the device using the Lumidox II protocol.
    /// 
    /// # Arguments
    /// * `port_name` - Name of the serial port
    /// * `baud_rate` - Baud rate to use
    /// * `config` - Detection configuration
    /// 
    /// # Returns
    /// * `Result<BaudTestDeviceInfo>` - Device information if successful
    fn attempt_communication(
        port_name: &str, 
        baud_rate: u32, 
        config: &BaudDetectionConfig
    ) -> Result<BaudTestDeviceInfo> {
        // Open port with the test baud rate
        let port = serialport::new(port_name, baud_rate)
            .timeout(config.test_timeout)
            .open()
            .map_err(LumidoxError::SerialError)?;
        
        // Create protocol handler
        let mut protocol = crate::communication::ProtocolHandler::new(port)?;
        
        // Test basic communication with device info command
        let device_info = crate::device::info::read_device_info(&mut protocol).ok();

        // Extract firmware and model info if available
        let firmware_version = device_info.as_ref().map(|info| info.firmware_version.clone());
        let model_number = device_info.as_ref().map(|info| info.model_number.clone());
        
        // Check consistency - if we got firmware but not model, it might be unreliable
        let consistent_responses = firmware_version.is_some() && model_number.is_some();
        
        if firmware_version.is_some() {
            Ok(BaudTestDeviceInfo {
                firmware_version,
                model_number,
                consistent_responses,
            })
        } else {
            Err(LumidoxError::DeviceError("No response to identification commands".to_string()))
        }
    }
    
    /// Calculate quality score for baud rate test results
    /// 
    /// Assigns a quality score (0-100) based on the success rate and
    /// consistency of responses at the tested baud rate.
    /// 
    /// # Arguments
    /// * `successful_responses` - Number of successful attempts
    /// * `total_attempts` - Total number of attempts made
    /// * `device_info` - Device information if available
    /// 
    /// # Returns
    /// * `u8` - Quality score (0-100)
    fn calculate_quality_score(
        successful_responses: u8, 
        total_attempts: u8, 
        device_info: &Option<BaudTestDeviceInfo>
    ) -> u8 {
        if successful_responses == 0 {
            return 0;
        }
        
        // Base score from success rate
        let success_rate = (successful_responses as f32 / total_attempts as f32) * 60.0;
        let mut score = success_rate as u8;
        
        // Bonus points for device information quality
        if let Some(info) = device_info {
            if info.firmware_version.is_some() {
                score += 15; // Got firmware version
            }
            if info.model_number.is_some() {
                score += 15; // Got model number
            }
            if info.consistent_responses {
                score += 10; // Consistent responses
            }
        }
        
        score.min(100)
    }
    
    /// Get recommended baud rate based on device type
    /// 
    /// Returns the recommended baud rate for Lumidox II devices,
    /// which can be used as a starting point for detection.
    /// 
    /// # Returns
    /// * `u32` - Recommended baud rate
    /// 
    /// # Example
    /// ```
    /// let recommended = BaudDetector::get_recommended_baud_rate();
    /// println!("Recommended baud rate: {}", recommended);
    /// ```
    pub fn get_recommended_baud_rate() -> u32 {
        crate::communication::protocol::constants::DEFAULT_BAUD_RATE
    }
    
    /// Create a quick detection configuration for fast results
    /// 
    /// Returns a configuration optimized for speed, testing only the most
    /// common baud rates with fewer attempts per rate.
    /// 
    /// # Returns
    /// * `BaudDetectionConfig` - Quick detection configuration
    /// 
    /// # Example
    /// ```
    /// let config = BaudDetector::quick_detection_config();
    /// let baud_rate = BaudDetector::detect_baud_rate("COM3", &config)?;
    /// ```
    pub fn quick_detection_config() -> BaudDetectionConfig {
        BaudDetectionConfig {
            test_timeout: Duration::from_millis(1000),
            test_baud_rates: vec![19200, 9600, 38400], // Only most common rates
            attempts_per_rate: 1, // Single attempt per rate
            comprehensive_testing: false, // Stop at first good match
        }
    }
    
    /// Create a thorough detection configuration for comprehensive testing
    /// 
    /// Returns a configuration that tests all common baud rates with multiple
    /// attempts for maximum reliability.
    /// 
    /// # Returns
    /// * `BaudDetectionConfig` - Thorough detection configuration
    /// 
    /// # Example
    /// ```
    /// let config = BaudDetector::thorough_detection_config();
    /// let results = BaudDetector::test_all_baud_rates("COM3", &config)?;
    /// ```
    pub fn thorough_detection_config() -> BaudDetectionConfig {
        BaudDetectionConfig {
            test_timeout: Duration::from_millis(2000),
            test_baud_rates: vec![
                19200, 9600, 38400, 57600, 115200, 4800, 2400, 1200
            ],
            attempts_per_rate: 3, // Multiple attempts for reliability
            comprehensive_testing: true, // Test all rates
        }
    }
}
