//! System and I/O error handling utilities
//!
//! This module provides specialized error handling utilities and documentation
//! for system-level, I/O, and configuration errors in the Lumidox II Controller.
//! 
//! System errors typically occur during:
//! - File system operations (reading/writing configuration files)
//! - Application initialization and configuration
//! - Environment setup and validation
//! - Resource allocation and management
//! - Operating system interaction

use super::types::LumidoxError;

/// System error categories for better error classification
/// 
/// This enum helps categorize different types of system errors
/// for more specific error handling and user feedback.
#[derive(Debug, Clone, PartialEq)]
pub enum SystemErrorCategory {
    /// File system I/O errors
    FileSystem,
    /// Configuration file or settings errors
    Configuration,
    /// Environment or system setup errors
    Environment,
    /// Resource allocation or permission errors
    Resource,
    /// Application initialization errors
    Initialization,
}

/// System error utilities and helper functions
pub struct SystemErrorUtils;

impl SystemErrorUtils {
    /// Create a file system operation error
    /// 
    /// Used when file system operations fail.
    /// 
    /// # Arguments
    /// * `operation` - The file operation that failed (e.g., "read", "write", "create")
    /// * `file_path` - The path to the file that caused the error
    /// * `details` - Specific details about the failure
    /// 
    /// # Returns
    /// * `LumidoxError::ConfigError` - Formatted file system error
    /// 
    /// # Example
    /// ```
    /// let error = SystemErrorUtils::file_system_error("read", "/config/settings.toml", "file not found");
    /// ```
    pub fn file_system_error(operation: &str, file_path: &str, details: &str) -> LumidoxError {
        LumidoxError::ConfigError(format!(
            "File system error during {} operation on '{}': {}", 
            operation, file_path, details
        ))
    }
    
    /// Create a configuration file error
    /// 
    /// Used when configuration files cannot be loaded or parsed.
    /// 
    /// # Arguments
    /// * `config_file` - The configuration file that caused the error
    /// * `issue` - Description of the configuration issue
    /// 
    /// # Returns
    /// * `LumidoxError::ConfigError` - Formatted configuration error
    /// 
    /// # Example
    /// ```
    /// let error = SystemErrorUtils::configuration_error("device_settings.toml", "invalid TOML syntax on line 15");
    /// ```
    pub fn configuration_error(config_file: &str, issue: &str) -> LumidoxError {
        LumidoxError::ConfigError(format!(
            "Configuration error in '{}': {}", 
            config_file, issue
        ))
    }
    
    /// Create an environment setup error
    /// 
    /// Used when the application environment is not properly configured.
    /// 
    /// # Arguments
    /// * `component` - The environment component that is misconfigured
    /// * `requirement` - What is required for proper operation
    /// 
    /// # Returns
    /// * `LumidoxError::ConfigError` - Formatted environment error
    /// 
    /// # Example
    /// ```
    /// let error = SystemErrorUtils::environment_error("PATH variable", "must include system32 directory");
    /// ```
    pub fn environment_error(component: &str, requirement: &str) -> LumidoxError {
        LumidoxError::ConfigError(format!(
            "Environment setup error for {}: {}", 
            component, requirement
        ))
    }
    
    /// Create a resource allocation error
    /// 
    /// Used when system resources cannot be allocated or accessed.
    /// 
    /// # Arguments
    /// * `resource` - The resource that couldn't be allocated
    /// * `reason` - The reason allocation failed
    /// 
    /// # Returns
    /// * `LumidoxError::ConfigError` - Formatted resource allocation error
    /// 
    /// # Example
    /// ```
    /// let error = SystemErrorUtils::resource_error("serial port COM3", "access denied - port in use");
    /// ```
    pub fn resource_error(resource: &str, reason: &str) -> LumidoxError {
        LumidoxError::ConfigError(format!(
            "Resource allocation error for {}: {}", 
            resource, reason
        ))
    }
    
    /// Create an application initialization error
    /// 
    /// Used when the application fails to initialize properly.
    /// 
    /// # Arguments
    /// * `component` - The application component that failed to initialize
    /// * `details` - Specific details about the initialization failure
    /// 
    /// # Returns
    /// * `LumidoxError::ConfigError` - Formatted initialization error
    /// 
    /// # Example
    /// ```
    /// let error = SystemErrorUtils::initialization_error("logging system", "cannot create log directory");
    /// ```
    pub fn initialization_error(component: &str, details: &str) -> LumidoxError {
        LumidoxError::ConfigError(format!(
            "Initialization error for {}: {}", 
            component, details
        ))
    }
    
    /// Create a permission or access error
    /// 
    /// Used when operations fail due to insufficient permissions.
    /// 
    /// # Arguments
    /// * `operation` - The operation that was denied
    /// * `resource` - The resource that couldn't be accessed
    /// 
    /// # Returns
    /// * `LumidoxError::ConfigError` - Formatted permission error
    /// 
    /// # Example
    /// ```
    /// let error = SystemErrorUtils::permission_error("write configuration", "/etc/lumidox/config.toml");
    /// ```
    pub fn permission_error(operation: &str, resource: &str) -> LumidoxError {
        LumidoxError::ConfigError(format!(
            "Permission denied: cannot {} for resource '{}'", 
            operation, resource
        ))
    }
    
    /// Create a dependency missing error
    /// 
    /// Used when required system dependencies are not available.
    /// 
    /// # Arguments
    /// * `dependency` - The missing dependency
    /// * `required_for` - What the dependency is required for
    /// 
    /// # Returns
    /// * `LumidoxError::ConfigError` - Formatted dependency error
    /// 
    /// # Example
    /// ```
    /// let error = SystemErrorUtils::dependency_error("Visual C++ Redistributable", "serial port communication");
    /// ```
    pub fn dependency_error(dependency: &str, required_for: &str) -> LumidoxError {
        LumidoxError::ConfigError(format!(
            "Missing dependency '{}' required for {}", 
            dependency, required_for
        ))
    }
    
    /// Categorize a system error for better handling
    /// 
    /// Analyzes a system error message to determine its category.
    /// This can be used for implementing category-specific error handling.
    /// 
    /// # Arguments
    /// * `error_message` - The system error message to categorize
    /// 
    /// # Returns
    /// * `SystemErrorCategory` - The determined error category
    /// 
    /// # Example
    /// ```
    /// let category = SystemErrorUtils::categorize_error("File system error during read operation");
    /// assert_eq!(category, SystemErrorCategory::FileSystem);
    /// ```
    pub fn categorize_error(error_message: &str) -> SystemErrorCategory {
        let message_lower = error_message.to_lowercase();
        
        if message_lower.contains("file") || message_lower.contains("directory") || message_lower.contains("path") {
            SystemErrorCategory::FileSystem
        } else if message_lower.contains("config") || message_lower.contains("settings") || message_lower.contains("toml") {
            SystemErrorCategory::Configuration
        } else if message_lower.contains("environment") || message_lower.contains("path variable") || message_lower.contains("dependency") {
            SystemErrorCategory::Environment
        } else if message_lower.contains("resource") || message_lower.contains("permission") || message_lower.contains("access") {
            SystemErrorCategory::Resource
        } else {
            SystemErrorCategory::Initialization
        }
    }
}
