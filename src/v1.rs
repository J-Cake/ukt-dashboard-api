use std::collections::HashMap;
use std::sync::LazyLock;
use std::io;
use std::time::{Duration, SystemTime};
use crate::Result;
use actix_web::{web, HttpResponse, Responder};
use actix_web::http::Uri;
use reqwest::Url;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use crate::cache::{Cache};
use crate::config::{Config, WeatherConfig};

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

#[derive(Debug, Default, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct ForecastParams {
    days: Option<u16>
}

static WEATHER_CACHE: Cache<ForecastParams, WeatherState> = Cache::new(Duration::from_secs(30));

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeatherState {
    pub time: SystemTime,
    pub response: serde_json::Value
}

#[actix_web::get("/forecast")]
pub async fn forecast(newer_than: Option<actix_web::web::Header<actix_web::http::header::Date>>, query: web::Query<ForecastParams>, cfg: web::Data<Config>) -> Result<impl Responder> {
    if let Some(newer_than) = newer_than {
        let newer_than = SystemTime::from(newer_than.0.0);

        if WEATHER_CACHE.get_last_modified_time(&query.0).await.is_some_and(|modified| modified < newer_than) {
            return Ok(HttpResponse::NotModified()
                .finish());
        }
    }

    let weather = WEATHER_CACHE.get(query.0.clone(), async || {
        let api = match reqwest::ClientBuilder::new()
            .build() {
            Ok(client) => client,
            Err(err) => return Err(io::Error::other(err))
        };

        let mut uri = Url::parse("https://api.open-meteo.com/v1/forecast")
            .map_err(io::Error::other)?;

        let query = serde_qs::to_string(&WeatherConfig {
            forecast_days: query.days.or(cfg.weather.forecast_days),
            ..cfg.weather.clone()
        })
            .map_err(io::Error::other)?;

        uri.set_query(Some(&query));

        log::debug!("URI: {uri:?}", uri=uri.to_string());

        let req = match api.get(uri)
            .header("Accept", "application/json")
            .send()
            .await {
            Ok(req) => req,
            Err(err) => {
                log::error!("Reqwest Error: {err:?}");
                return Err(std::io::Error::other(err));
            }
        };

        let res = match req.json::<serde_json::Value>().await {
            Ok(res) => res,
            Err(err) => {
                log::error!("Response Error: {err:?}");
                return Err(std::io::Error::other(err))
            }
        };

        log::debug!("weather: {res:#?}");

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