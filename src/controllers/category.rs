use actix_web::web;

use crate::services::category::find_categories;

pub fn routes() -> actix_web::Scope {
    web::scope("/categories").service(find_categories)
}
