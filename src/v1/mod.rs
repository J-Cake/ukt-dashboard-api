use actix_web::{HttpResponse, Responder};

mod buses;
mod weather;
mod config;
mod weather_schema;
mod buses_schema;

pub fn v1() -> actix_web::Scope {
    actix_web::web::scope("/v1")
        .service(version)
        .service(config::config)
        .service(weather::forecast)
        .service(buses::buses)
}

#[actix_web::get("/version")]
pub async fn version() -> impl Responder {
    HttpResponse::Ok()
        .json(serde_json::json! {{
            "service": "Azubitafel API",
            "version": env!("CARGO_PKG_VERSION").to_string(),
        }})
}