use crate::error::UsecaseError;
use crate::extractor::AuthorizedUser;
use crate::model::user::{LoginRequest, LoginResponse};

use axum::http::StatusCode;
use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};
use usecase::model::user::Token;
use usecase::service::service::Service;
use usecase::service::user::user_service::UserService;

use super::handler::Handler;
use axum::extract::{Json, State};
use std::sync::Arc;
use tracing::error;

impl Handler {
    pub async fn login_admin(
        jar: CookieJar,
        state: State<Arc<Service>>,
        Json(req): Json<LoginRequest>,
    ) -> Result<(CookieJar, Json<LoginResponse>), UsecaseError> {
        let service = state.0.clone();

        let (username, password) = (req.username, req.password);

        let result = service.login(&username, &password).await;
        if let Err(ref e) = result {
            error!("Failed to login: {}", e.message);
        }
        let (token, refresh_token) = result?;

        let session_cookie = Cookie::build(("session_id", token.access_token.clone()))
            .path("/")
            .http_only(true)
            .secure(true)
            .same_site(SameSite::Strict)
            .build();

        let refresh_cookie = Cookie::build(("refresh_token", refresh_token))
            .path("/users")
            .http_only(true)
            .secure(true)
            .same_site(SameSite::Strict)
            .build();

        let jar = jar.add(session_cookie).add(refresh_cookie);
        Ok((jar, Json(token.into())))
    }

    pub async fn logout(
        jar: CookieJar,
        user: AuthorizedUser,
        state: State<Arc<Service>>,
    ) -> Result<(CookieJar, StatusCode), UsecaseError> {
        let service = state.0.clone();
        let token = Token {
            id: user.user.id,
            access_token: user.access_token,
        };
        service.logout(token).await?;

        let jar = jar.remove("session_id").remove("refresh_token");

        Ok((jar, StatusCode::NO_CONTENT))
    }
}
