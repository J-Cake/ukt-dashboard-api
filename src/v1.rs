use std::collections::HashMap;
use std::sync::LazyLock;
use std::time::{Duration, SystemTime};
use crate::Result;
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use crate::cache::{Cache};
use crate::config::Config;

pub fn v1() -> actix_web::Scope {
    actix_web::web::scope("/v1")
        .service(config)
        .service(forecast)
}

#[actix_web::get("/config")]
pub async fn config(config: web::Data<Config>) -> Result<impl Responder> {
    Ok(HttpResponse::Ok()
        .json(config))
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct ForecastParams {
    #[serde(default = "default_forecast_days")]
    days: u16
}

#[inline]
const fn default_forecast_days() -> u16 { 7 }

static WEATHER_CACHE: Cache<ForecastParams, WeatherState> = Cache::new(Duration::from_secs(30));

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeatherState {
    pub time: SystemTime,
    pub response: serde_json::Value
}

#[actix_web::get("/forecast")]
pub async fn forecast(newer_than: Option<actix_web::web::Header<actix_web::http::header::Date>>, query: web::Query<ForecastParams>) -> Result<impl Responder> {
    if let Some(newer_than) = newer_than {
        let newer_than = SystemTime::from(newer_than.0.0);

        if WEATHER_CACHE.get_last_modified_time(&query.0).await.is_some_and(|modified| modified < newer_than) {
            return Ok(HttpResponse::NotModified()
                .finish());
        }
    }

    let weather = WEATHER_CACHE.get(query.0, async || {
        Ok(WeatherState {
            time: SystemTime::now(),
            response: serde_json::json! {{
                "Hello": SystemTime::now()
            }},
        })
    }).await?;

    Ok(HttpResponse::Ok()
        .json(weather))
}