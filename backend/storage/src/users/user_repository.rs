use crate::users::model::{AccessToken, AuthorizedUserId};

use super::super::repository::*;
use async_trait::async_trait;
use sqlx;
use usecase::errors::repo_error::RepoError;
use usecase::model::user::{Token, User};
use usecase::repository::user::UserRepository;
use uuid::Uuid;

#[async_trait]
impl UserRepository for Repository {
    async fn get_user_by_username(&self, username: &String) -> Result<User, RepoError> {
        let user = sqlx::query_as!(
            User,
            "SELECT id, name, password, salt FROM users WHERE name = $1",
            username
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => {
                RepoError::NotFound(format!("User with username: {} not found", username))
            }
            _ => RepoError::Internal(format!(
                "Failed to get user by username: {}, error: {}",
                username, e
            )),
        })?;

        Ok(user)
    }

    async fn create_token(&self, user_id: Uuid) -> Token {
        let token = Token::new(user_id.clone());
        let key: AccessToken = token.access_token.clone().into();
        let val: AuthorizedUserId = user_id.into();
        self.redis_client.set_ex(&key, &val, 300);
        token
    }

    async fn delete_token(&self, token: Token) {
        let key: AccessToken = token.into();
        self.redis_client.delete(key);
    }
}
