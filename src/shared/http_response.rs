use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use serde;
use serde::Serialize;

use crate::shared::apperror::{AppError, ErrorKind};

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    fields: Vec<FieldError>,
}

#[derive(Serialize)]
struct FieldError {
    pub field: String,
    pub message: String,
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self.kind {
            ErrorKind::NotFound => StatusCode::NOT_FOUND,
            ErrorKind::Conflict => StatusCode::CONFLICT,
            ErrorKind::Validation => StatusCode::UNPROCESSABLE_ENTITY,
            ErrorKind::Unauthorized => StatusCode::UNAUTHORIZED,
            ErrorKind::Forbidden => StatusCode::FORBIDDEN,
            ErrorKind::Internal => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(ErrorResponse {
            error: self
                .message
                .unwrap_or(self.kind.public_message())
                .to_string(),
            fields: self
                .fields
                .iter()
                .map(|f| FieldError {
                    field: f.field.clone(),
                    message: f.message.clone(),
                })
                .collect(),
        })
    }
}
