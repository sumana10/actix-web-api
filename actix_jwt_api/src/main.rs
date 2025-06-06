mod database;
mod errors;
mod handlers;
mod models;

use actix_web::{web, App, HttpServer, middleware::Logger};
use handlers::*;



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let pool = database::create_pool()
        .await
        .expect("Failed to create database pool");

    println!("Database connected successfully");

    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(Logger::default())
            .route("/auth/register", web::post().to(register))
            .route("/auth/login", web::post().to(login))
            .route("/notes", web::post().to(create_note_handler))
            .route("/notes", web::get().to(get_notes_handler))
            .route("/notes/{id}", web::get().to(get_note_handler))
            .route("/notes/{id}", web::put().to(update_note_handler))
            .route("/notes/{id}", web::delete().to(delete_note_handler))
            .route("/health", web::get().to(health_check))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}


