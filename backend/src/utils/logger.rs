// utils/logger.rs
use log::{info, warn};

pub fn log_info(message: &str) {
    info!("INFO: {}", message);
}

pub fn log_warning(message: &str) {
    warn!("WARNING: {}", message);
}

/// Additional component 1: Function to log error messages
pub fn log_error(message: &str) {
    log::error!("ERROR: {}", message);
}

/// Additional component 2: Function to log debug messages
pub fn log_debug(message: &str) {
    log::debug!("DEBUG: {}", message);
}

/// Example usage of the logger utilities
fn main() {
    env_logger::init(); // Initialize the logger

    log_info("This is an informational message.");
    log_warning("This is a warning message.");
    log_error("This is an error message.");
    log_debug("This is a debug message.");
}
