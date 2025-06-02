use actix_web::{web, HttpResponse, Result};
use sqlx::PgPool;
use crate::database;
use crate::errors::AppError;
use crate::models::{CreateTodo, UpdateTodo, Priority};

pub async fn create_todo_handler(
    pool: web::Data<PgPool>,
    todo: web::Json<CreateTodo>,
) -> Result<HttpResponse, AppError> {
    let priority = todo.priority.as_ref().unwrap_or(&Priority::Medium);
    
    let created_todo = database::create_todo(
        &pool,
        &todo.title,
        todo.description.as_deref(),
        priority,
    ).await?;
    
    Ok(HttpResponse::Created().json(created_todo))
}

pub async fn get_todo_handler(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();
    
    match database::get_todo(&pool, id).await? {
        Some(todo) => Ok(HttpResponse::Ok().json(todo)),
        None => Err(AppError::NotFound(format!("Todo with id {} not found", id))),
    }
}

pub async fn list_todos_handler(
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, AppError> {
    let todos = database::list_todos(&pool).await?;
    Ok(HttpResponse::Ok().json(todos))
}

pub async fn update_todo_handler(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
    todo: web::Json<UpdateTodo>,
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();
    
    match database::update_todo(
        &pool,
        id,
        todo.title.as_deref(),
        todo.description.as_deref(),
        todo.completed,
        todo.priority.as_ref(),
    ).await? {
        Some(updated_todo) => Ok(HttpResponse::Ok().json(updated_todo)),
        None => Err(AppError::NotFound(format!("Todo with id {} not found", id))),
    }
}

pub async fn delete_todo_handler(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();
    
    if database::delete_todo(&pool, id).await? {
        Ok(HttpResponse::NoContent().finish())
    } else {
        Err(AppError::NotFound(format!("Todo with id {} not found", id)))
    }
}
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "Todo API",
        "version": "1.0.0"
    }))
}
