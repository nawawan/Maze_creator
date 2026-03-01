use usecase::repository::base_repository::BaseRepository;
use super::super::repository::*;
use async_trait::async_trait;
use anyhow::Result;

#[async_trait]
impl BaseRepository for Repository {
    async fn create_transaction(&self) -> Result<sqlx::Transaction<'_, sqlx::Postgres>> {
        let tx = self.pool.begin().await?;
        Ok(tx)
    }
}