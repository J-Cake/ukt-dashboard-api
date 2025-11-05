mod buses;
mod weather;
mod config;

use common::prelude::*;
use actix_web::Responder;
use serde::Deserialize;
use serde::Serialize;

pub fn v1() -> actix_web::Scope {
    actix_web::web::scope("/v1")
        .service(config::config)
        .service(weather::forecast)
        .service(buses::buses)
}