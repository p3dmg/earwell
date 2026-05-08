use crate::AppResult;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use refinery::embed_migrations;
use std::path::Path;

embed_migrations!("migrations");

pub type DbPool = Pool<SqliteConnectionManager>;

pub fn open(db_path: &Path) -> AppResult<DbPool> {
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
