use common::prelude::*;
use actix_web::HttpResponse;
use actix_web::Responder;
use actix_web::web;

#[actix_web::get("/config")]
pub async fn config(config: web::Data<Config>) -> Result<impl Responder> {
    Ok(HttpResponse::Ok().json(config))
}
