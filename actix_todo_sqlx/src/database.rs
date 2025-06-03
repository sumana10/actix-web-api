use crate::errors::AppError;
use crate::models::{Priority, Todo};
use chrono::Utc;
use sqlx::PgPool;
use std::env;

pub async fn create_pool() -> Result<PgPool, sqlx::Error> {
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://todouser:todopass@localhost:5433/todoapp".to_string());

    let pool = PgPool::connect(&database_url).await?;

    println!("Database connection established");

    create_tables(&pool).await?;

    Ok(pool)
}

async fn create_tables(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query(
        //Raw string literal that preserves formatting and doesn't require escaping quotes
        r#"
        CREATE TABLE IF NOT EXISTS todos (
            id SERIAL PRIMARY KEY,
            title VARCHAR(255) NOT NULL,
            description TEXT,
            completed BOOLEAN NOT NULL DEFAULT FALSE,
            priority VARCHAR(50) NOT NULL DEFAULT 'medium',
            created_at TIMESTAMPTZ DEFAULT NOW(),
            updated_at TIMESTAMPTZ DEFAULT NOW()
        )
        "#,
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn create_todo(
    pool: &PgPool,
    title: &str,
    description: Option<&str>,
    priority: &Priority,
) -> Result<Todo, AppError> {
    let now = Utc::now();

    let todo = sqlx::query_as::<_, Todo>(
        r#"
        INSERT INTO todos (title, description, completed, priority, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id, title, description, completed, priority, created_at, updated_at
        "#,
    )
    .bind(title)
    .bind(description)
    .bind(false)
    .bind(priority) //Direct enum binding
    .bind(now)
    .bind(now)
    .fetch_one(pool)
    .await
    .map_err(|e| AppError::InternalError(format!("Database error: {}", e)))?;

    Ok(todo)
}

pub async fn get_todo(pool: &PgPool, id: i32) -> Result<Option<Todo>, AppError> {
    let todo = sqlx::query_as::<_, Todo>("SELECT * FROM todos WHERE id =  $1")
        .bind(id)
        .fetch_optional(pool) // Zero or One Row
        .await
        .map_err(|e| AppError::InternalError(format!("Database error: {}", e)))?;

    Ok(todo)
}

pub async fn list_todos(pool: &PgPool) -> Result<Vec<Todo>, AppError> {
    let todos = sqlx::query_as::<_, Todo>("SELECT * FROM todos ORDER BY created_at DESC")
        .fetch_all(pool)
        .await
        .map_err(|e| AppError::InternalError(format!("Database error: {}", e)))?;

    Ok(todos)
}

pub async fn update_todo(
    pool: &PgPool,
    id: i32,
    title: Option<&str>,
    description: Option<&str>,
    completed: Option<bool>,
    priority: Option<&Priority>,
) -> Result<Option<Todo>, AppError> {
    let existing_todo = match get_todo(pool, id).await? {
        Some(todo) => todo,
        None => return Ok(None),
    };

    let new_title = title.unwrap_or(&existing_todo.title);
    let new_description = description.or(existing_todo.description.as_deref()); //Convert Option<String> to Option<&str>
    let new_completed = completed.unwrap_or(existing_todo.completed);
    let new_priority = priority.unwrap_or(&existing_todo.priority);
    let now = Utc::now();

    let todo = sqlx::query_as::<_, Todo>(
        r#"
        UPDATE todos
        SET title = $1, description = $2, completed = $3, priority = $4, updated_at = $5
        WHERE id = $6
        RETURNING id, title, description, completed, priority, created_at, updated_at
        "#,
    )
    .bind(new_title)
    .bind(new_description)
    .bind(new_completed)
    .bind(new_priority) // Direct enum binding
    .bind(now)
    .bind(id)
    .fetch_one(pool)
    .await
    .map_err(|e| AppError::InternalError(format!("Database error: {}", e)))?;

    Ok(Some(todo))
}

pub async fn delete_todo(pool: &PgPool, id: i32) -> Result<bool, AppError> {
    let result = sqlx::query("DELETE FROM todos WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await
        .map_err(|e| AppError::InternalError(format!("Database error: {}", e)))?;

    Ok(result.rows_affected() > 0)
}
