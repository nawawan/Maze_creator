use thiserror::Error;

#[derive(Error, Debug)]
pub enum RepoError {
    #[error("conflict")]
    Conflict(String),
    #[error("internal")]
    Internal(String),
    #[error("not found")]
    NotFound(String),
}
