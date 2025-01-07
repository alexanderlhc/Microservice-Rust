use clap::{command, Parser, Subcommand};
use thiserror::Error;

use crate::{
    server::{serve, ApiError},
    settings::{Settings, SettingsError},
};

#[derive(Debug, Parser)]
#[command(name = env!("CARGO_PKG_NAME"))]
pub struct Cli {
    #[arg(long)]
    pub config: Option<String>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Start {
        #[arg(long, default_value = "3000")]
        port: u16,
    },
}

impl Cli {
    pub fn initialize() -> Self {
        Cli::parse()
    }

    pub async fn handle(self, app_settings: Settings) -> Result<(), CommandError> {
        match self.command {
            Commands::Start { port } => {
                if !(1024..=65535).contains(&port) {
                    return Err(CommandError::InvalidPort(port));
                }
                serve(port, app_settings).await?;
            }
        }
        Ok(())
    }

    pub fn load_settings(&self) -> Result<Settings, CommandError> {
        Ok(Settings::new(self.config.as_deref())?)
    }
}

#[derive(Error, Debug)]
pub enum CommandError {
    #[error("Invalid port number {0}")]
    InvalidPort(u16),
    #[error("API error: {0}")]
    ApiError(#[from] ApiError),
    #[error("Settings error: {0}")]
    SettingsError(#[from] SettingsError),
}
