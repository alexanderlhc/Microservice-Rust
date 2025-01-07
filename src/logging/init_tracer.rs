use tracing_subscriber::{fmt, prelude::*, EnvFilter};

use crate::settings::LogLevel;

pub fn setup_logging(level: &LogLevel) -> Result<(), String> {
    let filter_directive = match level {
        LogLevel::Error => "error",
        LogLevel::Warn => "warn",
        LogLevel::Info => "info",
        LogLevel::Debug => "debug",
        LogLevel::Trace => "trace",
    };

    let filter = EnvFilter::try_new(filter_directive)
        .map_err(|e| format!("Failed to create log filter: {}", e))?;

    let format = fmt::layer()
        .with_target(true)
        .with_thread_ids(true)
        .with_line_number(true)
        .with_file(true)
        .with_level(true)
        .with_thread_names(true)
        .pretty();

    // Initialize the subscriber
    tracing_subscriber::registry()
        .with(format)
        .with(filter)
        .try_init()
        .map_err(|e| format!("Failed to initialize logging: {}", e))?;

    Ok(())
}
