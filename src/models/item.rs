use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Serialize)]
pub struct Item {
    pub id: i64,
    pub name: String,
    pub votes: i32,
    pub category_id: i64,
}

#[derive(Deserialize)]
pub struct NewItem {
    pub name: String,
    pub category_id: i64,
}
