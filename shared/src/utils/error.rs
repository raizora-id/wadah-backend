use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::models::error::{ErrorDetails, ErrorResponse};

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Authentication error: {0}")]
    AuthenticationError(String),

    #[error("Authorization error: {0}")]
    AuthorizationError(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Resource not found: {0}")]
    NotFoundError(String),

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Service error: {0}")]
    ServiceError(String),

    #[error("Internal server error: {0}")]
    InternalServerError(String),
}

impl AppError {
    pub fn error_code(&self) -> String {
        match self {
            AppError::AuthenticationError(_) => "AUTHENTICATION_ERROR".to_string(),
            AppError::AuthorizationError(_) => "AUTHORIZATION_ERROR".to_string(),
            AppError::ValidationError(_) => "VALIDATION_ERROR".to_string(),
            AppError::NotFoundError(_) => "NOT_FOUND_ERROR".to_string(),
            AppError::DatabaseError(_) => "DATABASE_ERROR".to_string(),
            AppError::ServiceError(_) => "SERVICE_ERROR".to_string(),
            AppError::InternalServerError(_) => "INTERNAL_SERVER_ERROR".to_string(),
        }
    }

    pub fn error_message(&self) -> String {
        match self {
            AppError::AuthenticationError(msg) => msg.clone(),
            AppError::AuthorizationError(msg) => msg.clone(),
            AppError::ValidationError(msg) => msg.clone(),
            AppError::NotFoundError(msg) => msg.clone(),
            AppError::DatabaseError(msg) => msg.clone(),
            AppError::ServiceError(msg) => msg.clone(),
            AppError::InternalServerError(msg) => msg.clone(),
        }
    }
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::AuthenticationError(_) => StatusCode::UNAUTHORIZED,
            AppError::AuthorizationError(_) => StatusCode::FORBIDDEN,
            AppError::ValidationError(_) => StatusCode::BAD_REQUEST,
            AppError::NotFoundError(_) => StatusCode::NOT_FOUND,
            AppError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::ServiceError(_) => StatusCode::BAD_REQUEST,
            AppError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        let error_response = ErrorResponse {
            success: false,
            error: ErrorDetails {
                code: self.error_code(),
                message: self.error_message(),
                details: None,
            },
        };
        
        HttpResponse::build(status_code).json(error_response)
    }
}
