// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "cart_item"))]
    pub struct CartItem;
}

diesel::table! {
    categories (id) {
        id -> Int4,
        name -> Varchar,
        m_name -> Text,
        code_name -> Varchar,
        color -> Varchar,
        icon -> Varchar,
    }
}

diesel::table! {
    items (id) {
        id -> Int4,
        name -> Varchar,
        m_name -> Varchar,
        code_name -> Varchar,
        amount -> Int4,
        price -> Int4,
        cat_id -> Int4,
    }
}

diesel::table! {
    transections (id) {
        id -> Int4,
        time -> Timestamp,
        price -> Int4,
        cat_id -> Int4,
        count -> Int4,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::CartItem;

    vouchers (id) {
        id -> Int4,
        voucher_id -> Varchar,
        customer_name -> Nullable<Varchar>,
        customer_contact -> Nullable<Varchar>,
        cart_items -> Array<Nullable<CartItem>>,
        time -> Timestamp,
        status -> Bool,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    categories,
    items,
    transections,
    vouchers,
);
