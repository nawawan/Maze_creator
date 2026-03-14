use super::repo_error::RepoError;
use std::fmt;

pub enum ErrorStatus {
    NotFound,
    AlreadyExist,
    InternalError,
    Unauthorized,
    Invalid,
}

impl fmt::Display for ErrorStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorStatus::NotFound => write!(f, "Not Found"),
            ErrorStatus::AlreadyExist => write!(f, "Already Exist"),
            ErrorStatus::InternalError => write!(f, "Internal Error"),
            ErrorStatus::Unauthorized => write!(f, "Unauthorized"),
            ErrorStatus::Invalid => write!(f, "Invalid"),
        }
    }
}

pub struct AppError {
    pub status: ErrorStatus,
    pub message: String,
}

impl AppError {
    pub fn internal(message: Option<&str>) -> Self {
        match message {
            Some(msg) => AppError {
                status: ErrorStatus::InternalError,
                message: msg.into(),
            },
            None => AppError {
                status: ErrorStatus::InternalError,
                message: "Internal error".into(),
            },
        }
    }

    pub fn not_found(message: Option<&str>) -> Self {
        match message {
            Some(msg) => AppError {
                status: ErrorStatus::NotFound,
                message: msg.into(),
            },
            None => AppError {
                status: ErrorStatus::NotFound,
                message: "Not found".into(),
            },
        }
    }

    pub fn unauthorized(message: Option<&str>) -> Self {
        match message {
            Some(msg) => AppError {
                status: ErrorStatus::Unauthorized,
                message: msg.into(),
            },
            None => AppError {
                status: ErrorStatus::Unauthorized,
                message: "Unauthorized".into(),
            },
        }
    }

    pub fn already_exist(message: Option<&str>) -> Self {
        match message {
            Some(msg) => AppError {
                status: ErrorStatus::AlreadyExist,
                message: msg.into(),
            },
            None => AppError {
                status: ErrorStatus::AlreadyExist,
                message: "Already exist".into(),
            },
        }
    }

    pub fn invalid(message: Option<&str>) -> Self {
        match message {
            Some(msg) => AppError {
                status: ErrorStatus::Invalid,
                message: msg.into(),
            },
            None => AppError {
                status: ErrorStatus::Invalid,
                message: "Invalid request".into(),
            },
        }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "status: {}, message: {}", self.status, self.message)
    }
}

impl From<RepoError> for AppError {
    fn from(error: RepoError) -> Self {
        match error {
            RepoError::Conflict(e) => AppError::already_exist(Some(&e.to_string())),
            RepoError::Internal(e) => AppError::internal(Some(&e.to_string())),
            RepoError::NotFound(e) => AppError::not_found(Some(&e.to_string())),
        }
    }
}
