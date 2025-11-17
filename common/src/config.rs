use std::net::{IpAddr, SocketAddr};
use std::path::PathBuf;
use std::time::Duration;
use serde::Serialize;
use serde::Deserialize;
use toml::Value::Boolean;

#[derive(Debug, Clone, clap::Parser)]
pub struct CliArgs {
    #[clap(short, long, default_value = "config.toml")]
    pub config: PathBuf,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Config {
    pub bind: BindConfig,
    pub weather: WeatherConfig,
    pub departure: Vec<DepartureConfig>,

    pub app: AppConfig,

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

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub forecast_days: Option<u16>,
    
    #[serde(flatten)]
    pub config: serde_json::Value,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
}

#[inline]
const fn include_code() -> bool { true }

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct DepartureConfig {
    pub point: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppConfig {
    pub refresh_interval: Milliseconds
}

pub type Milliseconds = u32;