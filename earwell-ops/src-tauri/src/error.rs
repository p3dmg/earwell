use serde::{Serialize, Serializer};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("database error: {0}")]
    Db(#[from] rusqlite::Error),
    #[error("connection pool error: {0}")]
    Pool(#[from] r2d2::Error),
    #[error("migration error: {0}")]
    Migrate(#[from] refinery::Error),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("crypto error: {0}")]
    Crypto(String),
    #[error("auth error: {0}")]
    Auth(String),
    #[error("permission denied (requires {required})")]
    Forbidden { required: &'static str },
    #[error("not found: {0}")]
    NotFound(String),
    #[error("invalid input: {0}")]
    Invalid(String),
    #[error("locked")]
    Locked,
    #[error("unauthenticated")]
    Unauthenticated,
    #[error("internal: {0}")]
    Internal(String),
}

// Tauri IPC requires Serialize for Result::Err.
impl Serialize for AppError {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&self.to_string())
    }
}

pub type AppResult<T> = Result<T, AppError>;
