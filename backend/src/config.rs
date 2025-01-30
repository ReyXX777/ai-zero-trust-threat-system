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

    /// Additional component 1: Function to validate the configuration settings
    pub fn validate(&self) -> Result<(), String> {
        if self.database_url.is_empty() {
            return Err("Database URL cannot be empty".to_string());
        }
        if self.kafka_brokers.is_empty() {
            return Err("Kafka brokers cannot be empty".to_string());
        }
        if self.api_port == 0 {
            return Err("API port must be a valid non-zero value".to_string());
        }
        Ok(())
    }

    /// Additional component 2: Function to get a formatted string of the configuration
    pub fn to_formatted_string(&self) -> String {
        format!(
            "Database URL: {}\nKafka Brokers: {}\nAPI Port: {}\nLog Level: {}",
            self.database_url, self.kafka_brokers, self.api_port, self.log_level
        )
    }
}

/// Example usage of the configuration utilities
fn main() {
    match Settings::new() {
        Ok(settings) => {
            println!("Configuration loaded successfully:");
            println!("{}", settings.to_formatted_string());

            if let Err(err) = settings.validate() {
                println!("Configuration validation failed: {}", err);
            } else {
                println!("Configuration is valid.");
            }
        }
        Err(err) => println!("Failed to load configuration: {}", err),
    }
}
