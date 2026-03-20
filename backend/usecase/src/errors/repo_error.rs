use std::default;

use thiserror::Error;
use redis::RedisError;


#[derive(Error, Debug)]
pub enum RepoError {
    #[error("conflict")]
    Conflict(String),
    #[error("internal")]
    Internal(String),
    #[error("not found")]
    NotFound(String),
}


impl From<RedisError> for RepoError {
    fn from(error: RedisError) -> Self {
        RepoError::Internal(error.detail().map(String::from).unwrap_or("internal".to_string()))
    }
}