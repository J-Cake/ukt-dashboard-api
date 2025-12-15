use chrono::{Local, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BusSchema {
    parameters: Vec<Parameter>,
    dm: Dm,
    // arr: Arr,
    date_time: DateTime,
    date_range: Vec<DateRange>,
    #[serde(rename = "option")]
    json_schema_option: JsonSchemaOption,
    serving_lines: ServingLines,
    pub(crate) departure_list: Vec<DepartureList>,
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
    pub stop_name: String,
    #[serde(rename = "nameWO")]
    name_wo: String,
    countdown: String,
    pub(crate) date_time: RealDateTimeClass,
    pub(crate) real_date_time: Option<RealDateTimeClass>,
    realtime_trip_status: Option<String>,
    pub(crate) serving_line: ServingLine,
    operator: Operator,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RealDateTimeClass {
    pub(crate) year: String,
    pub(crate) month: String,
    pub(crate) day: String,
    weekday: String,
    pub(crate) hour: String,
    pub(crate) minute: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Operator {
    code: String,
    name: String,
    public_code: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServingLine {
    key: String,
    code: String,
    number: String,
    pub(crate) symbol: String,
    mot_type: String,
    mt_subcode: String,
    realtime: String,
    pub(crate) direction: String,
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

#[derive(Serialize, Deserialize)]
pub struct DepartureBoardStop {
    pub stop: String,
    pub line: String,
    pub direction: String,

    #[serde(rename = "expectedArrival")]
    pub expected_arrival: chrono::DateTime<Local>,

    #[serde(rename = "givenArrival")]
    pub given_arrival: chrono::DateTime<Local>
}