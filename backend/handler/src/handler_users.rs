use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

use crate::error::UsecaseError;
use usecase::errors::app_error::AppError;

use super::handler::Handler;
use tracing::{warn};

impl Handler {
    pub async fn login_admin(&self, username: String, password: String) -> Response {
        let res = self.service.login(username, password).await;
        let user = match res {
            Ok(user) => user,
            Err(e) => {
                warn!("Failed to get user: {}", username);
                e.message = "The pair of username and password is incorrect".to_string();
                let error = UsecaseError{error: e};
                return error.into_response();
            }
        };

        if user.password != password {
            warn!("Incorrect password for user: {}", username);
            let error = UsecaseError{error: AppError::invalid(Some("The pair of username and password is incorrect"))};
            return error.into_response();
        }

        StatusCode::OK.into_response()
    }
}