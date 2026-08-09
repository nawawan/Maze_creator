use crate::{error::UsecaseError, extractor::AuthorizedUser};

pub fn validate_admin(user: &AuthorizedUser) -> Result<(), UsecaseError> {
    if user.user.role == "admin" {
        Ok(())
    } else {
        Err(UsecaseError::permission_denied(
            "User does not have admin role",
        ))
    }
}
