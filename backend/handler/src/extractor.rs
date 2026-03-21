use axum::RequestPartsExt;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum_extra::TypedHeader;
use axum_extra::extract::CookieJar;
use axum_extra::headers::Authorization;
use axum_extra::headers::authorization::Bearer;
use std::sync::Arc;

use usecase::model::user::User;
use usecase::service::service::Service;
use usecase::service::user::user_service::UserService;

use crate::error::UsecaseError;

pub struct AuthorizedUser {
    pub access_token: String,
    pub user: User,
}

impl FromRequestParts<Arc<Service>> for AuthorizedUser {
    type Rejection = UsecaseError;

    async fn from_request_parts(
        parts: &mut Parts,
        service: &Arc<Service>,
    ) -> Result<Self, Self::Rejection> {
        let jar = CookieJar::from_request_parts(parts, service)
            .await
            .map_err(|_| UsecaseError::unauthorized("unauthorized error"))?;

        let access_token = jar
            .get("session_id")
            .map(|val| val.value().to_string())
            .unwrap_or_default();

        let user_id = service
            .fetch_user_id_by_token(access_token.clone())
            .await
            .map_err(|_| UsecaseError::unauthorized("unauthorized error"))?;

        let user = service
            .get_user(user_id)
            .await
            .map_err(|_| UsecaseError::unauthorized("unauthorized error"))?;

        Ok(Self { access_token, user })
    }
}
