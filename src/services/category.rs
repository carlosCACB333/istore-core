use crate::{
    make_response,
    models::categories::Category,
    utils::{db::Pool, tools::Status},
};
use actix_web::{get, Responder};

#[get("")]
pub async fn find_categories(pool: Pool) -> impl Responder {
    let products = match Category::find_all(pool.conn) {
        Ok(products) => products,
        Err(e) => {
            return make_response!(Status::FAILED, e.to_string().as_str());
        }
    };

    make_response!(products)
}
