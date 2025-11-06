use crate::Result;
use actix_web::HttpResponse;
use actix_web::Responder;
use chrono::Local;
use chrono::TimeZone;
use common::config::Config;
use common::prelude::DepartureConfig;
use reqwest::Url;
use serde::Deserialize;
use serde::Serialize;
use std::io::Error;
use crate::v1::buses_schema::BusSchema;
use crate::v1::buses_schema::RealDateTimeClass;

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

impl From<&DepartureConfig> for Query {
    fn from(value: &DepartureConfig) -> Self {
        Self {
            stop_id: value.point.clone(),
            ..Default::default()
        }
    }
}

#[actix_web::get("/buses")]
pub async fn buses(cfg: actix_web::web::Data<Config>) -> Result<impl Responder> {
    let client = reqwest::ClientBuilder::new()
        .build()
        .map_err(Error::other)?;

    let Some(stop) = cfg.departure.get(0) else {
        return Ok(HttpResponse::NotFound()
            .json(serde_json::json! {{

            }}))
    };

    let mut uri = Url::parse(BUS_API)
        .map_err(Error::other)?;

    let query = serde_qs::to_string(&Query::from(stop))
        .map_err(Error::other)?;

    uri.set_query(Some(&query));

    log::debug!("Bus URL: {uri:?}", uri=uri.to_string());

    let req = client.get(uri)
        .send()
        .await
        .expect("Failed to get buses");

    let res: BusSchema = req.json()
        .await
        .expect("Failed to get buses");

    let lines = res.departure_list
        .iter()
        .map(|line| {
            let eta = parse_date_time(line.real_date_time.clone()
                .unwrap_or(line.date_time.clone()))
                .map_err(Error::other)?;

            Ok(serde_json::json! {{
                "line": line.serving_line.symbol,
                "direction": line.serving_line.direction,
                "expectedArrival": eta
            }})
        });

    Ok(HttpResponse::Ok()
        .json(serde_json::json! {{
            "lines": lines.collect::<Result<Vec<_>>>()?
        }}))
}

fn parse_date_time(date: RealDateTimeClass) -> Result<chrono::DateTime<Local>> {
    let (yyyy, MM, dd, hh, mm) = (
        date.year.parse::<i32>().map_err(Error::other)?,
        date.month.parse::<u32>().map_err(Error::other)?,
        date.day.parse::<u32>().map_err(Error::other)?,
        date.hour.parse::<u32>().map_err(Error::other)?,
        date.minute.parse::<u32>().map_err(Error::other)?
    );

    let date = chrono::NaiveDate::from_ymd_opt(yyyy, MM, dd);
    let time = chrono::NaiveTime::from_hms_opt(hh, mm, 0);

    let datetime = date
        .and_then(|date| Some(date.and_time(time?)))
        .ok_or(Error::other("No time provided"))?;

    Ok(Local.from_local_datetime(&datetime).unwrap())
}