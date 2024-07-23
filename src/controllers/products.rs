use actix_web::web;

use crate::services::products::{
    create::{create_product, get_prods_with_categories, get_product_by_id, get_prods_by_category_id},
    search::search_product,
};

pub fn routes() -> actix_web::Scope {
    web::scope("/products")
        .service(search_product)
        .service(create_product)
        .service(get_prods_with_categories)
        .service(get_product_by_id)
        .service(get_prods_by_category_id)
}
