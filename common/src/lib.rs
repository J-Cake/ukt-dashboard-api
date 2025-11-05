use std::io;
use std::path::PathBuf;
use crate::config::{CliArgs, Config};

pub mod config;
pub type Result<T> = core::result::Result<T, std::io::Error>;

pub mod prelude {
    pub use crate::config::*;
    pub use super::Result;

    pub use tokio;
    pub use toml;
    pub use serde_json;
    pub use clap;
}

pub async fn get_config() -> Result<Config> {
    let args: CliArgs = clap::Parser::parse();
    let mut config: Config = tokio::fs::read_to_string(&args.config)
        .await
        .and_then(|config| toml::from_str(&config).map_err(io::Error::other))?;

    config.source = Some(PathBuf::from(&args.config));

    Ok(config)
}