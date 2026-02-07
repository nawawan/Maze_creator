
pub enum ErrorStatus {
    NotFound,
    AlreadyExist,
    InternalError,
    Unauthorized,
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
            }
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
            }
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
            }
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
            }
        }
    }
}