use std::str::FromStr;

use crate::users::model::{AccessToken, AuthorizedUserId};
use crate::redis::model::RedisValue;

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

    async fn create_token(&self, user_id: Uuid) -> Result<Token, RepoError> {
        let token = Token::new(user_id.clone());
        let key: AccessToken = token.access_token.clone().into();
        let val: AuthorizedUserId = user_id.into();
        self.redis_client.set_ex(&key, &val, 300).await?;
        Ok(token)
    }

    async fn delete_token(&self, token: Token) -> Result<u64, RepoError>{
        let key: AccessToken = token.into();
        self.redis_client.delete(key).await
    }

    async fn fetch_user_id_by_token(&self, token: Token) -> Option<Uuid> {
        let key: AccessToken = token.into();
        match self.redis_client.get(key).await {
            Ok(user_id) => {
                user_id.and_then(|uid| Uuid::from_str(&uid.inner()).ok())
            },
            Err(_) => None
        }

    }
}

#[cfg(test)]
mod tests {

    use crate::redis::RedisClient;
    use shared::config::RedisConfig;

    use super::*;
    use anyhow::Result;
    use aws_config::BehaviorVersion;
    use aws_sdk_s3::Client;
    use shared::config::Config;
    use uuid::Uuid;
    use std::env;
    use sqlx::postgres::{PgPoolOptions, PgPool};

    #[tokio::test]
    async fn succeed_in_creating_token() -> Result<(), RepoError>{
        let repo = initialize_repository().await;

        let current_user_id = Uuid::now_v7();
        let token = repo.create_token(current_user_id).await?;

        assert_eq!(current_user_id, token.id);
        Ok(())
    }

    #[tokio::test]
    async fn succeed_in_deleting_token() -> Result<(), RepoError>{
        let repo = initialize_repository().await;

        let current_user_id = Uuid::now_v7();
        let token = repo.create_token(current_user_id).await?;

        let deleted_item_num = repo.delete_token(token).await?;

        assert_eq!(1, deleted_item_num);
        Ok(())
    }

    #[tokio::test]
    async fn succeed_in_fetching_user_from_token() -> Result<(), RepoError>{
        let repo = initialize_repository().await;

        let current_user_id = Uuid::now_v7();
        let token = repo.create_token(current_user_id).await?;

        let user_id = repo.fetch_user_id_by_token(token).await;

        assert!(user_id.is_some());
        assert_eq!(current_user_id, user_id.unwrap());
        Ok(())
    }

    async fn initialize_repository() -> Repository{
        let repo = Repository::new(
            initialize_db().await,
            Client::new(&aws_config::load_defaults(BehaviorVersion::latest()).await),
            initialize_redis().await,
            Config { host: "test".into() }
        );

        repo
    }
    async fn initialize_db() -> PgPool {
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&db_url)
            .await
            .expect("Failed to connect to database");
        pool
    }

    async fn initialize_redis() -> RedisClient {
        let config = RedisConfig {
            host: env::var("REDIS_HOST").expect("REDIS_HOST must be set"),
            port: env::var("REDIS_PORT").expect("REDIS_PORT must be set"),
        };
        RedisClient::new(config).expect("creating redis client failed")
    }
}