pub mod pool;

use crate::AppResult;
use pool::DbPool;
use r2d2::Pool;
use refinery::embed_migrations;
use std::path::Path;

embed_migrations!("migrations");

pub fn open(db_path: &Path) -> AppResult<DbPool> {
    use r2d2_sqlite::SqliteConnectionManager;

    let manager = SqliteConnectionManager::file(db_path).with_init(|conn| {
        conn.execute_batch(
            "PRAGMA journal_mode=WAL;
             PRAGMA foreign_keys=ON;
             PRAGMA busy_timeout=5000;",
        )
    });

    let pool = Pool::builder().max_size(4).build(manager)?;

    let mut conn = pool.get()?;
    migrations::runner().run(&mut *conn)?;

    Ok(pool)
}
