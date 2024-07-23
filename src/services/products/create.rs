use crate::{
    make_response,
    models::products::{NewProduct, Product},
    utils::{db::Pool, tools::Status},
};
use actix_web::{get, post, web, Responder};

use serde_json::json;

#[post("")]
pub async fn create_product(mut body: web::Json<NewProduct>, pool: Pool) -> impl Responder {
    log::info!("Requesting estract text {:?}", body);

    body.company_id = Some(1);

    if body.original_price.is_none() {
        body.original_price = Some(body.price);
    }

    let product = body.save(pool.conn);

    let product = match product {
        Ok(product) => product,
        Err(e) => {
            return make_response!(Status::FAILED, e.to_string().as_str());
        }
    };

    make_response!(json!(product))
}

#[get("")]
pub async fn get_prods_with_categories(pool: Pool) -> impl Responder {
    let products = match Product::find_all(pool.conn) {
        Ok(products) => products,
        Err(e) => {
            return make_response!(Status::FAILED, e.to_string().as_str());
        }
    };

    make_response!(products)
}

#[get("/{id}")]
pub async fn get_product_by_id(pool: Pool, id: web::Path<i32>) -> impl Responder {
    let product = match Product::find_by_id(pool.conn, id.into_inner()) {
        Ok(product) => product,
        Err(e) => {
            return make_response!(Status::FAILED, e.to_string().as_str());
        }
    };

    make_response!(product)
}

#[get("/category/{id}")]
pub async fn get_prods_by_category_id(pool: Pool, id: web::Path<i32>) -> impl Responder {
    let products = match Product::find_by_category_id(pool.conn, id.into_inner()) {
        Ok(products) => products,
        Err(e) => {
            return make_response!(Status::FAILED, e.to_string().as_str());
        }
    };

    make_response!(products)
}
