use axum::{
    http::StatusCode,
    Extension,
    Json,
    extract::Path
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
         RETURNING *",
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
         RETURNING *",
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

pub async fn items_by_cat(
    Extension(pool): Extension<Pool<Postgres>>,
    Path(cat_id): Path<i32>,
) -> (StatusCode, Json<Vec<Item>>) {

    let items = sqlx::query_as!(Item, "SELECT * FROM items WHERE cat_id = $1", cat_id)
        .fetch_all(&pool)
        .await
        .expect("Error loading items by cat");

    (StatusCode::OK, Json(items))
}

// purchase should submit a voucher
pub async fn purchase(
    Extension(pool): Extension<Pool<Postgres>>,
    Json(payload): Json<NewVoucher>
) -> (StatusCode, Json<Voucher>) {
    let status = payload.cart_items.iter().fold(0, |prev, item| {
        prev + item.quantity * item.price
    }) == payload.paid_amount;

    let voucher = sqlx::query!(
        "INSERT INTO vouchers (voucher_id, customer_name, customer_contact, cart_items, time, status)
         VALUES ($1, $2, $3, $4, $5, $6)
         RETURNING *",
        payload.voucher_id,
        payload.customer_name,
        payload.customer_contact,
        Json(payload.cart_items) as _,
        chrono::Utc::now().naive_utc(),
        status)
        .fetch_one(&pool)
        .await
        .expect("Error saving new voucher");

    (StatusCode::CREATED, Json(voucher))
}
