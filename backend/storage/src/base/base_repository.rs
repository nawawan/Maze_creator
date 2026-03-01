use usecase::repository::base_repository::BaseRepository;
use usecase::errors::repo_error::RepoError;
use usecase::repository::types::Transaction;
use super::super::repository::*;
use async_trait::async_trait;
use tracing::error;

#[async_trait]
impl BaseRepository for Repository {
    async fn create_transaction(&self) -> Result<Transaction<'_>, RepoError> {
        let tx = self.pool.begin().await.map_err(|e| {
            error!("Failed to create transaction: {e}");
            RepoError::Internal("Failed to create transaction".to_string())
        })?;
        Ok(tx)
    }
}