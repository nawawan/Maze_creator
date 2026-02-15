use usecase::repository::user::{UserRepository};
use usecase::model::user::{User};
use super::super::repository::*;

use anyhow::Result;
use sqlx;
use async_trait::async_trait;

#[async_trait]
impl UserRepository for Repository {
    async fn get_user_by_username(&self, username: &String) -> Result<User> {
        let user = sqlx::query_as!(
            User,
            "SELECT name, password FROM users WHERE name = $1",
            username)
            .fetch_one(&self.pool)
            .await?;

        Ok(user)
    }
}