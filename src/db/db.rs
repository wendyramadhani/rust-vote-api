use log::debug;
use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};

pub struct AppState {
    pub db_pool: SqlitePool,
}

pub async fn init_db() -> SqlitePool {
    debug!("masuk init db");
    dotenv::dotenv().ok();
    let db_url: String = std::env::var("DB_URL").expect("DB_URL must be set");
    SqlitePoolOptions::new()
        .connect(&db_url)
        .await
        .expect("Failed to create database pool")
}
