use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct Category {
    pub id: i64,
    pub name: String,
}

#[derive(Deserialize)]
pub struct NewCategory {
    pub name: String,
}
