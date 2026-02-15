use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher};
use tracing::{error};

use crate::errors::app_error::AppError;

pub fn hash_with_salt_pepper(
    input: &str,
    salt: &str,
    pepper: &str,
) -> Result<String, AppError> {
    let preppered = format!("{input}{pepper}");
    let salt = SaltString::encode_b64(salt.as_bytes()).map_err(|e| {
        error!("Failed to encode salt: {}", e);
        AppError::internal(Some(&format!("Internal error on Encoding")))
    })?;
    let hash = Argon2::default()
        .hash_password(preppered.as_bytes(), &salt).map_err(|e| {
            error!("Failed to hash password with salt and pepper: {}", e);
            AppError::internal(Some(&format!("Internal error on hashing password")))
        })?
        .to_string();
    Ok(hash)
}