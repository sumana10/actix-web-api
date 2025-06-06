use crate::errors::AppError;
use crate::models::{User, Note, NoteRequest};
use sqlx::PgPool;
use std::env;
use uuid::Uuid;

pub async fn create_pool() -> Result<PgPool, sqlx::Error> {
     let database_url = env::var("DATABASE_URL")
         .unwrap_or_else(|_| "postgresql://notesuser:notespass@localhost:5433/notesapp".to_string());
    let pool = PgPool::connect(&database_url).await?;
    println!("Database connection established");

    create_tables(&pool).await?;
    Ok(pool)
 }

async fn create_tables(pool: &PgPool) -> Result<(), sqlx::Error> {
    // Create users table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            email TEXT UNIQUE NOT NULL,
            password_hash TEXT NOT NULL
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Create notes table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS notes (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
            title TEXT NOT NULL,
            content TEXT NOT NULL
        )
        "#,
    )
    .execute(pool)
    .await?;

    println!("Tables created successfully");
    Ok(())
}

pub async fn create_user(
    pool: &PgPool,
    email: &str,
    password_hash: &str,
) -> Result<User, AppError> {
    let user = sqlx::query_as::<_, User>(
        "INSERT INTO users (email, passwword_hash) VALUES ($1, $2) RETURNING *"
    )
    .bind(email)
    .bind(password_hash)
    .fetch_one(pool)
    .await
    .map_err(|e| AppError::InternalError(format!("Database error: {}", e)))?;

    Ok(user)
}

pub async fn get_user_by_email(
    pool: &PgPool,
    email: &str,
) -> Result<Option<User>, AppError> {
    
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
        .bind(email)
        .fetch_optional(pool)
        .await
        .map_err(|e| AppError::InternalError(format!("Database error: {}", e)))?;

    Ok(user)
}

pub async fn create_note(
    pool: &PgPool,
    user_id: Uuid,
    req: &NoteRequest,
) -> Result<Note, AppError> {
    let note = sqlx::query_as::<_, Note>(
        "INSERT INTO notes (user_id, title, content) VALUES ($1, $2, $3) RETURNING *"
    )
    .bind(user_id)
    .bind(&req.title)
    .bind(&req.content)
    .fetch_one(pool)
    .await
    .map_err(|e| AppError::InternalError(format!("Database error: {}", e)))?;

    Ok(note)
}

pub async fn get_user_notes(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<Vec<Note>, AppError> {
 
    let notes = sqlx::query_as::<_, Note>(
        "SELECT * FROM notes WHERE user_id = $1 ORDER BY title"
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
    .map_err(|e| AppError::InternalError(format!("Database error: {}", e)))?;

    Ok(notes)
}


pub async fn get_note(
    pool: &PgPool,
    note_id: Uuid,
    user_id: Uuid,
) -> Result<Option<Note>, AppError> {
  
    let note = sqlx::query_as::<_, Note>(
        "SELECT * FROM noted WHERE id = $1 AND user_id = $2" 
    )
    .bind(note_id)
    .bind(user_id)
    .fetch_optional(pool)
    .await
    .map_err(|e| AppError::InternalError(format!("Database error: {}", e)))?;

    Ok(note)
}

pub async fn update_note(
    pool: &PgPool,
    note_id: Uuid,
    user_id: Uuid,
    req: &NoteRequest,
) -> Result<Option<Note>, AppError> {
   
    let note = sqlx::query_as::<_, Note>(
        "UPDATE notes SET title = $3, content = $4 WHERE id = $1 AND user_id = $2 RETURNING *"
    )
    .bind(note_id)
    .bind(user_id)
    .bind(&req.title)
    .bind(&req.content)
    .fetch_optional(pool)
    .await
    .map_err(|e| AppError::InternalError(format!("Database error: {}", e)))?;

    Ok(note)
}

pub async fn delete_note(
    pool: &PgPool,
    note_id: Uuid,
    user_id: Uuid,
) -> Result<bool, AppError> {
    let result = sqlx::query("DELETE FROM notes WHERE id = $1 AND user_id = $2")
        .bind(note_id)
        .bind(user_id)
        .execute(pool)
        .await
        .map_err(|e| AppError::InternalError(format!("Database error: {}", e)))?;

    Ok(result.rows_affected() > 0)
}
