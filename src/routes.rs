use axum::{
    http::StatusCode,
    Json,
    extract::{
        Path,
        State,
    }
};
use crate::models::*;

pub async fn categories(
    State(state): State<AppState>
) -> (StatusCode, Json<Vec<Category>>) {
    let categories = sqlx::query_as!(Category, "SELECT * FROM categories")
        .fetch_all(&state.db)
        .await
        .expect("Error loading categories");

    (StatusCode::OK, Json(categories))
}

pub async fn items(
    State(state): State<AppState>
) -> (StatusCode, Json<Vec<Item>>) {
    let items = sqlx::query_as!(Item, "SELECT * FROM items")
        .fetch_all(&state.db)
        .await
        .expect("Error loading items");

    (StatusCode::OK, Json(items))
}

pub async fn new_item(
    State(state): State<AppState>,
    Json(payload): Json<NewItem>
) -> StatusCode {
    let cat = sqlx::query_as!(Category,"SELECT * FROM categories WHERE id = $1", payload.cat_id)
        .fetch_one(&state.db)
        .await
        .expect("Error loading category");

    let code_name = cat.code_name + &payload.code_name;

    let item: Result<Item, sqlx::Error> = sqlx::query_as!(Item, "SELECT * FROM items WHERE code_name = $1", code_name)
        .fetch_one(&state.db)
        .await;

    if item.is_ok() {
        return StatusCode::CONFLICT;
    }

    sqlx::query!(
        "INSERT INTO items (name, m_name, code_name, amount, price, cat_id)
         VALUES ($1, $2, $3, $4, $5, $6)",
        payload.name,
        payload.m_name,
        code_name,
        0,
        payload.price,
        payload.cat_id)
        .execute(&state.db)
        .await
        .expect("Error saving new item");

    StatusCode::CREATED
}

pub async fn add_stock(
    State(state): State<AppState>,
    Json(payload): Json<NewStock>
) -> StatusCode {
    sqlx::query!("UPDATE items SET amount = amount + $1 WHERE id = $2",
        payload.amount,
        payload.id)
        .execute(&state.db)
        .await
        .expect("Error updating item amount");

    StatusCode::OK
}

pub async fn new_cat(
    State(state): State<AppState>,
    Json(payload): Json<NewCategory>
) -> StatusCode {
    let existing_cats: Vec<Category> = sqlx::query_as!(Category, "SELECT * FROM categories WHERE code_name = $1", payload.code_name)
        .fetch_all(&state.db)
        .await
        .expect("Error loading category");

    if existing_cats.len() > 0 {
        return StatusCode::CONFLICT;
    }

    sqlx::query!("INSERT INTO categories (name, m_name, code_name, color, icon)
                  VALUES ($1, $2, $3, $4, $5)
                  RETURNING *",
        payload.name,
        payload.m_name,
        payload.code_name,
        payload.color,
        payload.icon)
        .fetch_one(&state.db)
        .await
        .expect("Error saving new category");

    StatusCode::CREATED
}

pub async fn delete_cat(
    State(state): State<AppState>,
    Path(id): Path<i32>
) -> StatusCode {
    let cats: Vec<Item> = sqlx::query_as!(Item, "SELECT * FROM items WHERE cat_id = $1", id)
        .fetch_all(&state.db)
        .await
        .expect("Error feting item with cat_id");

    if cats.len() > 0 {
        return StatusCode::CONFLICT;
    }
    sqlx::query!("DELETE FROM categories WHERE id = $1", id)
        .execute(&state.db)
        .await
        .expect("Cannot delete category with id");

    StatusCode::OK
}

pub async fn delete_item(
    State(state): State<AppState>,
    Path(id): Path<i32>
) -> StatusCode {
    let item: Result<Item, sqlx::Error> = sqlx::query_as!(Item,"SELECT * FROM items WHERE id = $1 AND amount = 0", id)
        .fetch_one(&state.db)
        .await;

    if item.is_err() {
        return StatusCode::UNPROCESSABLE_ENTITY;
    }

    sqlx::query!("DELETE FROM items WHERE id = $1 AND amount = 0", id)
        .execute(&state.db)
        .await
        .expect("Error deleting item with id and amount = 0");

    StatusCode::OK
}

pub async fn cash_flow_list(
    State(state): State<AppState>
) -> (StatusCode, Json<Vec<CashFlow>>) {
    let cash_flows: Vec<CashFlow> = sqlx::query_as!(CashFlow, "SELECT * FROM cashflow")
        .fetch_all(&state.db)
        .await
        .expect("Error loading cash flow");

    (StatusCode::OK, Json(cash_flows))
}

pub async fn new_cash_flow(
    State(state): State<AppState>,
    Json(payload): Json<NewCashFlow>
) -> (StatusCode, Json<CashFlow>) {

    let inserted_cash_flow = sqlx::query_as!(CashFlow,
        "INSERT INTO cashflow (time, amount, description)
         VALUES ($1, $2, $3)
         RETURNING *",
        payload.time.naive_utc(),
        payload.amount,
        payload.description)
        .fetch_one(&state.db)
        .await
        .expect("Error saving new cash flow");

    (StatusCode::CREATED, Json(inserted_cash_flow))
}

pub async fn items_by_cat(
    State(state): State<AppState>,
    Path(cat_id): Path<i32>,
) -> (StatusCode, Json<Vec<Item>>) {

    let items = sqlx::query_as!(Item, "SELECT * FROM items WHERE cat_id = $1", cat_id)
        .fetch_all(&state.db)
        .await
        .expect("Error loading items by cat");

    (StatusCode::OK, Json(items))
}

pub async fn purchase(
    State(state): State<AppState>,
    Json(payload): Json<NewVoucher>
) -> (StatusCode, Json<Voucher>) {

    let total = {
        let mut total = 0;
        for i in 0..payload.item_ids.len() {
            total += payload.item_prices[i] * payload.item_quantities[i];
        }
        total
    };

    for i in 0..payload.item_ids.len() {
        let _ = sqlx::query!("UPDATE items SET amount = amount - $1 WHERE id = $2",
            payload.item_quantities[i],
            payload.item_ids[i])
            .execute(&state.db)
            .await
            .expect("Error updating item amount");
    }

    let voucher = sqlx::query_as!(Voucher,
        "INSERT INTO vouchers (voucher_id, customer_name, customer_contact, item_ids, item_quantities, item_prices, time, total, paid)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
         RETURNING *",
        payload.voucher_id,
        payload.customer_name,
        payload.customer_contact,
        payload.item_ids.as_slice(),
        payload.item_quantities.as_slice(),
        payload.item_prices.as_slice(),
        payload.time.naive_utc(),
        total,
        payload.paid)
        .fetch_one(&state.db)
        .await
        .expect("Error saving new voucher");

    // TODO: Error handling

    if payload.paid != 0 {
        sqlx::query!("INSERT INTO cashflow (time, amount, description)
            VALUES ($1, $2, $3)",
            payload.time.naive_utc(),
            payload.paid,
            "Sale")
            .execute(&state.db)
            .await
            .expect("Error saving new cash flow");
    }

    (StatusCode::CREATED, Json(voucher))
}

pub async fn voucher_list(
    State(state): State<AppState>
) -> (StatusCode, Json<Vec<Voucher>>) {
    let vouchers = sqlx::query_as!(Voucher, "SELECT * FROM vouchers")
        .fetch_all(&state.db)
        .await
        .expect("Error loading vouchers");

    (StatusCode::OK, Json(vouchers))
}

pub async fn partial_vouchers(
    State(state): State<AppState>,
) -> (StatusCode, Json<Vec<PartialVoucher>>) {
    let vouchers = sqlx::query_as!(Voucher, "SELECT * FROM vouchers")
        .fetch_all(&state.db)
        .await
        .expect("Error loading vouchers");

    let partial_vouchers: Vec<PartialVoucher> = vouchers.iter().map(|voucher| PartialVoucher {
        id: voucher.id,
        voucher_id: voucher.voucher_id.clone(),
        customer_name: voucher.customer_name.clone(),
        customer_contact: voucher.customer_contact.clone(),
        time: voucher.time,
        total: voucher.total,
        paid: voucher.paid
    }).collect();
    (StatusCode::OK, Json(partial_vouchers))
}

pub async fn voucher_by_id(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> (StatusCode, Json<FullVoucher>) {
    let voucher: Voucher = sqlx::query_as!(Voucher, "SELECT * FROM vouchers WHERE id = $1", id)
        .fetch_one(&state.db)
        .await
        .expect("Error loading voucher by id");

    let items: Vec<Item> = sqlx::query_as!(Item, "SELECT * FROM items WHERE id = ANY($1::int[])", &voucher.item_ids)
        .fetch_all(&state.db)
        .await
        .expect("loading items by ids");

    let cart_items: Vec<CartItem> = voucher.item_ids.iter().map(|item_id| {
        let item = items.iter().find(|i| i.id == *item_id).unwrap();
        let ind = voucher.item_ids.iter().position(|i| i == item_id).unwrap();

        CartItem {
            item_id: *item_id,
            name: item.name.clone(),
            m_name: item.m_name.clone(),
            quantity: voucher.item_quantities[ind],
            price: voucher.item_prices[ind]
        }
    }).collect();

    let full_voucher = FullVoucher {
        id: voucher.id,
        voucher_id: voucher.voucher_id,
        customer_name: voucher.customer_name,
        customer_contact: voucher.customer_contact,
        items: cart_items,
        time: voucher.time,
        total: voucher.total,
        paid: voucher.paid
    };

    (StatusCode::OK, Json(full_voucher))
}
