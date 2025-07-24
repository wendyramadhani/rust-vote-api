use crate::db;
use crate::db::db::AppState;
use crate::models::item::{Item, NewItem};
use actix_web::{HttpResponse, Responder, get, post, put, web};
use sqlx::Row;

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

#[put("/vote_up/{id}")]
async fn vote_up(db_pool: web::Data<AppState>, vote_id: web::Path<i64>) -> impl Responder {
    let res = sqlx::query("UPDATE items SET votes = votes + 1 WHERE id = ?")
        .bind(*vote_id)
        .execute(&db_pool.db_pool)
        .await;
    match res {
        Ok(_) => HttpResponse::Ok().body("Vote added successfully"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error inserting vote: {}", e)),
    }
}

#[put("/vote_down/{id}")]
async fn vote_down(db_pool: web::Data<AppState>, vote_id: web::Path<i64>) -> impl Responder {
    let vote_count = sqlx::query("SELECT votes FROM items WHERE id = ?")
        .bind(&*vote_id)
        .fetch_one(&db_pool.db_pool)
        .await;

    if vote_count.is_err() {
        return HttpResponse::NotFound().body("Vote not found");
    } else if vote_count.unwrap().get::<i32, _>("votes") <= 0 {
        return HttpResponse::Ok().body("Request ignored, vote count is already zero");
    } else {
        let res = sqlx::query("UPDATE items SET votes = votes - 1 WHERE id = ?")
            .bind(*vote_id)
            .execute(&db_pool.db_pool)
            .await;
        match res {
            Ok(_) => HttpResponse::Ok().body("Vote added successfully"),
            Err(e) => {
                HttpResponse::InternalServerError().body(format!("Error inserting vote: {}", e))
            }
        }
    }
}
