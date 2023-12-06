use serde::{Serialize, Deserialize};
use sqlx::sqlx_macros::FromRow;

#[derive(Serialize)]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub m_name: String,
    pub code_name: String,
    pub color: String,
    pub icon: String
}

#[derive(Deserialize)]
pub struct NewCategory {
    pub name: String,
    pub m_name: String,
    pub code_name: String,
    pub color: String,
    pub icon: String
}

#[derive(Serialize, FromRow)]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub m_name: String,
    pub code_name: String,
    pub amount: i32,
    pub price: i32,
    pub cat_id: i32
}

#[derive(Deserialize)]
pub struct NewItem {
    pub name: String,
    pub m_name: String,
    pub code_name: String,
    pub amount: i32,
    pub price: i32,
    pub cat_id: i32
}

#[derive(Serialize, Deserialize)]
pub struct Voucher {
    pub id: i32,
    pub voucher_id: String,
    pub customer_name: Option<String>,
    pub customer_contact: Option<String>,
    pub item_ids: Vec<i32>,
    pub item_quantities: Vec<i32>,
    pub item_prices: Vec<i32>,
    pub time: chrono::NaiveDateTime,
    pub status: bool
}

#[derive(Serialize, Deserialize)]
pub struct CartItem {
    pub id: i32,
    pub quantity: i32,
    pub price: i32
}

#[derive(Deserialize)]
pub struct NewVoucher {
    pub voucher_id: String,
    pub customer_name: Option<String>,
    pub customer_contact: Option<String>,
    pub cart_items: Vec<CartItem>,
    pub paid_amount: i32
}
