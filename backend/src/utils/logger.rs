// utils/logger.rs
use log::{info, warn, error, debug};

/// Logs an informational message.
pub fn log_info(message: &str) {
    info!("INFO: {}", message);
}

/// Logs a warning message.
pub fn log_warning(message: &str) {
    warn!("WARNING: {}", message);
}

/// Additional component 1: Function to log error messages
pub fn log_error(message: &str) {
    error!("ERROR: {}", message);
}

/// Additional component 2: Function to log debug messages
pub fn log_debug(message: &str) {
    debug!("DEBUG: {}", message);
}

/// Additional component 3: Function to log trace messages
pub fn log_trace(message: &str) {
    log::trace!("TRACE: {}", message);
}

/// Additional component 4: Function to log messages with a custom log level
pub fn log_custom(level: log::Level, message: &str) {
    log::log!(level, "CUSTOM: {}", message);
}

/// Additional component 5: Function to log structured JSON data
pub fn log_json(data: &serde_json::Value) {
    info!("JSON DATA: {}", data);
}

/// Example usage of the logger utilities
fn main() {
    env_logger::init(); // Initialize the logger

    log_info("This is an informational message.");
    log_warning("This is a warning message.");
    log_error("This is an error message.");
    log_debug("This is a debug message.");
    log_trace("This is a trace message.");
    log_custom(log::Level::Info, "This is a custom log message.");

    let json_data = serde_json::json!({
        "key": "value",
        "number": 42
    });
    log_json(&json_data);
}
