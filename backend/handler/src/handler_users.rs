use crate::{error::UsecaseError, model::user::UserResponse};
use crate::model::user::LoginRequest;

use super::handler::Handler;
use tracing::{error};
use axum::{extract::{Json, State}};
use std::sync::Arc;
use usecase::service::service::Service;
use usecase::service::user::user_service::UserService;

impl Handler {
    pub async fn login_admin(&self, Json(req): Json<LoginRequest>, state: State<Arc<Service>>) -> Result<Json<UserResponse>, UsecaseError>{
        let service = state.0.clone();

        let (username, password) = (req.username, req.password);

        let result = service.login(&username, &password).await;
        if let Err(ref e) = result {
            error!("Failed to login: {}", e.message);
        }
        let user = result?;
        Ok(Json(user.into()))
    }
}