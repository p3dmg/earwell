use argon2::{Algorithm, Argon2, Params, Version};
use crate::error::{AppError, AppResult};

/// Derive a 32-byte DB key from passphrase + per-install salt using Argon2id.
/// 64 MiB / 3 passes / 1 lane — strong enough for an interactive desktop unlock.
pub fn derive_db_key(passphrase: &str, salt: &[u8]) -> AppResult<[u8; 32]> {
    if salt.len() < 16 {
        return Err(AppError::Crypto("salt must be >= 16 bytes".into()));
    }
    let params = Params::new(64 * 1024, 3, 1, Some(32))
        .map_err(|e| AppError::Crypto(format!("argon2 params: {e}")))?;
    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
    let mut out = [0u8; 32];
    argon2.hash_password_into(passphrase.as_bytes(), salt, &mut out)
        .map_err(|e| AppError::Crypto(format!("argon2 derive: {e}")))?;
    Ok(out)
}
