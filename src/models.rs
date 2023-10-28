use diesel::prelude::*;
use chrono::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::categories)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Category {
    pub id: i32,
    pub name: String
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::categories)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewCategory {
    pub name: String
}

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::subcategories)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SubCategory {
    pub id: i32,
    pub name: String
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::subcategories)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewSubCategory {
    pub name: String
}

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::items)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub code_name: String,
    pub amount: i32,
    pub price: i32,
    pub cat_id: i32,
    pub subcat_id: i32
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::items)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewItem {
    pub name: String,
    pub code_name: String,
    pub amount: i32,
    pub price: i32,
    pub cat_id: i32,
    pub subcat_id: i32
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::transections)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Transection {
    pub id: i32,
    pub direction: bool,
    pub time: NaiveDateTime,
    pub price: i32,
    pub cat_id: i32,
    pub subcat_id: i32
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::transections)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewTransection {
    pub direction: bool,
    pub time: NaiveDateTime,
    pub price: i32,
    pub cat_id: i32,
    pub subcat_id: i32
}
