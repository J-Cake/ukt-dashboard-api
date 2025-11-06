mod buses;
mod weather;
mod config;


pub fn v1() -> actix_web::Scope {
    actix_web::web::scope("/v1")
        .service(config::config)
        .service(weather::forecast)
        .service(buses::buses)
}