use crate::schema::products;
use crate::{schema::companies, utils::db::Conn};
use diesel::prelude::*;
use diesel::{pg::Pg, result::Error};
use serde::{Deserialize, Serialize};
use std::result::Result;
use super::products::Product;

#[derive(Queryable, Selectable, Identifiable, Serialize, Deserialize, Debug)]
#[diesel(table_name = companies)]
#[diesel(check_for_backend(Pg))]
pub struct Company {
    pub id: i32,
    pub name: String,
    pub address: String,
    pub phone: String,
    pub email: String,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

impl Company {
    pub fn find_all(mut conn: Conn) -> Result<Vec<CompanyWithProducts>, Error> {
        let all_companies = companies::table
            .select(Company::as_select())
            .load(&mut conn)?;

        // get all pages for all books
        let all_products = Product::belonging_to(&all_companies)
            .select(Product::as_select())
            .load(&mut conn)?;
 
        let pages_per_book = all_products
            .grouped_by(&all_companies)
            .into_iter()
            .zip(all_companies)
            .map(|(products, company)| CompanyWithProducts { company, products })
            .collect::<Vec<CompanyWithProducts>>();
        Ok(pages_per_book)
    }

    pub fn join_products( mut conn: Conn) -> Result<Vec<(Product, Company)>, Error> {
        let products = products::table.inner_join(companies::table)
            .select((Product::as_select(), Company::as_select()))
            // .filter(companies::id.eq(self.id))
            .load::<(Product, Company)>(&mut conn)?;
        Ok(products)
    }
}


#[derive(Serialize)]
pub struct CompanyWithProducts {
    #[serde(flatten)]
    company: Company,
    products: Vec<Product>,
}
