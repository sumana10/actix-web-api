use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;
use uuid::Uuid;

#[derive(Serialize, Deserialize, ToSchema, Clone)]

struct User {
    id: Uuid,
    name: String,
    email: String,
    #[serde(skip_deserializing)]
    created_at: u64,
}

type Db = Mutex<HashMap<Uuid, User>>;
//macro is used to generate OpenAPI documentation
#[utoipa::path(
    post, // HTTP method
    path = "/users", // URL path
    request_body = User, // The expected request body (a User struct)
    responses(
        (status = 201, description = "User created", body = User) // Expected response
    )
)]

async fn create_user(user: web::Json<User>, db: web::Data<Db>) -> impl Responder {
    let mut db = db.lock().unwrap();
    let mut new_user = user.into_inner();
    new_user.id = Uuid::new_v4();
    new_user.created_at = chrono::Utc::now().timestamp() as u64;

    db.insert(new_user.id, new_user.clone());
    HttpResponse::Created().json(new_user)
}

#[utoipa::path(
    get,
    path = "/users/{id}",
    params(("id" = Uuid, Path, description = "User ID")),
    responses(
        (status = 200, description = "User found", body = User),
        (status = 404, description = "User not found")
    )
)]

async fn get_user_by_id(path: web::Path<Uuid>, db: web::Data<Db>) -> impl Responder {
    let id = path.into_inner();
    let db = db.lock().unwrap();

    if let Some(user) = db.get(&id) {
        HttpResponse::Ok().json(user)
    } else {
        HttpResponse::NotFound().body("User not found")
    }
}

#[derive(OpenApi)]
#[openapi(paths(get_user_by_id, create_user), components(schemas(User)))]
struct ApiDoc;

#[actix_web::main]

async fn main() -> std::io::Result<()> {
    println!("Server at http://localhost:8080");

    let db = web::Data::new(Mutex::new(HashMap::<Uuid, User>::new()));

    HttpServer::new(move || {
        App::new()
            .app_data(db.clone())
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-doc/openapi.json", ApiDoc::openapi()),
            )
            .route("/users/{id}", web::get().to(get_user_by_id))
            .route("/users", web::post().to(create_user))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
