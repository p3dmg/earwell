pub mod pool;

use crate::AppResult;
use pool::DbPool;
use refinery::embed_migrations;

embed_migrations!("migrations");

/// Run all pending refinery migrations against an already-open pool.
pub fn migrate(pool: &DbPool) -> AppResult<()> {
    let mut conn = pool.get()?;
    migrations::runner().run(&mut *conn)?;
    Ok(())
}
