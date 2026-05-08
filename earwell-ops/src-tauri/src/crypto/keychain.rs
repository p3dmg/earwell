use keyring::Entry;
use crate::error::{AppError, AppResult};

const SERVICE: &str = "earwell-ops";

pub fn set_secret(name: &str, value: &str) -> AppResult<()> {
    Entry::new(SERVICE, name)
        .map_err(|e| AppError::Crypto(format!("keychain entry: {e}")))?
        .set_password(value)
        .map_err(|e| AppError::Crypto(format!("keychain set: {e}")))
}

pub fn get_secret(name: &str) -> AppResult<Option<String>> {
    let entry = Entry::new(SERVICE, name)
        .map_err(|e| AppError::Crypto(format!("keychain entry: {e}")))?;
    match entry.get_password() {
        Ok(v) => Ok(Some(v)),
        Err(keyring::Error::NoEntry) => Ok(None),
        Err(e) => Err(AppError::Crypto(format!("keychain get: {e}"))),
    }
}
