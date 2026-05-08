use std::sync::RwLock;
use crate::auth::session::Session;
use crate::db::pool::DbPool;

pub struct AppState {
    pub db:      RwLock<Option<DbPool>>,
    pub session: RwLock<Option<Session>>,
}

impl AppState {
    pub fn new() -> Self {
        Self { db: RwLock::new(None), session: RwLock::new(None) }
    }
}
