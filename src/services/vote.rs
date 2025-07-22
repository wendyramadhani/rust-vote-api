use crate::db::db::AppState;
use crate::models::item::{Item, NewItem};
use actix_web::{HttpResponse, Responder, get, post, web};

#[get("/votes")]
async fn get_votes(app_state: web::Data<AppState>) -> impl Responder {
    let items_result = sqlx::query_as::<_, Item>("SELECT * FROM items")
        .fetch_all(&app_state.db_pool)
        .await;

    match items_result {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}

#[post("/votes_ins")]
async fn add_vote(db_pool: web::Data<AppState>, new_vote: web::Json<NewItem>) -> impl Responder {
    // Simpan item terlebih dahulu
    let res_item = sqlx::query("INSERT INTO items (name,category_id) VALUES (?,?)")
        .bind(&new_vote.name)
        .bind(&new_vote.category_id)
        .execute(&db_pool.db_pool)
        .await;

    match res_item {
        Ok(_) => HttpResponse::Ok().body("Vote added successfully"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error inserting vote: {}", e)),
    }
}
