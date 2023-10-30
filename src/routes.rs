use axum::{
    http::StatusCode,
    Json,
};
use crate::utils::establish_connection;
use crate::models::{
    Category, Item, NewItem
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

pub async fn items() -> (StatusCode, Json<Vec<Item>>) {
    let conn = &mut establish_connection();
    use crate::schema::items::dsl::items;

    let result = items
        .select(Item::as_select())
        .load(conn)
        .expect("Cannot load items");

    (StatusCode::OK, Json(result))
}

pub async fn new_item(Json(payload): Json<NewItem>) -> (StatusCode, Json<Item>) {
    let conn = &mut establish_connection();
    use crate::schema::items::dsl::items;

    let result = diesel::insert_into(items)
        .values(&payload)
        .returning(Item::as_returning())
        .get_result(conn)
        .expect("Error saving new item");

    (StatusCode::CREATED, Json(result))
}
