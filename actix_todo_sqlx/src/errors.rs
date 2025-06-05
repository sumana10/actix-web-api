use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use std::fmt;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
}
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
            AppError::NotFound(msg) => {
                let response = ErrorResponse {
                    error: "not_found".to_string(),
                    message: msg.clone(),
                };
                HttpResponse::NotFound().json(response)
            }
            AppError::InternalError(msg) => {
                let response = ErrorResponse {
                    error: "internal_error".to_string(),
                    message: msg.clone(),
                };
                HttpResponse::InternalServerError().json(response)
            }
        }
    }
}
