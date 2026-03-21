use crate::error::UsecaseError;
use crate::model::user::{LoginRequest, LoginResponse};
use crate::extractor::AuthorizedUser;

use axum::http::StatusCode;
use usecase::model::user::Token;
use usecase::service::service::Service;
use usecase::service::user::user_service::UserService;

use super::handler::Handler;
use axum::extract::{Json, State};
use std::sync::Arc;
use tracing::error;

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

    pub async fn logout(
        user: AuthorizedUser, 
        state: State<Arc<Service>>
    ) -> Result<StatusCode, UsecaseError> {
        let service = state.0.clone();
        let token = Token {
            id: user.user.id,
            access_token: user.access_token
        };
        service.logout(token).await?;

        Ok(StatusCode::NO_CONTENT)
    }
}
