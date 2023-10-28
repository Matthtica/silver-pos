use axum::{
    http::StatusCode,
    Json,
};
use crate::utils::establish_connection;
use crate::models::{
    Category, SubCategory, Item
};
use diesel::{
    QueryDsl, RunQueryDsl, SelectableHelper
};

pub async fn categories() -> (StatusCode, Json<Vec<Category>>) {
    let conn = &mut establish_connection();
    use crate::schema::categories::dsl::categories;
    let result = categories
        .select(Category::as_select())
        .load(conn)
        .expect("Cannot load categories");

    (StatusCode::OK, Json(result))
}

pub async fn subcategories() -> (StatusCode, Json<Vec<SubCategory>>) {
    let conn = &mut establish_connection();
    use crate::schema::subcategories::dsl::subcategories;

    let result = subcategories
        .select(SubCategory::as_select())
        .load(conn)
        .expect("Cannot load subcategories");

    (StatusCode::OK, Json(result))
}

pub async fn items() -> (StatusCode, Json<Vec<Item>>) {
    let conn = &mut establish_connection();
    use crate::schema::items::dsl::items;

    let result = items
        .select(Item::as_select())
        .load(conn)
        .expect("Cannot load items");

    (StatusCode::OK, Json(result))
}
