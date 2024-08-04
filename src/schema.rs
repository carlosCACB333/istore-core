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
    chats (id) {
        #[max_length = 36]
        id -> Varchar,
        #[max_length = 50]
        model -> Varchar,
        #[max_length = 255]
        api_key -> Varchar,
        user_id -> Nullable<Int4>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
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
    messages (id) {
        #[max_length = 36]
        id -> Varchar,
        #[max_length = 10]
        role -> Varchar,
        content -> Jsonb,
        #[max_length = 36]
        chat_id -> Varchar,
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

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 255]
        first_name -> Varchar,
        #[max_length = 255]
        last_name -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(chats -> users (user_id));
diesel::joinable!(messages -> chats (chat_id));
diesel::joinable!(product_categories -> categories (category_id));
diesel::joinable!(product_categories -> products (product_id));
diesel::joinable!(products -> companies (company_id));

diesel::allow_tables_to_appear_in_same_query!(
    categories,
    chats,
    companies,
    messages,
    product_categories,
    products,
    users,
);
