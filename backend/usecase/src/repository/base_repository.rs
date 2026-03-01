use async_trait::async_trait;
use anyhow::Result;

#[async_trait]
pub trait BaseRepository: Send + Sync {
    async fn create_transaction(&self) -> Result<sqlx::Transaction<'_, sqlx::Postgres>>;
}