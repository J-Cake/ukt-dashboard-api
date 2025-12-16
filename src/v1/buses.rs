use crate::v1::buses_schema::BusSchema;
use crate::v1::buses_schema::DepartureBoardStop;
use crate::v1::buses_schema::RealDateTimeClass;
use crate::Result;
use actix_web::HttpResponse;
use actix_web::Responder;
use chrono::Local;
use chrono::TimeZone;
use common::config::Config;
use common::prelude::tokio::task::JoinSet;
use common::prelude::DepartureConfig;
use reqwest::Url;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::io::Error;
use std::time::{Duration, SystemTime};
use crate::cache::Cache;

const BUS_API: &'static str = "https://www.efa-bw.de/mobidata-bw/XML_DM_REQUEST";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Query {
    #[serde(rename = "outputFormat")]
    output_format: &'static str,

    mode: &'static str,

    #[serde(rename = "useProxFootSearch")]
    include_proximate: u8,

    #[serde(rename = "name_dm")]
    stop_id: String,

    // #[serde(rename = "itdDate")]
    // date: String,
    //
    // #[serde(rename = "itdTime")]
    // time: String,
    limit: usize,

    #[serde(rename = "itdDateTimeDepArr")]
    depart_or_arrive: &'static str,

    type_dm: &'static str,
}

impl Default for Query {
    fn default() -> Self {
        Self {
            output_format: "JSON",
            mode: "direct",
            include_proximate: 1,
            stop_id: "".to_string(),
            limit: 50,
            depart_or_arrive: "dep",
            type_dm: "any",
        }
    }
}

impl Query {
    fn with_limit(mut self, limit: usize) -> Self {
        self.limit = limit;
        return self;
    }

    fn with_stop(mut self, stop: impl AsRef<str>) -> Self {
        self.stop_id = stop.as_ref().to_string();
        return self;
    }
}

impl From<&Vec<DepartureConfig>> for Query {
    fn from(value: &Vec<DepartureConfig>) -> Self {
        Self {
            stop_id: value.iter().map(|i| &i.point).cloned().collect(),
            include_proximate: 0,
            ..Default::default()
        }
    }
}

static DEPARTURES: Cache<String, Vec<DepartureBoardStop>> = Cache::new(Duration::from_secs(10));

#[actix_web::get("/buses")]
pub async fn buses(cfg: actix_web::web::Data<Config>) -> Result<impl Responder> {
    let mut tasks = JoinSet::new();

    for stop in cfg.departure.iter().map(|i| i.point.clone()) {

        tasks.spawn(async move {
            (stop.clone(), DEPARTURES.get(stop.clone(), async || {
                get_times(stop.clone()).await
            }).await)
        });
    }

    let stops = tasks.join_all().await
        .into_iter()
        .map(|(stop, list)| list.map(|i| (stop, i)))
        .collect::<Result<HashMap<String, Vec<DepartureBoardStop>>>>()?;

    Ok(HttpResponse::Ok().json(serde_json::json! {{
        "time": SystemTime::now(),
        "stops": stops
    }}))
}

fn parse_date_time(date: RealDateTimeClass) -> Result<chrono::DateTime<Local>> {
    let (yyyy, MM, dd, hh, mm) = (
        date.year.parse::<i32>().map_err(Error::other)?,
        date.month.parse::<u32>().map_err(Error::other)?,
        date.day.parse::<u32>().map_err(Error::other)?,
        date.hour.parse::<u32>().map_err(Error::other)?,
        date.minute.parse::<u32>().map_err(Error::other)?,
    );

    let date = chrono::NaiveDate::from_ymd_opt(yyyy, MM, dd);
    let time = chrono::NaiveTime::from_hms_opt(hh, mm, 0);

    let datetime = date
        .and_then(|date| Some(date.and_time(time?)))
        .ok_or(Error::other("No time provided"))?;

    // log::trace!("Assuming Timezone: {tz}", tz=Local::now().offset());

    Ok(Local.from_local_datetime(&datetime).unwrap())
}

async fn get_times(stop: String) -> Result<Vec<DepartureBoardStop>> {
    let mut uri = Url::parse(BUS_API).map_err(Error::other)?;
    let client = reqwest::ClientBuilder::new()
        .build()
        .map_err(Error::other)?;

    let query = serde_qs::to_string(&Query {
        stop_id: stop.to_string(),
        ..Default::default()
    })
    .map_err(Error::other)?;

    uri.set_query(Some(&query));

    log::debug!("Bus URL: {uri:?}", uri = uri.to_string());

    let req = client.get(uri).send().await.expect("Failed to get buses");

    let res: BusSchema = req.json().await.expect("Failed to get buses");

    res.departure_list
        .iter()
        .map(|line| {
            let eta = parse_date_time(
                line.real_date_time
                    .clone()
                    .unwrap_or(line.date_time.clone()),
            )
            .map_err(Error::other)?;

            let given_eta = parse_date_time(
                line.date_time
                    .clone(),
            )
            .map_err(Error::other)?;

            Ok(DepartureBoardStop {
                stop: line.stop_name.clone(),
                line: line.serving_line.symbol.to_string(),
                direction: line.serving_line.direction.to_string(),
                expected_arrival: eta,
                given_arrival: given_eta
            })
        })
        .collect()
}
