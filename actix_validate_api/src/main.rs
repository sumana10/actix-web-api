use actix_web::{web, App, HttpServer, HttpResponse, Result, middleware::Logger};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;
use validator::{Validate};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Validation error: {0}")]
    ValidationError(String),
    #[error("User not found")]
    UserNotFound,
    #[error("Internal server error")]
    InternalError,
}

impl actix_web::error::ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::ValidationError(msg) => HttpResponse::BadRequest().json(ErrorResponse {
                error: "validation_error".to_string(),
                message: msg.clone(),
            }),
            AppError::UserNotFound => HttpResponse::NotFound().json(ErrorResponse {
                error: "not_found".to_string(),
                message: "User not found".to_string(),
            }),
            AppError::InternalError => HttpResponse::InternalServerError().json(ErrorResponse {
                error: "internal_error".to_string(),
                message: "Something went wrong".to_string(),
            }),

        }
    }
}


#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub age: u32,
}

//Route level validation
#[derive(Debug, Deserialize, Validate)]
pub struct CreateUserJson {
    #[validate(length(min = 2, max = 30, message = "Name must be 2-50 characters"))]
    pub name: String,
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    #[validate(range(min = 18, max = 120, message = "Age must be 18-120"))]
    pub age: u32,
}

//Form Data Handling
#[derive(Debug, Deserialize, Validate)]
pub struct CreateUserForm {
    #[validate(length(min = 2, max = 50, message = "Name must be 2-50 characters"))]
    pub name: String,
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    #[validate(range(min = 18, max = 120, message = "Age must be 18-120"))]
    pub age: u32,
}

#[derive(Serialize)]
pub struct UserResponse {
    pub user: User,
    pub message: String,
    pub source: String,
}

type UserStore = Mutex<HashMap<u32, User>>;

async fn create_user_json(
    data: web::Json<CreateUserJson>,
    store: web::Data<UserStore>,
) -> Result<HttpResponse, AppError> {
    data.validate().map_err(|e| {
        AppError::ValidationError(format!("JSON validation: {:?}", e))
    })?;

    let mut users = store.lock().map_err(|_| AppError::InternalError)?;
    let id = users.len() as u32 + 1;

    let user = User {
        id,
        name: data.name.clone(),
        email: data.email.clone(),
        age: data.age,
    };

    users.insert(id, user.clone());

    Ok(HttpResponse::Created().json(UserResponse {
        user,
        message: "User created via JSON".to_string(),
        source: "json".to_string(),
    }))
}

async fn create_user_form(
    form: web::Form<CreateUserForm>,
    store: web::Data<UserStore>,
) -> Result<HttpResponse, AppError> {
    form.validate().map_err(|e| {
        AppError::ValidationError(format!("Form validation: {:?}", e))
    })?;

    let mut users = store.lock().map_err(|_| AppError::InternalError)?;
    let id = users.len() as u32 + 1;

    let user = User {
        id,
        name: form.name.clone(),
        email: form.email.clone(),
        age: form.age,
    };

    users.insert(id, user.clone());

    Ok(HttpResponse::Created().json(UserResponse {
        user,
        message: "User created via Form".to_string(),
        source: "form".to_string(),
    }))
}

async fn get_user(
    path: web::Path<u32>,
    store: web::Data<UserStore>,
) -> Result<HttpResponse, AppError> {
    let user_id = path.into_inner();
    let users = store.lock().map_err(|_| AppError::InternalError)?;

    match users.get(&user_id) {
        Some(user) => Ok(HttpResponse::Ok().json(user)),
        None => Err(AppError::UserNotFound),
    }
}

async fn list_users(store: web::Data<UserStore>) -> Result<HttpResponse, AppError> {
    let users = store.lock().map_err(|_| AppError::InternalError)?;
    let users_vec: Vec<User> = users.values().cloned().collect();

    Ok(HttpResponse::Ok().json(users_vec))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let user_store = web::Data::new(UserStore::new(HashMap::new()));

    HttpServer::new(move || {
        App::new()
            .app_data(user_store.clone())
            .wrap(Logger::default())
            .route("/users/json", web::post().to(create_user_json))
            .route("/users/form", web::post().to(create_user_form))
            .route("/users", web::get().to(list_users))
            .route("/users/{id}", web::get().to(get_user))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
