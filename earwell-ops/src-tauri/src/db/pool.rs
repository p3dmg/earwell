use std::path::Path;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::Connection;
use crate::error::AppResult;

pub type DbPool = Pool<SqliteConnectionManager>;

/// Open an encrypted SQLite pool. The 32-byte key comes from Argon2id over the
/// master passphrase + per-install salt (see crypto::kdf).
pub fn open_encrypted(path: &Path, key: &[u8; 32]) -> AppResult<DbPool> {
    // SQLCipher: x'<hex>' = raw key, skipping its built-in KDF (we already did Argon2id).
    let key_hex = hex::encode(key);
    let pragma = format!("PRAGMA key = \"x'{}'\";", key_hex);

    let manager = SqliteConnectionManager::file(path).with_init(move |c: &mut Connection| {
        c.execute_batch(&pragma)?;
        c.execute_batch(
            "PRAGMA cipher_compatibility = 4;
             PRAGMA foreign_keys = ON;
             PRAGMA journal_mode  = WAL;
             PRAGMA synchronous   = NORMAL;
             PRAGMA busy_timeout  = 5000;",
        )?;
        // Smoke test the key — if it's wrong this will error.
        c.query_row("SELECT count(*) FROM sqlite_schema", [], |r| r.get::<_, i64>(0))?;
        Ok(())
    });

    Ok(Pool::builder().max_size(8).build(manager)?)
}
