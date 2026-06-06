use actix_web::{web, App, HttpServer};
use std::io;
use std::sync::Mutex;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;

#[path = "../handlers.rs"]
mod handlers;
#[path = "../models.rs"]
mod models;
#[path = "../routers.rs"]
mod routers;
#[path = "../state.rs"]
mod state;
#[path = "../db_access.rs"]
mod db_access;

use routers::*;
use state::AppState;

// https://actix.rs/docs/

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL 没有在 .env 文件里设置");

    let db_pool = PgPoolOptions::new()
        .connect(&database_url)
        .await
        .unwrap();


    let shared_data = web::Data::new(AppState {
        health_check_response: "I'm OK.".to_string(),
        visit_count: Mutex::new(0),
        db: db_pool,
        // courses: Mutex::new(vec![]),
    });
    let app = move || {
        App::new()
            // clone
            .app_data(shared_data.clone())
            .configure(general_routes)
            .configure(course_routes)
    };

    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}