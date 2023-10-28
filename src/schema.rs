// @generated automatically by Diesel CLI.

diesel::table! {
    categories (id) {
        id -> Int4,
        name -> Text,
    }
}

diesel::table! {
    items (id) {
        id -> Int4,
        name -> Varchar,
        code_name -> Varchar,
        amount -> Int4,
        price -> Int4,
        cat_id -> Int4,
        subcat_id -> Int4,
    }
}

diesel::table! {
    subcategories (id) {
        id -> Int4,
        cat_id -> Int4,
        name -> Text,
    }
}

diesel::table! {
    transections (id) {
        id -> Int4,
        direction -> Bool,
        time -> Timestamp,
        price -> Int4,
        cat_id -> Int4,
        subcat_id -> Int4,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    categories,
    items,
    subcategories,
    transections,
);
