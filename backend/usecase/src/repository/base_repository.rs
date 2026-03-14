use super::super::errors::repo_error::RepoError;
use async_trait::async_trait;

#[async_trait]
pub trait BaseRepository: Send + Sync {
    async fn create_transaction(&self) -> Result<sqlx::Transaction<'_, sqlx::Postgres>, RepoError>;
}
