mod config;
mod v1;
mod cache;

use actix_web::{HttpResponse, Responder, web};
pub use std::io;

pub type Result<T> = core::result::Result<T, io::Error>;

#[actix_web::main]
pub async fn main() -> Result<()> {
    env_logger::init();

    let args: config::CliArgs = clap::Parser::parse();
    let config: config::Config = tokio::fs::read_to_string(&args.config)
        .await
        .and_then(|config| toml::from_str(&config).map_err(io::Error::other))?;

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
