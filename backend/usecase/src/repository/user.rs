use crate::errors::repo_error::RepoError;

use super::super::model::user::*;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn get_user_by_username(&self, username: &String) -> Result<User, RepoError>;
    async fn create_token(&self, user_id: Uuid) -> Result<Token, RepoError>;
    async fn delete_token(&self, token: Token) -> Result<u64, RepoError>;
    async fn fetch_user_id_by_token(&self, token: Token) -> Option<Uuid>;
}
