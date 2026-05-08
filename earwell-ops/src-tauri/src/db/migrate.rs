use crate::db::pool::DbPool;
use crate::error::AppResult;

mod embedded {
    refinery::embed_migrations!("./migrations");
}

pub fn run_migrations(pool: &DbPool) -> AppResult<()> {
    let mut conn = pool.get()?;
    embedded::migrations::runner().run(&mut *conn)?;
    Ok(())
}
