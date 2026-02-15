
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

use crate::error::UsecaseError;
use crate::model::user::LoginRequest;

use super::handler::Handler;
use tracing::{warn};
use axum::{extract::{Json, State}};
use std::sync::Arc;
use usecase::service::service::Service;
use usecase::service::user::user_service::UserService;

impl Handler {
    pub async fn login_admin(&self, Json(req): Json<LoginRequest>, state: State<Arc<Service>>) -> Response {
        let service = state.0.clone();

        let (username, password) = (req.username, req.password);

        let res = service.login(&username, &password).await;

        if let Err(mut e) = res {
            warn!("Login failed: {}", e.message);
            e.message = "Invalid username or password".to_string();
            return UsecaseError { error: e }.into_response();
        }

        StatusCode::OK.into_response()
    }
}