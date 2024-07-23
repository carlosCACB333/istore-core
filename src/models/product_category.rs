use crate::models::categories::Category;
use crate::models::products::Product;
use crate::schema::product_categories;
use diesel::{
    associations::{Associations, Identifiable},
    deserialize::Queryable,
    Selectable,
};
use serde::Serialize;
#[derive(Identifiable, Selectable, Queryable, Associations, Debug)]
#[diesel(belongs_to(Category))]
#[diesel(belongs_to(Product))]
#[diesel(table_name = product_categories)]
#[diesel(primary_key(product_id, category_id))]
pub struct ProductCategory {
    pub product_id: i32,
    pub category_id: i32,
}

#[derive(Serialize)]
pub struct CategoryWithProducts {
    #[serde(flatten)]
    pub category: Category,
    pub products: Vec<Product>,
}

#[derive(Serialize)]
pub struct ProductWithCategory {
    #[serde(flatten)]
    pub product: Product,
    pub categories: Vec<Category>,
}
