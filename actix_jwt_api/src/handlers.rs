use actix_web::{HttpRequest, HttpResponse, Result, web};
use bcrypt::{DEFAULT_COST, hash, verify};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::Serialize;
use sqlx::PgPool;
use std::env;
use uuid::Uuid;

use crate::database::*;
use crate::errors::AppError;
use crate::models::{AuthRequest, Claims, NoteRequest};

#[derive(Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user_id: String,
}

#[derive(Serialize)]
pub struct MessageResponse {
    pub message: String,
}

fn get_jwt_secret() -> String {
    env::var("JWT_SECRET").unwrap_or_else(|_| "mysecretkey".to_string())
}

fn create_jwt_token(user_id: &Uuid) -> Result<String, AppError> {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        user_id: user_id.to_string(),
        exp: expiration,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(get_jwt_secret().as_ref()),
    )
    .map_err(|e| AppError::InternalError(format!("Token creation failed: {}", e)))?;

    Ok(token)
}

fn extract_user_id_from_token(req: &HttpRequest) -> Result<Uuid, AppError> {
    let auth_header = req
        .headers()
        .get("Authorization")
        .ok_or_else(|| AppError::Unauthorized("Missing Authorization header".to_string()))?
        .to_str()
        .map_err(|_| AppError::Unauthorized("Invalid Authorization header".to_string()))?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or_else(|| AppError::Unauthorized("Invalid token format".to_string()))?;

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(get_jwt_secret().as_ref()),
        &Validation::default(),
    )
    .map_err(|_| AppError::Unauthorized("Invalid token".to_string()))?;

    let user_id = Uuid::parse_str(&token_data.claims.user_id)
        .map_err(|_| AppError::Unauthorized("Invalid user ID in token".to_string()))?;

    Ok(user_id)
}

pub async fn register(
    pool: web::Data<PgPool>,
    req: web::Json<AuthRequest>,
) -> Result<HttpResponse, AppError> {
    if let Some(_) = get_user_by_email(&pool, &req.email).await? {
        return Err(AppError::Conflict("User already exists".to_string()));
    }

    let password_hash = hash(&req.password, DEFAULT_COST)
        .map_err(|e| AppError::InternalError(format!("Password hashing failed: {}", e)))?;

    let user = create_user(&pool, &req.email, &password_hash).await?;

    let token = create_jwt_token(&user.id)?;

    let response = AuthResponse {
        token,
        user_id: user.id.to_string(),
    };

    Ok(HttpResponse::Created().json(response))
}

pub async fn login(
    pool: web::Data<PgPool>,
    req: web::Json<AuthRequest>,
) -> Result<HttpResponse, AppError> {
    let user = get_user_by_email(&pool, &req.email)
        .await?
        .ok_or_else(|| AppError::Unauthorized("Invalid credentials".to_string()))?;

    let is_valid = verify(&req.password, &user.password_hash)
        .map_err(|e| AppError::InternalError(format!("Password verification failed: {}", e)))?;

    if !is_valid {
        return Err(AppError::Unauthorized("Invalid credentilas".to_string()));
    }

    let token = create_jwt_token(&user.id)?;

    let response = AuthResponse {
        token,
        user_id: user.id.to_string(),
    };

    Ok(HttpResponse::Ok().json(response))
}

pub async fn create_note_handler(
    pool: web::Data<PgPool>,
    req: web::Json<NoteRequest>,
    http_req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let user_id = extract_user_id_from_token(&http_req)?;
    let note = create_note(&pool, user_id, &req).await?;
    Ok(HttpResponse::Created().json(note))
}

pub async fn get_notes_handler(
    pool: web::Data<PgPool>,
    http_req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let user_id = extract_user_id_from_token(&http_req)?;
    let notes = get_user_notes(&pool, user_id).await?;
    Ok(HttpResponse::Ok().json(notes))
}

pub async fn get_note_handler(
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    http_req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let note_id = path.into_inner();
    let user_id = extract_user_id_from_token(&http_req)?;

    let note = get_note(&pool, note_id, user_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Note not found".to_string()))?;

    Ok(HttpResponse::Ok().json(note))
}

pub async fn update_note_handler(
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    req: web::Json<NoteRequest>,
    http_req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let note_id = path.into_inner();
    let user_id = extract_user_id_from_token(&http_req)?;

    let note = update_note(&pool, note_id, user_id, &req)
        .await?
        .ok_or_else(|| AppError::NotFound("Note not found".to_string()))?;

    Ok(HttpResponse::Ok().json(note))
}

pub async fn delete_note_handler(
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    http_req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let note_id = path.into_inner();
    let user_id = extract_user_id_from_token(&http_req)?;
    let deleted = delete_note(&pool, note_id, user_id).await?;

    if !deleted {
        return Err(AppError::NotFound("Note not found".to_string()));
    }

    let response = MessageResponse {
        message: "Note deleted successfully".to_string(),
    };

    Ok(HttpResponse::Ok().json(response))
}

pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "message": "Notes API is running"
    }))
}
