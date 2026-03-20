use async_trait::async_trait;
use std::env;
use tracing::{error, warn};

use crate::model::user::{Token, User};

use super::super::super::errors::app_error::AppError;
use super::super::service::Service;
use super::helper;

#[async_trait]
pub trait UserService {
    async fn login(&self, username: &String, password: &String) -> Result<Token, AppError>;
}

#[async_trait]
impl UserService for Service {
    async fn login(&self, username: &String, password: &String) -> Result<Token, AppError> {
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

        let token = self.repository.create_token(user.id).await?;

        Ok(token)
    }
}
