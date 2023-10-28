use crate::models::{
    Item, NewItem, Category, NewCategory
};

use diesel::prelude::*;

pub fn create_item(conn: &mut PgConnection, new_items: Vec<NewItem>) -> Item {
    use crate::schema::items;

    diesel::insert_into(items::table)
        .values(&new_items)
        .returning(Item::as_returning())
        .get_result(conn)
        .expect("Error saving new item")
}

pub fn init_cats(conn: &mut PgConnection, cats: &Vec<&str>) {
    use crate::schema::categories;
    let new_cats: Vec<NewCategory> = cats.iter().map(|name| NewCategory { name: name.to_string() }).collect();

    diesel::insert_into(categories::table)
        .values(&new_cats)
        .execute(conn)
        .expect("Cannot insert new categories");
}

pub fn find_cat_by_name(conn: &mut PgConnection, pname: &str) -> Category {
    use crate::schema::categories::dsl::*;

    categories
        .filter(name.eq(pname))
        .first(conn)
        .expect("Cannot find the category")
}
