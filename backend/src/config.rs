// config.rs
use config::{Config, ConfigError, File, Environment};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub database_url: String,
    pub kafka_brokers: String,
    pub api_port: u16,
    pub log_level: String,
}

impl Settings {
    /// Creates a new instance of Settings by loading the configuration.
    pub fn new() -> Result<Self, ConfigError> {
        // Initialize the configuration reader
        let mut cfg = Config::new();

        // Load the default configuration file
        cfg.merge(File::with_name("config/default"))?;

        // Optionally override with an environment-specific configuration
        if let Ok(env) = std::env::var("RUN_MODE") {
            cfg.merge(File::with_name(&format!("config/{}", env)).required(false))?;
        }

        // Override with environment variables prefixed with "APP_" if set
        cfg.merge(Environment::with_prefix("APP").separator("_"))?;

        // Deserialize into the Settings struct
        cfg.try_into()
    }
}
