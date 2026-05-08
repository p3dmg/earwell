use rusqlite::params;
use serde::Deserialize;
use tauri::State;

use crate::audit::chain;
use crate::auth::{password, require_role, session::{Role, Session}};
use crate::error::{AppError, AppResult};
use crate::state::AppState;

#[derive(Debug, Deserialize)]
pub struct LoginArgs { pub username: String, pub password: String }

#[tauri::command]
pub async fn login(state: State<'_, AppState>, args: LoginArgs) -> AppResult<Session> {
    let row = {
        let g = state.db.read().unwrap();
        let pool = g.as_ref().ok_or(AppError::Locked)?;
        let conn = pool.get()?;
        conn.query_row(
            "SELECT id, full_name, role, password_hash, is_active FROM users WHERE username = ?1",
            params![args.username],
            |r| Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?, r.get::<_, String>(2)?,
                    r.get::<_, String>(3)?, r.get::<_, i32>(4)?)),
        ).ok()
    };
    let (id, full_name, role_db, hash, is_active) = row.ok_or(AppError::Auth("invalid credentials".into()))?;
    if is_active == 0 { return Err(AppError::Auth("account is disabled".into())); }
    if !password::verify(&args.password, &hash)? { return Err(AppError::Auth("invalid credentials".into())); }
    let role = Role::from_db(&role_db).ok_or_else(|| AppError::Internal("bad role".into()))?;

    let session = Session {
        user_id: id.clone(),
        username: args.username.clone(),
        full_name, role,
        started_at: chrono::Utc::now(),
    };

    {
        let g = state.db.read().unwrap();
        let pool = g.as_ref().ok_or(AppError::Locked)?;
        let conn = pool.get()?;
        chain::append(&*conn, &chain::AuditEntry {
            action: "auth.login",
            actor_id: Some(&id), entity_id: None,
            payload: serde_json::json!({ "username": args.username }),
        })?;
    }

    *state.session.write().unwrap() = Some(session.clone());
    Ok(session)
}

#[tauri::command]
pub async fn logout(state: State<'_, AppState>) -> AppResult<()> {
    let s = state.session.read().unwrap().clone();
    if let Some(s) = s {
        if let Some(pool) = state.db.read().unwrap().as_ref() {
            let conn = pool.get()?;
            chain::append(&*conn, &chain::AuditEntry {
                action: "auth.logout",
                actor_id: Some(&s.user_id), entity_id: None,
                payload: serde_json::json!({}),
            })?;
        }
    }
    *state.session.write().unwrap() = None;
    Ok(())
}

#[tauri::command]
pub async fn current_session(state: State<'_, AppState>) -> AppResult<Option<Session>> {
    Ok(state.session.read().unwrap().clone())
}

#[derive(Debug, Deserialize)]
pub struct CreateUserArgs {
    pub username:  String,
    pub full_name: String,
    pub role:      Role,
    pub password:  String,
}

#[tauri::command]
pub async fn create_user(state: State<'_, AppState>, args: CreateUserArgs) -> AppResult<()> {
    let actor = require_role(&state, &[Role::Admin])?;
    if args.password.len() < 10 { return Err(AppError::Invalid("password must be >= 10 chars".into())); }
    let g = state.db.read().unwrap();
    let pool = g.as_ref().ok_or(AppError::Locked)?;
    let conn = pool.get()?;
    let id = uuid::Uuid::now_v7().to_string();
    let hash = password::hash(&args.password)?;
    let now = chrono::Utc::now().to_rfc3339();
    conn.execute(
        "INSERT INTO users (id, username, full_name, role, password_hash, created_at, updated_at)
         VALUES (?1,?2,?3,?4,?5,?6,?6)",
        params![id, args.username, args.full_name, args.role.as_db(), hash, now],
    )?;
    chain::append(&*conn, &chain::AuditEntry {
        action: "user.create",
        actor_id: Some(&actor.user_id), entity_id: None,
        payload: serde_json::json!({
            "user_id": id, "username": args.username, "role": args.role.as_db(),
        }),
    })?;
    Ok(())
}
