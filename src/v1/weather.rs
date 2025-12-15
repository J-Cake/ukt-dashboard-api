use crate::cache::Cache;
use crate::v1::weather_schema::PresentWeather;
use crate::v1::weather_schema::WeatherDay;
use crate::v1::weather_schema::WeatherResponse;
use crate::v1::weather_schema::WeatherSchema;
use crate::v1::weather_schema::WeatherState;
use crate::Result;
use actix_web::web;
use actix_web::HttpResponse;
use actix_web::Responder;
use common::config::Config;
use common::config::WeatherConfig;
use common::prelude::*;
use reqwest::Url;
use serde::Deserialize;
use serde::Serialize;
use std::io;
use std::io::Error;
use std::time::Duration;
use std::time::SystemTime;

const WEATHER_API: &'static str = "https://api.open-meteo.com/v1/forecast";
const CITY_NAME_API: &'static str = "https://api.bigdatacloud.net/data/reverse-geocode-client";

#[derive(Debug, Default, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct ForecastParams {
    days: Option<u16>,
}

static WEATHER_CACHE: Cache<ForecastParams, WeatherState> = Cache::new(Duration::from_secs(30));
static CITY_NAME: Cache<Coordinate, String> = Cache::new(Duration::from_hours(12));
type Coordinate = String;

#[actix_web::get("/forecast")]
pub async fn forecast(newer_than: Option<web::Header<actix_web::http::header::Date>>, query: web::Query<ForecastParams>, cfg: web::Data<Config>) -> Result<impl Responder> {
    if let Some(newer_than) = newer_than {
        let newer_than = SystemTime::from(newer_than.0.0);

        if WEATHER_CACHE.get_last_modified_time(&query.0).await.is_some_and(|modified| modified < newer_than) {
            return Ok(HttpResponse::NotModified().finish());
        }
    }

    let weather = WEATHER_CACHE.get(query.0.clone(), async || {
        let api = match reqwest::ClientBuilder::new().build() {
            Ok(client) => client,
            Err(err) => return Err(io::Error::other(err)),
        };

        let mut uri =
            Url::parse(WEATHER_API).map_err(io::Error::other)?;

        let query = serde_qs::to_string(&WeatherConfig {
            forecast_days: query.days.or(cfg.weather.forecast_days),
            config: serde_json::json! {{
                "daily": ["weather_code", "temperature_2m_max", "temperature_2m_min", "precipitation_sum", "wind_speed_10m_max"],
                "current": ["temperature_2m", "relative_humidity_2m", "precipitation", "weather_code", "wind_speed_10m", "is_day"]
            }},
            ..cfg.weather.clone()
        })
            .map_err(io::Error::other)?;

        uri.set_query(Some(&query));

        log::debug!("URI: {uri:?}", uri = uri.to_string());

        let req = match api.get(uri).header("Accept", "application/json").send().await {
            Ok(req) => req,
            Err(err) => {
                log::error!("Reqwest Error: {err:?}");
                return Err(std::io::Error::other(err));
            }
        };

        match req.json::<WeatherSchema>().await {
            Ok(res) => convert_to_weather_state(res).await
                .ok_or(Error::other("Not all data was received")),
            Err(err) => {
                log::error!("Response Error: {err:?}");
                return Err(std::io::Error::other(err));
            }
        }
    }).await?;

    Ok(HttpResponse::Ok().json(weather))
}

async fn convert_to_weather_state(incoming: WeatherSchema) -> Option<WeatherState> {
    // &current=temperature_2m,relative_humidity_2m,precipitation,weather_code,wind_speed_10m,is_day,relative_humidity_2m
    let current = WeatherDay {
        temperature: incoming.current.get("temperature_2m")?.as_f64()?,
        wind_speed: incoming.current.get("wind_speed_10m")?.as_f64()?,
        precipitation: incoming.current.get("precipitation")?.as_f64()?,
        humidity: incoming.current.get("relative_humidity_2m")?.as_f64()?,
        weather: PresentWeather::from_code(incoming.current.get("weather_code")?.as_u64()? as u8)?,
        code: incoming.current.get("weather_code")?.as_u64()?,
    };

    // &daily=weather_code,temperature_2m_max,temperature_2m_min,precipitation_sum,wind_speed_10m_max,relative_humidity_2m
    let daily = (0..incoming.daily.get("weather_code")?.as_array()?.len())
        .map(|a| {
            Some(WeatherDay {
                precipitation: incoming.daily.get("precipitation_sum")?
                    .as_array()?
                    .get(a)
                    .and_then(|i| i.as_f64())
                    .unwrap_or_default(),
                humidity: f64::NEG_INFINITY,
                wind_speed: incoming.daily.get("wind_speed_10m_max")?
                    .as_array()?
                    .get(a)
                    .and_then(|i| i.as_f64())
                    .unwrap_or_default(),
                temperature: (incoming.daily.get("temperature_2m_min")?
                    .as_array()?
                    .get(a)
                    .and_then(|i| i.as_f64())
                    .unwrap_or_default() + incoming.daily.get("temperature_2m_max")?
                    .as_array()?
                    .get(a)
                    .and_then(|i| i.as_f64())
                    .unwrap_or_default()) / 2.0,
                weather: PresentWeather::from_code(incoming.daily.get("weather_code")?
                    .as_array()?
                    .get(a)
                    .and_then(|i| i.as_f64())
                    .unwrap_or_default() as u8,
                )?,
                code: incoming.daily.get("weather_code")?
                    .as_array()?
                    .get(a)
                    .and_then(|i| i.as_u64())
                    .unwrap_or_default(),
            })
        })
        .collect::<Option<Vec<_>>>()?;

    Some(WeatherState {
        time: SystemTime::now(),
        city: get_city_name(incoming.latitude, incoming.longitude)
            .await
            .ok()?,
        response: WeatherResponse {
            is_day: incoming.current.get("is_day")?.as_u64()? == 1,

            current,
            daily,
        },
    })
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct LatLongCityResolverQueryString {
    latitude: f64,
    longitude: f64,
    locality_language: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CityResponse {
    latitude: f64,
    longitude: f64,
    continent: String,
    continent_code: String,
    country_name: String,
    country_code: String,
    city: String,
    locality: String,
    postcode: String,
    plus_code: String,
}

async fn get_city_name(lat: f64, long: f64) -> Result<String> {
    let qs = serde_qs::to_string(&(lat, long)).map_err(Error::other)?;

    CITY_NAME
        .get(qs.clone(), async || {
            let api = match reqwest::ClientBuilder::new().build() {
                Ok(client) => client,
                Err(err) => return Err(io::Error::other(err)),
            };

            let mut uri = Url::parse(CITY_NAME_API).map_err(io::Error::other)?;

            let query = serde_qs::to_string(&LatLongCityResolverQueryString {
                latitude: lat,
                longitude: long,
                locality_language: "default".into(),
            })
            .map_err(io::Error::other)?;

            uri.set_query(Some(&query));

            match api.get(uri).header("Accept", "application/json").send().await {
                Ok(req) => match req.json::<CityResponse>().await {
                    Ok(res) => {
                        log::debug!("Resolved to city: {city}, {locality}", city=res.city, locality=res.locality);
                        Ok(format!("{city}, {locality}", city = res.city, locality = res.locality))
                    },
                    Err(err) => {
                        log::error!("Response Error: {err:?}");
                        return Err(std::io::Error::other(err));
                    }
                },
                Err(err) => {
                    log::error!("Reqwest Error: {err:?}");
                    return Err(std::io::Error::other(err));
                }
            }
        })
        .await
}
