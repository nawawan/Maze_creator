use super::super::model::user::*;
use async_trait::async_trait;
use anyhow::Result;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn get_user_by_username(&self, username: &String) -> Result<User>;
}