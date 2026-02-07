use axum::{http::StatusCode, response::{IntoResponse}, Json};
use serde::Serialize;
use usecase::errors::app_error::{AppError, ErrorStatus};

pub struct UsecaseError{
    error: AppError
}

#[derive(Serialize)]
struct ErrorBody {
    code: &'static str,
    message: String
}

impl IntoResponse for UsecaseError {
    fn into_response(self) -> axum::response::Response {
        let (status, code, message) = match self.error.status {
            ErrorStatus::NotFound => (StatusCode::NOT_FOUND, "NOT_FOUND", self.error.message),
            ErrorStatus::AlreadyExist => (StatusCode::CONFLICT, "ALREADY_EXIST", self.error.message),
            ErrorStatus::InternalError => (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", self.error.message),
            ErrorStatus::Unauthorized => (StatusCode::UNAUTHORIZED, "UNAUTHORIZED", self.error.message),
        };

        (status, Json(ErrorBody{code, message})).into_response()
    }
}