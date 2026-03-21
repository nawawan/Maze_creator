use crate::error::UsecaseError;
use crate::model::user::{LoginRequest, LoginResponse};

use super::handler::Handler;
use axum::extract::{Json, State};
use std::sync::Arc;
use tracing::error;
use usecase::service::service::Service;
use usecase::service::user::user_service::UserService;

impl Handler {
    pub async fn login_admin(
        state: State<Arc<Service>>,
        Json(req): Json<LoginRequest>,
    ) -> Result<Json<LoginResponse>, UsecaseError> {
        let service = state.0.clone();

        let (username, password) = (req.username, req.password);

        let result = service.login(&username, &password).await;
        if let Err(ref e) = result {
            error!("Failed to login: {}", e.message);
        }
        let token = result?;
        Ok(Json(token.into()))
    }
}
