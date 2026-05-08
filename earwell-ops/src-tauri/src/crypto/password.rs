use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use rand::rngs::OsRng;

use crate::error::{AppError, AppResult};

pub fn hash(password: &str) -> AppResult<String> {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map(|h| h.to_string())
        .map_err(|e| AppError::Crypto(format!("password hash: {e}")))
}

pub fn verify(password: &str, phc: &str) -> AppResult<bool> {
    let parsed = PasswordHash::new(phc)
        .map_err(|e| AppError::Crypto(format!("password parse: {e}")))?;
    Ok(Argon2::default().verify_password(password.as_bytes(), &parsed).is_ok())
}
