use std::net::SocketAddr;
use std::path::PathBuf;
use serde::Serialize;
use serde::Deserialize;

#[derive(Debug, Clone, clap::Parser)]
pub struct CliArgs {
    #[clap(short, long, default_value = "config.toml")]
    pub config: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub bind: BindConfig,

    pub weather: WeatherConfig
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BindConfig {
    pub socket: SocketAddr
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeatherConfig {
    pub latitude: f32,
    pub longitude: f32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub forecast_days: Option<u16>,
    pub daily: Vec<String>,
    pub hourly: Vec<String>,
    pub current: Vec<String>,
    pub models: Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>
}