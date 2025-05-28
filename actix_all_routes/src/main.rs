use actix_web::{web, App, HttpServer, HttpResponse, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct User {
    id: u32,
    name: String,
    email: String,
}

#[derive(Serialize, Deserialize)]
struct Post {
    id: u32,
    title: String,
    user_id: u32,
}

#[derive(Deserialize)]
struct UserQuery {
    page: Option<u32>,
    limit: Option<u32>,
    search: Option<String>,
}

#[derive(Deserialize)]
struct CreateUser {
    name: String,
    email: String,
}

// 1. Basic Routes
async fn home() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json("Welcome to API"))
}

async fn health() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json("Healthy"))
}

// 2. Path Parameters + 3. HTTP Methods (CRUD)
async fn get_user(path: web::Path<u32>) -> Result<HttpResponse> {
    let user_id = path.into_inner();
    //format!("User{}", user_id) string interpolation
    let user = User { id: user_id, name: format!("User{}", user_id), email: format!("user{}@test.com", user_id) };
    Ok(HttpResponse::Ok().json(user))
}

async fn create_user(data: web::Json<CreateUser>) -> Result<HttpResponse> {
    let user = User { id: 123, name: data.name.clone(), email: data.email.clone() };
    Ok(HttpResponse::Created().json(user))
}

async fn update_user(path: web::Path<u32>, data: web::Json<CreateUser>) -> Result<HttpResponse> {
    let user_id = path.into_inner();
    let user = User { id: user_id, name: data.name.clone(), email: data.email.clone() };
    Ok(HttpResponse::Ok().json(user))
}

async fn delete_user(path: web::Path<u32>) -> Result<HttpResponse> {
    let user_id = path.into_inner();
    Ok(HttpResponse::Ok().json(format!("User {} deleted", user_id)))
}

// 4. Query Parameters
async fn get_users(query: web::Query<UserQuery>) -> Result<HttpResponse> {
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(10);
    let search = query.search.as_deref().unwrap_or("");
    
    let users = vec![
        User { id: 1, name: format!("User1-{}", search), email: "user1@test.com".into() },
        User { id: 2, name: "User2".into(), email: "user2@test.com".into() },
    ];
    
    Ok(HttpResponse::Ok().json(format!("Page: {}, Limit: {}, Users: {:?}", page, limit, users)))
}

// 6. Nested Routes
async fn get_user_posts(path: web::Path<u32>) -> Result<HttpResponse> {
    let user_id = path.into_inner();
    let posts = vec![
        Post { id: 1, title: "First Post".into(), user_id },
        Post { id: 2, title: "Second Post".into(), user_id },
    ];
    Ok(HttpResponse::Ok().json(posts))
}

async fn get_user_post(path: web::Path<(u32, u32)>) -> Result<HttpResponse> {
    let (user_id, post_id) = path.into_inner();
    let post = Post { id: post_id, title: format!("Post {} by User {}", post_id, user_id), user_id };
    Ok(HttpResponse::Ok().json(post))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            // 1. Basic Routes
            .route("/", web::get().to(home))
            .route("/health", web::get().to(health))
            
            // 2. Path Parameters + 3. HTTP Methods
            .route("/users/{id}", web::get().to(get_user))
            .route("/users/{id}", web::put().to(update_user))
            .route("/users/{id}", web::delete().to(delete_user))
            
            // 3. HTTP Methods + 5. Query Parameters
            .route("/users", web::get().to(get_users))
            .route("/users", web::post().to(create_user))
            
            // 6. Nested Routes
            .route("/users/{id}/posts", web::get().to(get_user_posts))
            .route("/users/{user_id}/posts/{post_id}", web::get().to(get_user_post))
            
            // 4. Scoped Routes
            .service(
                web::scope("/api/v1")
                    .route("/users", web::get().to(get_users))
                    .route("/users/{id}", web::get().to(get_user))
            )
            .service(
                web::scope("/admin")
                    .route("/users", web::get().to(get_users))
                    .route("/health", web::get().to(health))
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
