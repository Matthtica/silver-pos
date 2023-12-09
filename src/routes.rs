use axum::{
    http::StatusCode,
    Json,
    extract::{
        Path,
        State
    }
};
use crate::models::*;

use sqlx::{
    Pool,
    postgres::Postgres
};

pub async fn categories(
    State(pool): State<Pool<Postgres>>
) -> (StatusCode, Json<Vec<Category>>) {
    let categories = sqlx::query_as!(Category, "SELECT * FROM categories")
        .fetch_all(&pool)
        .await
        .expect("Error loading categories");

    (StatusCode::OK, Json(categories))
}

pub async fn items(
    State(pool): State<Pool<Postgres>>
) -> (StatusCode, Json<Vec<Item>>) {
    let items = sqlx::query_as!(Item, "SELECT * FROM items")
        .fetch_all(&pool)
        .await
        .expect("Error loading items");

    (StatusCode::OK, Json(items))
}

pub async fn new_item(
    State(pool): State<Pool<Postgres>>,
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
    State(pool): State<Pool<Postgres>>,
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

pub async fn cash_flow_list(
    State(pool): State<Pool<Postgres>>
) -> (StatusCode, Json<Vec<CashFlow>>) {
    let cash_flows: Vec<CashFlow> = sqlx::query_as!(CashFlow, "SELECT * FROM cashflow")
        .fetch_all(&pool)
        .await
        .expect("Error loading cash flow");

    (StatusCode::OK, Json(cash_flows))
}

pub async fn new_cash_flow(
    State(pool): State<Pool<Postgres>>,
    Json(payload): Json<NewCashFlow>
) -> (StatusCode, Json<CashFlow>) {

    let inserted_cash_flow = sqlx::query_as!(CashFlow,
        "INSERT INTO cashflow (time, amount, description)
         VALUES ($1, $2, $3)
         RETURNING *",
        payload.time.naive_utc(),
        payload.amount,
        payload.description)
        .fetch_one(&pool)
        .await
        .expect("Error saving new cash flow");

    (StatusCode::CREATED, Json(inserted_cash_flow))
}

pub async fn items_by_cat(
    State(pool): State<Pool<Postgres>>,
    Path(cat_id): Path<i32>,
) -> (StatusCode, Json<Vec<Item>>) {

    let items = sqlx::query_as!(Item, "SELECT * FROM items WHERE cat_id = $1", cat_id)
        .fetch_all(&pool)
        .await
        .expect("Error loading items by cat");

    (StatusCode::OK, Json(items))
}

pub async fn purchase(
    State(pool): State<Pool<Postgres>>,
    Json(payload): Json<NewVoucher>
) -> (StatusCode, Json<Voucher>) {

    let total = {
        let mut total = 0;
        for i in 0..payload.item_ids.len() {
            total += payload.item_prices[i] * payload.item_quantities[i];
        }
        total
    };

    let status = total == payload.paid_amount;

    for i in 0..payload.item_ids.len() {
        let _ = sqlx::query!("UPDATE items SET amount = amount - $1 WHERE id = $2",
            payload.item_quantities[i],
            payload.item_ids[i])
            .execute(&pool)
            .await
            .expect("Error updating item amount");
    }

    let voucher = sqlx::query_as!(Voucher,
        "INSERT INTO vouchers (voucher_id, customer_name, customer_contact, item_ids, item_quantities, item_prices, time, status)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
         RETURNING *",
        payload.voucher_id,
        payload.customer_name,
        payload.customer_contact,
        payload.item_ids.as_slice(),
        payload.item_quantities.as_slice(),
        payload.item_prices.as_slice(),
        payload.time.naive_utc(),
        status)
        .fetch_one(&pool)
        .await
        .expect("Error saving new voucher");

    // TODO: Error handling

    if payload.paid_amount != 0 {
        sqlx::query!("INSERT INTO cashflow (time, amount, description)
            VALUES ($1, $2, $3)",
            payload.time.naive_utc(),
            payload.paid_amount,
            "Sale")
            .execute(&pool)
            .await
            .expect("Error saving new cash flow");
    }

    (StatusCode::CREATED, Json(voucher))
}

pub async fn voucher_list(
    State(pool): State<Pool<Postgres>>
) -> (StatusCode, Json<Vec<Voucher>>) {
    let vouchers = sqlx::query_as!(Voucher, "SELECT * FROM vouchers")
        .fetch_all(&pool)
        .await
        .expect("Error loading vouchers");

    (StatusCode::OK, Json(vouchers))
}
