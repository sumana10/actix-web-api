use crate::database;
use crate::errors::AppError;
use crate::models::{CreateTodo, Priority, UpdateTodo};
use actix_web::{HttpResponse, Result, web};
use sqlx::PgPool;

#[utoipa::path(
    post,
    path = "/todos",
    request_body = CreateTodo,
    responses(
        (status = 201, description = "Todo created successfully", body = Todo),
        (status = 400, description = "Bad request", body = ErrorResponse),
        (status = 404, description = "Todo not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "todos"
)]

pub async fn create_todo_handler(
    pool: web::Data<PgPool>,
    todo: web::Json<CreateTodo>,
) -> Result<HttpResponse, AppError> {
    // // Convert Option<Priority> to &Priority with default
    let priority = todo.priority.as_ref().unwrap_or(&Priority::Medium);

    let created_todo = database::create_todo(
        pool.get_ref(),
        &todo.title,
        todo.description.as_deref(),
        priority,
    )
    .await?;

    Ok(HttpResponse::Created().json(created_todo))
}

#[utoipa::path(
    get,
    path = "/todos/{id}",
    responses(
        (status = 200, description = "Todo found successfully", body = Todo),
        (status = 404, description = "Todo not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse),
    ),
    params(
        ("id" = i32, Path, description = "Todo ID")
    ),
    tag = "todos"
)]
pub async fn get_todo_handler(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();

    match database::get_todo(pool.get_ref(), id).await? {
        Some(todo) => Ok(HttpResponse::Ok().json(todo)),
        None => Err(AppError::NotFound(format!("Todo with id {} not found", id))),
    }
}

#[utoipa::path(
    get,
    path = "/todos",
    responses(
        (status = 200, description = "List of todos retrieved successfully", body = [Todo]),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "todos"
)]
pub async fn list_todos_handler(pool: web::Data<PgPool>) -> Result<HttpResponse, AppError> {
    let todos = database::list_todos(pool.get_ref()).await?;
    Ok(HttpResponse::Ok().json(todos))
}

#[utoipa::path(
    put,
    path = "/todos/{id}",
    request_body = UpdateTodo,
    responses(
        (status = 200, description = "Todo updated successfully", body = Todo),
        (status = 404, description = "Todo not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse),
    ),
    params(
        ("id" = i32, Path, description = "Todo ID")
    ),
    tag = "todos"
)]
pub async fn update_todo_handler(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
    todo: web::Json<UpdateTodo>,
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();

    match database::update_todo(
        pool.get_ref(),
        id,
        todo.title.as_deref(),
        todo.description.as_deref(),
        todo.completed,
        todo.priority.as_ref(),
    )
    .await?
    {
        Some(updated_todo) => Ok(HttpResponse::Ok().json(updated_todo)),
        None => Err(AppError::NotFound(format!("Todo with id {} not found", id))),
    }
}

#[utoipa::path(
    delete,
    path = "/todos/{id}",
    responses(
        (status = 204, description = "Todo deleted successfully"),
        (status = 404, description = "Todo not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    params(
        ("id" = i32, Path, description = "Todo ID")
    ),
    tag = "todos"
)]
pub async fn delete_todo_handler(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();
    //    // pool.get_ref() converts web::Data<PgPool> -> &PgPool

    if database::delete_todo(pool.get_ref(), id).await? {
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
// as_ref(): Option<T> → Option<&T> (borrow the value)
// as_deref(): Option<String> → Option<&str> (borrow the dereferenced content)
