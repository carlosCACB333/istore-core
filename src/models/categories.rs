use super::{product_category::CategoryWithProducts, products::Product};
use crate::{
    schema::{categories, product_categories, products},
    utils::db::Conn,
};
use diesel::pg::Pg;
use diesel::prelude::*;
use diesel::result::Error;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Identifiable, Serialize, Deserialize, Debug)]
#[diesel(table_name = categories)]
#[diesel(check_for_backend(Pg))]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub image: String,
}

impl Category {
    pub fn find_all(mut conn: Conn) -> Result<Vec<Category>, Error> {
        let result = categories::table.load::<Category>(&mut conn)?;
        Ok(result)
    }

    pub fn with_products(mut conn: Conn) -> Result<Vec<CategoryWithProducts>, Error> {
        let cat_prods = categories::table
            .inner_join(product_categories::table.inner_join(products::table))
            .select((Category::as_select(), Product::as_select()))
            .load::<(Category, Product)>(&mut conn)?;

        let mut map = std::collections::HashMap::new();

        for (cat, prod) in cat_prods {
            map.entry(cat.id)
                .or_insert_with(|| CategoryWithProducts {
                    category: cat,
                    products: vec![],
                })
                .products
                .push(prod);
        }

        let users_with_posts: Vec<CategoryWithProducts> = map.into_values().collect();

        Ok(users_with_posts)
    }
}
