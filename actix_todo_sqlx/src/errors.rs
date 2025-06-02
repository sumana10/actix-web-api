use actix_web::{HttpResponse, ResponseError};
use serde_json::json;
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    NotFound(String),
    InternalError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::NotFound(msg) => write!(f, "Not Found: {}", msg),
            AppError::InternalError(msg) => write!(f, "Internal Error: {}", msg),
        }
    }
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::NotFound(msg) => HttpResponse::NotFound().json(json!({
                "error": "not_found",
                "message": msg
            })),
          
            AppError::InternalError(msg) => HttpResponse::InternalServerError().json(json!({
                "error": "internal_error",
                "message": msg
            })),
        }
    }
}
