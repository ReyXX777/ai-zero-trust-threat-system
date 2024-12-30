// config.rs
use config::{Config, ConfigError, File, Environment};
use serde::Deserialize;
use std::env;

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

        // Load the default configuration file (config/default.toml)
        cfg.merge(File::with_name("config/default"))?;

        // Optionally, load an environment-specific configuration file if RUN_MODE is set
        if let Ok(env) = env::var("RUN_MODE") {
            // Load the environment-specific config (e.g., config/dev.toml)
            cfg.merge(File::with_name(&format!("config/{}", env)).required(false))?;
        }

        // Optionally, override with environment variables prefixed with "APP_"
        // (e.g., APP_DATABASE_URL, APP_KAFKA_BROKERS, etc.)
        cfg.merge(Environment::with_prefix("APP").separator("_"))?;

        // Deserialize into the Settings struct
        cfg.try_into()
    }
}

