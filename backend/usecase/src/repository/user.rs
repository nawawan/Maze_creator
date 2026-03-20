use crate::errors::repo_error::RepoError;

use super::super::model::user::*;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn get_user_by_username(&self, username: &String) -> Result<User, RepoError>;
    async fn create_token(&self, user_id: Uuid) -> Token;
    async fn delete_token(&self, token: Token) -> Result<(), RepoError>;
    async fn fetch_user_by_token(&self, token: Token);
}
