// config.rs
use config::{Config, ConfigError, File};

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub database_url: String,
    pub kafka_brokers: String,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut cfg = Config::new();
        cfg.merge(File::with_name("config/default"))?;
        cfg.try_into()
    }
}
