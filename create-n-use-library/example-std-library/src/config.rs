//! This config module contains the application configuration options
//! 


pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

pub enum OutputLog{
    Stdout,
    Stderr,
    File(String),
}

/// This struct contains the application configuration options for logging
/// # Examples:
/// ```
/// use examplestdlibrary::config::ConfigLogging;
/// let config = ConfigLogging::new();
/// ```
/// 
/// Logging with custom values
/// # Examples:
/// ```
/// use examplestdlibrary::config::{ConfigLogging, LogLevel, OutputLog};
/// let config = ConfigLogging{
///    enabled: true,
///   log_level: LogLevel::Debug,
///   output_log: OutputLog::File("log.txt".to_string()),
/// };
/// ```
pub struct ConfigLogging {
    pub enabled: bool,
    pub log_level: LogLevel,
    pub output_log: OutputLog,
}

impl ConfigLogging{
    pub fn new() -> ConfigLogging {
        ConfigLogging {
            enabled: false,
            log_level: LogLevel::Info,
            output_log: OutputLog::Stdout,
        }
    }
}
