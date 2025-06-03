use actix_web::{App, HttpServer, middleware::Logger, web};

mod database;
mod errors;
mod handlers;
mod models;

use database::create_pool;
use handlers::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let pool = create_pool()
        .await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?; //pool owned by main

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone())) //pool ownership move into the closure
            .wrap(Logger::default())
            .route("/health", web::get().to(health_check))
            .service(
                web::scope("/api/v1")
                    .route("/todos", web::get().to(list_todos_handler))
                    .route("/todos", web::post().to(create_todo_handler))
                    .route("/todos/{id}", web::get().to(get_todo_handler))
                    .route("/todos/{id}", web::put().to(update_todo_handler))
                    .route("/todos/{id}", web::delete().to(delete_todo_handler)),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
