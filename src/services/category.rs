use crate::db::db::AppState;
use crate::models::category::{Category, NewCategory};
use actix_web::{HttpResponse, Responder, get, post, web};

#[get("/categories")]
async fn get_categories(app_state: web::Data<AppState>) -> impl Responder {
    let categories_res = sqlx::query_as::<_, Category>("SELECT * FROM categories")
        .fetch_all(&app_state.db_pool)
        .await;

    match categories_res {
        Ok(categories) => HttpResponse::Ok().json(categories),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}

#[post("/categories_ins")]
async fn add_category(
    db_pool: web::Data<AppState>,
    new_category: web::Json<NewCategory>,
) -> impl Responder {
    // Simpan item terlebih dahulu
    let res_item = sqlx::query("INSERT INTO categories (name) VALUES (?)")
        .bind(&new_category.name)
        .execute(&db_pool.db_pool)
        .await;

    match res_item {
        Ok(_) => HttpResponse::Ok().body("Category added successfully"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error category vote: {}", e)),
    }
}
