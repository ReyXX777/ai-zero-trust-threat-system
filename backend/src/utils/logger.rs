// utils/logger.rs
use log::{info, warn};

pub fn log_info(message: &str) {
    info!("INFO: {}", message);
}

pub fn log_warning(message: &str) {
    warn!("WARNING: {}", message);
}
