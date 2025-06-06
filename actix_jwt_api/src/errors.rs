use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use std::fmt;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
}

#[derive(Debug)]
pub enum AppError {
    NotFound(String),
    InternalError(String),
    BadRequest(String),
    Unauthorized(String),
    Conflict(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::NotFound(msg) => write!(f, "Not Found: {}", msg),
            AppError::InternalError(msg) => write!(f, "Internal Error: {}", msg),
            AppError::BadRequest(msg) => write!(f, "Bad Request: {}", msg),
            AppError::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg),
            AppError::Conflict(msg) => write!(f, "Conflict: {}", msg),
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
            AppError::BadRequest(msg) => {
                let response = ErrorResponse {
                    error: "bad_request".to_string(),
                    message: msg.clone(),
                };
                HttpResponse::BadRequest().json(response)
            }
            AppError::Unauthorized(msg) => {
                let response = ErrorResponse {
                    error: "unauthorized".to_string(),
                    message: msg.clone(),
                };
                HttpResponse::Unauthorized().json(response)
            }
            AppError::Conflict(msg) => {
                let response = ErrorResponse {
                    error: "conflict".to_string(),
                    message: msg.clone(),
                };
                HttpResponse::Conflict().json(response)
            }
        }
    }
}
