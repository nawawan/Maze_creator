use async_trait::async_trait;
use tracing::{error, warn};

use super::super::super::errors::app_error::AppError;
use super::super::service::Service;

#[async_trait]
pub trait UserService {
    async fn login(&self, username: &String, password: &String) -> Result<(), AppError>;
}

#[async_trait]
impl UserService for Service {
    async fn login(&self, username: &String, password: &String) -> Result<(), AppError> {
        let res = self.repository.get_user_by_username(username).await;
        let user = match res {
            Ok(user) => user,
            Err(e) => {
                warn!("Failed to get user: {}", username);
                return Err(AppError::invalid(Some("The pair of username and password is incorrect")));
            }
        };

        if user.password != *password {
            warn!("Incorrect password for user: {}", username);
            return Err(AppError::invalid(Some("The pair of username and password is incorrect")));
        }

        Ok(())
    }
}