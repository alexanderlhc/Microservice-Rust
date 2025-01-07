use config::{Config, File};
use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Clone, Deserialize)]
pub struct Database {
    pub connection_url: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    pub db: Database,
    pub log_level: String,
}

impl Settings {
    pub fn new(config_path: Option<&str>) -> Result<Self, SettingsError> {
        let builder =
            Config::builder().add_source(File::with_name(config_path.unwrap_or("env.json")));

        Ok(builder.build()?.try_deserialize()?)
    }
}

#[derive(Debug, Error)]
pub enum SettingsError {
    #[error("Configuration error: {0}")]
    ConfigError(#[from] config::ConfigError),
}

#[derive(Debug, Deserialize, Clone)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl std::str::FromStr for LogLevel {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "error" => Ok(LogLevel::Error),
            "warn" => Ok(LogLevel::Warn),
            "info" => Ok(LogLevel::Info),
            "debug" => Ok(LogLevel::Debug),
            "trace" => Ok(LogLevel::Trace),
            _ => Err(format!("Unknown log level: {}", s)),
        }
    }
}
