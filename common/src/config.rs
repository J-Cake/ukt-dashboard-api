use std::net::{IpAddr, SocketAddr};
use std::path::PathBuf;
use std::time::Duration;
use serde::Serialize;
use serde::Deserialize;

#[derive(Debug, Clone, clap::Parser)]
pub struct CliArgs {
    #[clap(short, long, default_value = "config.toml")]
    pub config: PathBuf,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Config {
    pub bind: BindConfig,
    pub weather: WeatherConfig,
    pub departure: DepartureConfig,

    #[serde(skip)]
    pub source: Option<PathBuf>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BindConfig {
    pub socket: SocketAddr
}

impl Default for BindConfig {
    fn default() -> Self {
        Self {
            socket: SocketAddr::from(SocketAddr::new(IpAddr::from([127, 0, 0, 1]), 1920))
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct DepartureConfig {
    pub point: Option<String>,
    pub lines: Vec<String>,

    #[serde(default = "default_show_next_time_window")]
    pub show_next: Duration
}

fn default_show_next_time_window() -> Duration { Duration::from_hours(1) }