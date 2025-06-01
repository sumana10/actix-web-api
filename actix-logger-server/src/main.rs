mod middleware;
mod handlers;
use actix_web::{web, App, HttpServer, middleware::Logger};
use middleware::RequestLogger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    println!("🚀 Starting server at http://localhost:8080");
    println!("📊 Request logging middleware is active");
    println!("\nAvailable endpoints:");
    println!("  GET    /              - Hello world");
    println!("  GET    /health        - Health check");
    println!("  GET    /users         - Get all users");
    println!("  GET    /users/{{id}}    - Get user by ID");
    println!("  POST   /users         - Create new user");
    println!("  DELETE /users/{{id}}    - Delete user by ID");

    HttpServer::new(|| {
        App::new()
            .wrap(RequestLogger)
            .wrap(Logger::default())
            .route("/", web::get().to(handlers::hello))
            .route("/health", web::get().to(handlers::health_check))
            .service(
                web::scope("/users")
                    .route("", web::get().to(handlers::get_users))
                    .route("", web::post().to(handlers::create_user))
                    .route("/{id}", web::get().to(handlers::get_user))
                    .route("/{id}", web::delete().to(handlers::delete_user))
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
