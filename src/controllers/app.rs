use crate::services::app::home_page;
use actix_web::web;

pub fn routes() -> actix_web::Scope {
    web::scope("").service(home_page)
}
