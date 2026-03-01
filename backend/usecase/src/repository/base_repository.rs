use async_trait::async_trait;
use super::super::errors::repo_error::RepoError;

#[async_trait]
pub trait BaseRepository: Send + Sync {
    async fn create_transaction(&self) -> Result<sqlx::Transaction<'_, sqlx::Postgres>, RepoError>;
}