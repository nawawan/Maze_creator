use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::Serialize;
use usecase::errors::app_error::{AppError, ErrorStatus};

pub struct UsecaseError {
    pub error: AppError,
}

#[derive(Serialize)]
struct ErrorBody {
    code: &'static str,
    message: String,
}

impl UsecaseError {
    pub fn bad_request(message: &str) -> Self {
        UsecaseError {
            error: AppError::invalid(Some(message)),
        }
    }
    pub fn unauthorized(message: &str) -> Self {
        UsecaseError {
            error: AppError::unauthorized(Some(message)),
        }
    }
}

impl IntoResponse for UsecaseError {
    fn into_response(self) -> axum::response::Response {
        let (status, code, message) = match self.error.status {
            ErrorStatus::NotFound => (StatusCode::NOT_FOUND, "NOT_FOUND", self.error.message),
            ErrorStatus::AlreadyExist => {
                (StatusCode::CONFLICT, "ALREADY_EXIST", self.error.message)
            }
            ErrorStatus::InternalError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "INTERNAL_ERROR",
                self.error.message,
            ),
            ErrorStatus::Unauthorized => {
                (StatusCode::UNAUTHORIZED, "UNAUTHORIZED", self.error.message)
            }
            ErrorStatus::Invalid => (StatusCode::BAD_REQUEST, "INVALID", self.error.message),
        };

        (status, Json(ErrorBody { code, message })).into_response()
    }
}

impl From<AppError> for UsecaseError {
    fn from(error: AppError) -> Self {
        UsecaseError { error }
    }
}
