use axum::{
    http::StatusCode,
    Extension,
    Json,
};
use serde::Deserialize;
use crate::utils::establish_connection;
use diesel::{
    QueryDsl, RunQueryDsl, SelectableHelper, ExpressionMethods
};
use crate::models::*;

use sqlx::{
    Pool,
    postgres::Postgres
};

pub async fn categories(
    Extension(pool): Extension<Pool<Postgres>>
) -> (StatusCode, Json<Vec<Category>>) {
    let categories = sqlx::query_as!(Category, "SELECT * FROM categories")
        .fetch_all(&pool)
        .await
        .expect("Error loading categories");

    (StatusCode::OK, Json(categories))
}

pub async fn items(
    Extension(pool): Extension<Pool<Postgres>>
) -> (StatusCode, Json<Vec<Item>>) {
    let items = sqlx::query_as!(Item, "SELECT * FROM items")
        .fetch_all(&pool)
        .await
        .expect("Error loading items");

    (StatusCode::OK, Json(items))
}

pub async fn new_item(
    Extension(pool): Extension<Pool<Postgres>>,
    Json(payload): Json<NewItem>
) -> (StatusCode, Json<Item>) {
    let cat = sqlx::query_as!(Category,"SELECT * FROM categories WHERE id = $1", payload.cat_id)
        .fetch_one(&pool)
        .await
        .expect("Error loading category");

    let inserted_item = sqlx::query_as!(Item,
        "INSERT INTO items (name, m_name, code_name, amount, price, cat_id)
         VALUES ($1, $2, $3, $4, $5, $6)
         RETURNING id, name, m_name, code_name, amount, price, cat_id",
        payload.name,
        payload.m_name,
        cat.code_name + &payload.code_name,
        payload.amount,
        payload.price,
        payload.cat_id)
        .fetch_one(&pool)
        .await
        .expect("Error saving new item");

    (StatusCode::CREATED, Json(inserted_item))
}

pub async fn new_cat(
    Extension(pool): Extension<Pool<Postgres>>,
    Json(payload): Json<NewCategory>
) -> (StatusCode, Json<Category>) {
    let cat = sqlx::query_as!(Category,
        "INSERT INTO categories (name, m_name, code_name, color, icon)
         VALUES ($1, $2, $3, $4, $5)
         RETURNING id, name, m_name, code_name, color, icon",
        payload.name,
        payload.m_name,
        payload.code_name,
        payload.color,
        payload.icon)
        .fetch_one(&pool)
        .await
        .expect("Error saving new category");

    (StatusCode::CREATED, Json(cat))
}

// TODO: this should be on dynamic url
#[derive(Deserialize)] pub struct WithCatId { cat_id: i32 }
pub async fn items_by_cat(
    Json(payload): Json<WithCatId>
) -> (StatusCode, Json<Vec<Item>>) {
    let conn = &mut establish_connection();
    use crate::schema::items;

    let result = items::table
        .filter(items::cat_id.eq(payload.cat_id))
        .select(Item::as_select())
        .load(conn)
        .expect("Cannot load items by cat");

    (StatusCode::OK, Json(result))
}

// purchase should submit a voucher
pub async fn purchase(
    Json(payload): Json<Vec<CartItem>>
) -> (StatusCode, Json<Vec<Transection>>) {
    let conn = &mut establish_connection();
    use crate::schema::transections;
    use crate::schema::items;

    // Step 1: get the list of items and transform into hashmap
    let cart_map: std::collections::HashMap<i32, i32> = {
        let mut cartmap = std::collections::HashMap::new();
        payload.iter().for_each(|cart| {
            let _ = cartmap.insert(cart.item_id, cart.count);
        });
        cartmap
    };

    let items: Vec<Item> = items::table
        .filter(items::id.eq_any(payload.iter().map(|cart| cart.item_id)))
        .select(Item::as_returning())
        .load(conn)
        .expect("Cannot load items for transections");

    // Step 2: update the items table with modified count values
    items.iter().for_each(|item| {
        diesel::update(items::table.filter(items::id.eq(item.id)))
            .set(items::amount.eq(item.amount - cart_map.get(&item.id).unwrap()))
            .execute(conn)
            .expect("Error updating items amount");
    });

    // Step 3: insert new transections
    let transections: Vec<NewTransection> = items.iter().map(|item| {
        NewTransection::new(item.price, item.cat_id, *cart_map.get(&item.id).unwrap())
    }).collect();

    let result: Vec<Transection> = diesel::insert_into(transections::table)
        .values(&transections)
        .returning(Transection::as_returning())
        .get_results(conn)
        .expect("Error inserting transections");

    // Step 4: return StatusCode::CREATED and the list of transections for the client to use as
    //         paycheck
    (StatusCode::CREATED, Json(result))
}
