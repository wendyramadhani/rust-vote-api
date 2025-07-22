use actix_web::{
    App, HttpResponse, HttpServer, Responder, get,
    web::{self, Data},
};
mod db;
mod models;
use db::db::{AppState, init_db};
mod services;
use services::vote::{add_vote, get_votes};

#[get("/")]
async fn root() -> impl Responder {
    HttpResponse::Ok().body("Welcome to the Vote API!")
}

// #[post("/api/addvote")]

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    unsafe {
        std::env::set_var("RUST_LOG", "debug");
    }
    env_logger::init();
    let connect_db = init_db().await;
    let appstate = web::Data::new(AppState {
        db_pool: connect_db.clone(),
    });
    HttpServer::new(move || {
        App::new()
            .app_data(appstate.clone())
            .service(root)
            .service(get_votes)
            .service(add_vote)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
