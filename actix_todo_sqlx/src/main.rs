use actix_web::{App, HttpServer, middleware::Logger, web};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod database;
mod errors;
mod handlers;
mod models;

use database::create_pool;
use handlers::*;

#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::list_todos_handler,
        handlers::create_todo_handler,
        handlers::get_todo_handler,
        handlers::update_todo_handler,
        handlers::delete_todo_handler,
    ),
    components(
        schemas(
            models::Todo,
            models::CreateTodo,
            models::UpdateTodo,
            models::Priority,
            models::TodoStats,
            models::PriorityStats,
            errors::ErrorResponse,
        )
    ),
    tags(
        (name = "todos", description = "Todo management API"),
        (name = "health", description = "Health check endpoints")
    ),
    info(
        title = "Todo API",
        version = "1.0.0",
        description = "A simple Todo management API built with Actix Web and PostgreSQL"
    ),
    servers(
        (url = "/api/v1/", description="Local Server")
    )
)]
struct ApiDoc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let pool = create_pool()
        .await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?; //pool owned by main

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone())) // //pool ownership move into the closure
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
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
