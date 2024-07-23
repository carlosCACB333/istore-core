use crate::{
    make_response,
    models::products::Product,
    utils::{db::Pool, tools::Status},
};
use actix_web::{post, web, Responder};
use rust_decimal::Decimal;

#[derive(Debug, serde::Deserialize)]
struct SeachReq {
    keywords: Vec<String>,
    min_price: Option<Decimal>,
    max_price: Option<Decimal>,
    is_on_sale: Option<bool>,
    order_by_price: Option<String>,
    order_by_date: Option<String>,
    order_by_rating: Option<String>,
    order_by_discount: Option<String>,
}

#[post("/search")]
pub async fn search_product(body: web::Json<SeachReq>, pool: Pool) -> impl Responder {
    log::info!("Requesting estract text {:?}", body);

    let products = match Product::search(
        pool.conn,
        body.keywords.clone(),
        body.min_price.clone(),
        body.max_price.clone(),
        body.is_on_sale.clone(),
        body.order_by_price.clone(),
        body.order_by_date.clone(),
        body.order_by_rating.clone(),
        body.order_by_discount.clone(),
    ) {
        Ok(products) => products,
        Err(e) => {
            return make_response!(Status::FAILED, e.to_string().as_str());
        }
    };

    make_response!(Status::SUCCESS, "Productos encontrados", products)
}
