mod cache;
mod v1;

use std::io;
use actix_web::{web, HttpResponse, Responder};
use common::prelude::*;

#[actix_web::main]
pub async fn main() -> Result<()> {
    env_logger::init();

    let config = common::get_config().await?;

    log::debug!("Using config: {config:#?}");

    let data = web::Data::new(config.clone());
    actix_web::HttpServer::new(move || actix_web::App::new()
        .app_data(data.clone())
        .service(v1::v1()))
        .bind(&config.bind.socket)?
        .run()
        .await?;

    Ok(())
}

#[actix_web::get("/hello")]
pub async fn hello(req: actix_web::HttpRequest) -> Result<impl Responder> {
    Ok(HttpResponse::NotImplemented().body("Hello World"))
}
