use crate::Result;
use actix_web::{HttpResponse, Responder};
use chrono::{Local, TimeZone};
use common::config::Config;
use common::prelude::clap::builder::Str;
use common::prelude::DepartureConfig;
use reqwest::Url;
use serde::{Deserialize, Serialize};
use std::alloc::System;
use std::io::Error;
use std::ops::Add;
use std::str::FromStr;
use std::time::{Duration, SystemTime};

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

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonSchema {
    parameters: Vec<Parameter>,
    dm: Dm,
    arr: Arr,
    date_time: DateTime,
    date_range: Vec<DateRange>,
    #[serde(rename = "option")]
    json_schema_option: JsonSchemaOption,
    serving_lines: ServingLines,
    departure_list: Vec<DepartureList>,
}

#[derive(Serialize, Deserialize)]
pub struct Arr {
    input: Input,
    points: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize)]
pub struct Input {
    input: String,
}

#[derive(Serialize, Deserialize)]
pub struct DateRange {
    day: String,
    month: String,
    year: String,
    weekday: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DateTime {
    deparr: String,
    ttp_from: String,
    ttp_to: String,
    year: String,
    month: String,
    day: String,
    hour: String,
    minute: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DepartureList {
    #[serde(rename = "stopID")]
    stop_id: String,
    x: String,
    y: String,
    map_name: String,
    area: String,
    platform: String,
    platform_name: String,
    stop_name: String,
    #[serde(rename = "nameWO")]
    name_wo: String,
    countdown: String,
    date_time: RealDateTimeClass,
    real_date_time: Option<RealDateTimeClass>,
    realtime_trip_status: Option<RealtimeTripStatus>,
    serving_line: ServingLine,
    operator: Operator,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RealDateTimeClass {
    year: String,
    month: String,
    day: String,
    weekday: String,
    hour: String,
    minute: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Operator {
    code: String,
    name: String,
    public_code: String,
}

#[derive(Serialize, Deserialize)]
pub enum RealtimeTripStatus {
    #[serde(rename = "")]
    Empty,
    #[serde(rename = "MONITORED")]
    Monitored,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServingLine {
    key: String,
    code: String,
    number: String,
    symbol: String,
    mot_type: String,
    mt_subcode: String,
    realtime: String,
    direction: String,
    direction_from: String,
    train_num: String,
    name: String,
    delay: Option<String>,
    hints: Option<Vec<Hint>>,
    li_erg_ri_proj: LiErgRiProj,
    #[serde(rename = "destID")]
    dest_id: String,
    stateless: String,
    line_display: String,
}

#[derive(Serialize, Deserialize)]
pub struct Hint {
    content: String,
}

#[derive(Serialize, Deserialize)]
pub struct LiErgRiProj {
    line: String,
    project: String,
    direction: String,
    supplement: String,
    network: String,
    gid: String,
}

// #[derive(Serialize, Deserialize)]
// pub enum Dir {
//     H,
//     R,
// }

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dm {
    input: Input,
    points: Points,
    itd_odv_assigned_stops: ItdOdvAssignedStops,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItdOdvAssignedStops {
    #[serde(rename = "stopID")]
    stop_id: String,
    name: String,
    x: String,
    y: String,
    map_name: String,
    value: String,
    place: String,
    name_with_place: String,
    distance_time: String,
    is_transfer_stop: String,
    vm: String,
    gid: String,
}

#[derive(Serialize, Deserialize)]
pub struct Points {
    point: Point,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Point {
    usage: String,
    #[serde(rename = "type")]
    point_type: String,
    name: String,
    stateless: String,
    any_type: String,
    sort: String,
    quality: String,
    best: String,
    object: String,
    #[serde(rename = "ref")]
    point_ref: Ref,
    infos: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ref {
    id: String,
    gid: String,
    omc: String,
    #[serde(rename = "placeID")]
    place_id: String,
    place: String,
    coords: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonSchemaOption {
    pt_option: PtOption,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PtOption {
    active: String,
    max_changes: String,
    max_time: String,
    max_wait: String,
    route_type: String,
    change_speed: String,
    line_restriction: String,
    use_prox_foot_search: String,
    use_prox_foot_search_origin: String,
    use_prox_foot_search_destination: String,
    bike: String,
    plane: String,
    no_crowded: String,
    no_solid_stairs: String,
    no_escalators: String,
    no_elevators: String,
    low_platform_vhcl: String,
    wheelchair: String,
    need_elevated_plt: String,
    assistance: String,
    #[serde(rename = "SOSAvail")]
    sos_avail: String,
    no_lonely_transfer: String,
    illum_transfer: String,
    overground_transfer: String,
    no_insecure_places: String,
    private_transport: String,
    excluded_means: Vec<ExcludedMean>,
    active_imp: String,
    active_com: String,
    active_sec: String,
}

#[derive(Serialize, Deserialize)]
pub struct ExcludedMean {
    means: String,
    value: String,
    selected: String,
}

#[derive(Serialize, Deserialize)]
pub struct Parameter {
    name: String,
    value: String,
}

#[derive(Serialize, Deserialize)]
pub struct ServingLines {
    lines: Vec<Line>,
}

#[derive(Serialize, Deserialize)]
pub struct Line {
    mode: Mode,
    index: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mode {
    name: String,
    number: String,
    product: String,
    product_id: String,
    #[serde(rename = "type")]
    mode_type: String,
    code: String,
    destination: String,
    #[serde(rename = "destID")]
    dest_id: String,
    desc: String,
    timetable_period: String,
    diva: Diva,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Diva {
    branch: String,
    line: String,
    supplement: String,
    dir: String,
    project: String,
    network: String,
    stateless: String,
    global_id: String,
    trip_code: String,
    operator: String,
    op_public_code: String,
    op_code: String,
    v_f: String,
    v_to: String,
    line_display: String,
    #[serde(rename = "isTTB")]
    is_ttb: String,
    #[serde(rename = "isSTT")]
    is_stt: String,
    attrs: Vec<Parameter>,
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

    let res: JsonSchema = req.json()
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