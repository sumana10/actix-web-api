use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// Database models
#[derive(Debug, Serialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
}

#[derive(Debug, Serialize, FromRow)]
pub struct Note {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub content: String,
}

// Request models
#[derive(Deserialize)]
pub struct AuthRequest {
    pub email: String,
    pub password: String,
}


#[derive(Deserialize)]
pub struct NoteRequest {
    pub title: String,
    pub content: String,
}

// JWT Claims (required for JWT to work)
 #[derive(Serialize, Deserialize)]
 pub struct Claims {
    pub user_id: String,
    pub exp: usize,
 }