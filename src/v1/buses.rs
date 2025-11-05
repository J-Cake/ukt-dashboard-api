use actix_web::{HttpResponse, Responder};
use crate::Result;

#[actix_web::get("/buses")]
pub async fn buses() -> Result<impl Responder> {
    Ok(HttpResponse::NotImplemented())
}