pub mod password;
pub mod session;

use crate::auth::session::{Role, Session};
use crate::error::{AppError, AppResult};
use crate::state::AppState;

pub fn require_session(state: &AppState) -> AppResult<Session> {
    state.session.read().unwrap().clone().ok_or(AppError::Unauthenticated)
}

pub fn require_role(state: &AppState, allowed: &[Role]) -> AppResult<Session> {
    let s = require_session(state)?;
    if allowed.contains(&s.role) { Ok(s) }
    else { Err(AppError::Forbidden { required: "elevated role" }) }
}
