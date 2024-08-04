use super::categories::Category;
use super::product_category::ProductWithCategory;
use crate::models::companies::Company;
use crate::models::product_category::ProductCategory;
use crate::schema::{categories, product_categories};
use crate::{schema::products, utils::db::Conn};
use diesel::result::Error;
use diesel::{pg::Pg, prelude::*};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::result::Result;

#[derive(Queryable, Selectable, Identifiable, Associations, Serialize, Deserialize, Debug)]
#[diesel(table_name = products)]
#[diesel(belongs_to(Company))]
#[diesel(check_for_backend(Pg))]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub original_price: Decimal,
    pub price: Decimal,
    pub stock: i32,
    pub rating: Decimal,
    pub description: String,
    pub images: serde_json::Value,
    pub colors: serde_json::Value,
    pub sizes: serde_json::Value,
    pub company_id: i32,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = products)]
#[diesel(check_for_backend(Pg))]
pub struct NewProduct {
    pub name: String,
    pub original_price: Option<Decimal>,
    pub price: Decimal,
    pub stock: i32,
    pub rating: Option<Decimal>,
    pub description: String,
    pub images: serde_json::Value,
    pub colors: serde_json::Value,
    pub sizes: serde_json::Value,
    pub company_id: Option<i32>,
}

impl NewProduct {
    pub fn save(&self, mut conn: Conn) -> Result<Product, Error> {
        let result = diesel::insert_into(products::table)
            .values(self)
            .returning(Product::as_returning())
            .get_result(&mut conn)?;

        Ok(result)
    }
}

impl Product {
    pub fn find_all(mut conn: Conn) -> Result<Vec<Product>, Error> {
        let result = products::table
            .select(Product::as_select())
            .load(&mut conn)?;
        Ok(result)
    }

    pub fn search2(
        mut conn: Conn,
        query: Option<String>,
    ) -> Result<Vec<ProductWithCategory>, Error> {
        let filter_fmt = format!("%{}%", query.unwrap_or("".to_string()));

        let all_categories = categories::table
            .select(Category::as_select())
            .get_result(&mut conn)?;

        let products = ProductCategory::belonging_to(&all_categories)
            .inner_join(products::table)
            .filter(products::name.like(filter_fmt.clone()))
            .select(Product::as_select())
            .load(&mut conn)?;

        let all_products = products::table
            .select(Product::as_select())
            .load(&mut conn)?;

        let categories = ProductCategory::belonging_to(&all_products)
            .inner_join(categories::table)
            .select((ProductCategory::as_select(), Category::as_select()))
            .load(&mut conn)?;

        let cat_per_prod: Vec<ProductWithCategory> = categories
            .grouped_by(&all_products)
            .into_iter()
            .zip(products)
            .map(|(b, product)| {
                let categories = b.into_iter().map(|(_, cat)| cat).collect();
                ProductWithCategory {
                    product,
                    categories,
                }
            })
            .collect();

        Ok(cat_per_prod)
    }

    pub fn search(
        mut conn: Conn,
        keywords: Vec<String>,
        min_price: Option<Decimal>,
        max_price: Option<Decimal>,
        is_on_sale: Option<bool>,
        order_by_price: Option<String>,
        order_by_date: Option<String>,
        order_by_rating: Option<String>,
        order_by_discount: Option<String>,
    ) -> Result<Vec<Product>, Error> {
        let mut query = products::table.into_boxed().limit(10);

        if !keywords.is_empty() {
            query = query.filter(
                products::name
                    .ilike(format!("%{}%", keywords[0]))
                    .or(products::description.ilike(format!("%{}%", keywords[0]))),
            );
            for keyword in &keywords[1..] {
                query = query.or_filter(products::name.ilike(format!("%{}%", keyword)));
                query = query.or_filter(products::description.ilike(format!("%{}%", keyword)));
            }
        }

        if let Some(min_price) = min_price {
            query = query.filter(products::price.ge(min_price));
        }

        if let Some(max_price) = max_price {
            query = query.filter(products::price.le(max_price));
        }

        if let Some(_) = is_on_sale {
            query = query.filter(products::original_price.gt(products::price));
        };

        if let Some(order) = order_by_price {
            if order.to_uppercase() == "ASC" {
                query = query.order(products::price.asc());
            } else {
                query = query.order(products::price.desc());
            }
        };

        if let Some(order) = order_by_date {
            if order.to_uppercase() == "ASC" {
                query = query.order(products::created_at.asc());
            } else {
                query = query.order(products::created_at.desc());
            }
        };

        if let Some(order) = order_by_rating {
            if order.to_uppercase() == "ASC" {
                query = query.order(products::rating.asc());
            } else {
                query = query.order(products::rating.desc());
            }
        };

        let mut products = query
            .select(Product::as_select())
            .load::<Product>(&mut conn)?;

        if let Some(order) = order_by_discount {
            if order.to_uppercase() == "ASC" {
                products.sort_by(|a, b| {
                    let a_discount = (a.original_price - a.price) / a.original_price;
                    let b_discount = (b.original_price - b.price) / b.original_price;
                    a_discount.partial_cmp(&b_discount).unwrap()
                });
            } else {
                products.sort_by(|a, b| {
                    let a_discount = (a.original_price - a.price) / a.original_price;
                    let b_discount = (b.original_price - b.price) / b.original_price;
                    b_discount.partial_cmp(&a_discount).unwrap()
                });
            }
        }

        return Ok(products);
    }

    pub fn find_by_id(mut conn: Conn, id: i32) -> Result<ProductWithCategory, Error> {
        let product = products::table
            .filter(products::id.eq(id))
            .select(Product::as_select())
            .first::<Product>(&mut conn)?;

        let categories = ProductCategory::belonging_to(&product)
            .inner_join(categories::table)
            .select(Category::as_select())
            .load(&mut conn)?;

        Ok(ProductWithCategory {
            product,
            categories,
        })
    }

    pub fn find_by_category_id(mut conn: Conn, id: i32) -> Result<Vec<ProductWithCategory>, Error> {
        let cat_prods = categories::table
            .filter(categories::id.eq(id))
            .inner_join(product_categories::table.inner_join(products::table))
            .select((Category::as_select(), Product::as_select()))
            .limit(10)
            .load::<(Category, Product)>(&mut conn)?;

        let mut map = HashMap::new();

        for (cat, prod) in cat_prods {
            map.entry(prod.id)
                .or_insert_with(|| ProductWithCategory {
                    product: prod,
                    categories: vec![],
                })
                .categories
                .push(cat);
        }

        let users_with_posts: Vec<ProductWithCategory> = map.into_values().collect();

        Ok(users_with_posts)
    }
}
