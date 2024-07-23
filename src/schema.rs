// @generated automatically by Diesel CLI.

diesel::table! {
    categories (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        description -> Text,
        #[max_length = 255]
        image -> Varchar,
    }
}

diesel::table! {
    companies (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        address -> Varchar,
        #[max_length = 255]
        phone -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    product_categories (product_id, category_id) {
        product_id -> Int4,
        category_id -> Int4,
    }
}

diesel::table! {
    products (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        original_price -> Numeric,
        price -> Numeric,
        description -> Text,
        stock -> Int4,
        colors -> Jsonb,
        sizes -> Jsonb,
        images -> Jsonb,
        rating -> Numeric,
        company_id -> Int4,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(product_categories -> categories (category_id));
diesel::joinable!(product_categories -> products (product_id));
diesel::joinable!(products -> companies (company_id));

diesel::allow_tables_to_appear_in_same_query!(
    categories,
    companies,
    product_categories,
    products,
);
