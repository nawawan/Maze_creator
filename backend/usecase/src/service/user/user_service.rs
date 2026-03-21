use async_trait::async_trait;
use std::env;
use tracing::{error, warn};
use uuid::Uuid;

use crate::model::user::{self, Token, User};

use super::super::super::errors::app_error::AppError;
use super::super::service::Service;
use super::helper;

#[async_trait]
pub trait UserService {
    async fn login(
        &self,
        username: &String,
        password: &String,
    ) -> Result<(Token, String), AppError>;
    async fn logout(&self, access_token: Token) -> Result<(), AppError>;
    async fn get_user(&self, user_id: Uuid) -> Result<User, AppError>;
    async fn fetch_user_id_by_token(&self, access_token: String) -> Result<Uuid, AppError>;
}

#[async_trait]
impl UserService for Service {
    async fn login(
        &self,
        username: &String,
        password: &String,
    ) -> Result<(Token, String), AppError> {
        let res = self.repository.get_user_by_username(username).await;
        let user = match res {
            Ok(user) => user,
            Err(e) => {
                warn!("Failed to get user: {}, error: {}", username, e);
                return Err(AppError::invalid(Some(
                    "The pair of username and password is incorrect",
                )));
            }
        };

        let pepper = env::var("PASSWORD_PEPPER").expect("A value of pepper is not set");

        let hash = match helper::hash_with_salt_pepper(password, &user.salt, &pepper) {
            Ok(hash) => hash,
            Err(e) => {
                error!("Failed to hash password: {}, error: {}", username, e);
                return Err(AppError::internal(Some(
                    "Internal error on hashing password",
                )));
            }
        };

        if hash != user.password {
            warn!("Incorrect password for user: {}", username);
            return Err(AppError::invalid(Some(
                "The pair of username and password is incorrect",
            )));
        }

        let token = self
            .repository
            .create_token(user.id, self.config.token_ttl)
            .await?;
        let refresh_token = self
            .repository
            .create_token(user.id, self.config.refresh_ttl)
            .await?;

        Ok((token, refresh_token.access_token))
    }

    async fn logout(&self, token: Token) -> Result<(), AppError> {
        self.repository.delete_token(token).await?;
        Ok(())
    }

    async fn get_user(&self, user_id: Uuid) -> Result<User, AppError> {
        let user = self.repository.get_user(user_id).await?;
        Ok(user)
    }

    async fn fetch_user_id_by_token(&self, access_token: String) -> Result<Uuid, AppError> {
        let user_id = self.repository.fetch_user_id_by_token(access_token).await;

        if let Some(uid) = user_id {
            return Ok(uid);
        }

        Err(AppError::not_found(Some("user_id is not found")))
    }
}
