pub mod auth;

use std::path::PathBuf;
use rand::RngCore;
use rusqlite::params;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager, State};

use crate::audit::chain;
use crate::auth::{password, session::{Role, Session}};
use crate::crypto::kdf;
use crate::db::{migrate, pool::open_encrypted};
use crate::error::{AppError, AppResult};
use crate::state::AppState;

const DB_FILENAME:   &str = "earwell.db";
const SALT_FILENAME: &str = "salt.bin";

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AppPhase { NeedsSetup, Locked, Unlocked, Active }

#[derive(Debug, Serialize)]
pub struct AppStatus {
    pub phase:      AppPhase,
    pub session:    Option<Session>,
    pub db_path:    String,
    pub user_count: Option<u32>,
}

fn data_dir(app: &AppHandle) -> AppResult<PathBuf> {
    let dir = app.path().app_data_dir()
        .map_err(|e| AppError::Internal(format!("data dir: {e}")))?;
    std::fs::create_dir_all(&dir)?;
    Ok(dir)
}
fn db_path  (app: &AppHandle) -> AppResult<PathBuf> { Ok(data_dir(app)?.join(DB_FILENAME)) }
fn salt_path(app: &AppHandle) -> AppResult<PathBuf> { Ok(data_dir(app)?.join(SALT_FILENAME)) }

#[tauri::command]
pub async fn app_status(app: AppHandle, state: State<'_, AppState>) -> AppResult<AppStatus> {
    let db_p   = db_path(&app)?;
    let salt_p = salt_path(&app)?;
    let phase = if !db_p.exists() || !salt_p.exists() {
        AppPhase::NeedsSetup
    } else if state.db.read().unwrap().is_none() {
        AppPhase::Locked
    } else if state.session.read().unwrap().is_some() {
        AppPhase::Active
    } else {
        AppPhase::Unlocked
    };
    let session = state.session.read().unwrap().clone();
    let user_count = if let Some(pool) = state.db.read().unwrap().as_ref() {
        let conn = pool.get()?;
        Some(conn.query_row("SELECT COUNT(*) FROM users", [], |r| r.get::<_, u32>(0))?)
    } else { None };
    Ok(AppStatus { phase, session, db_path: db_p.to_string_lossy().into(), user_count })
}

#[derive(Debug, Deserialize)]
pub struct SetupArgs {
    pub passphrase:       String,
    pub admin_username:   String,
    pub admin_full_name:  String,
    pub admin_password:   String,
}

#[tauri::command]
pub async fn setup(app: AppHandle, state: State<'_, AppState>, args: SetupArgs) -> AppResult<AppStatus> {
    if args.passphrase.len() < 12     { return Err(AppError::Invalid("master passphrase must be >= 12 chars".into())); }
    if args.admin_password.len() < 10 { return Err(AppError::Invalid("admin password must be >= 10 chars".into())); }

    let db_p = db_path(&app)?;
    let salt_p = salt_path(&app)?;
    if db_p.exists() { return Err(AppError::Invalid("database already exists; use unlock".into())); }

    let mut salt = [0u8; 32];
    rand::rngs::OsRng.fill_bytes(&mut salt);
    std::fs::write(&salt_p, salt)?;

    let key = kdf::derive_db_key(&args.passphrase, &salt)?;
    let pool = open_encrypted(&db_p, &key)?;
    migrate::run_migrations(&pool)?;

    let admin_id = uuid::Uuid::now_v7().to_string();
    let admin_hash = password::hash(&args.admin_password)?;
    let now = chrono::Utc::now().to_rfc3339();
    {
        let conn = pool.get()?;
        conn.execute(
            "INSERT INTO users (id, username, full_name, role, password_hash, created_at, updated_at)
             VALUES (?1, ?2, ?3, 'admin', ?4, ?5, ?5)",
            params![admin_id, args.admin_username, args.admin_full_name, admin_hash, now],
        )?;
        chain::append(&*conn, &chain::AuditEntry {
            action: "system.setup",
            actor_id:  Some(&admin_id),
            entity_id: None,
            payload:   serde_json::json!({ "admin_username": args.admin_username }),
        })?;
    }

    *state.db.write().unwrap() = Some(pool);
    *state.session.write().unwrap() = Some(Session {
        user_id:    admin_id,
        username:   args.admin_username,
        full_name:  args.admin_full_name,
        role:       Role::Admin,
        started_at: chrono::Utc::now(),
    });

    app_status(app, state).await
}

#[derive(Debug, Deserialize)]
pub struct UnlockArgs { pub passphrase: String }

#[tauri::command]
pub async fn unlock(app: AppHandle, state: State<'_, AppState>, args: UnlockArgs) -> AppResult<AppStatus> {
    let db_p   = db_path(&app)?;
    let salt_p = salt_path(&app)?;
    if !db_p.exists() || !salt_p.exists() {
        return Err(AppError::Invalid("no database; run setup first".into()));
    }
    let salt = std::fs::read(&salt_p)?;
    let key = kdf::derive_db_key(&args.passphrase, &salt)?;
    let pool = open_encrypted(&db_p, &key)
        .map_err(|_| AppError::Auth("incorrect passphrase".into()))?;
    *state.db.write().unwrap() = Some(pool);
    app_status(app, state).await
}

#[tauri::command]
pub async fn lock(app: AppHandle, state: State<'_, AppState>) -> AppResult<AppStatus> {
    *state.session.write().unwrap() = None;
    *state.db.write().unwrap() = None;
    app_status(app, state).await
}

#[tauri::command]
pub async fn audit_verify_chain(state: State<'_, AppState>) -> AppResult<(u64, Option<i64>)> {
    let g = state.db.read().unwrap();
    let pool = g.as_ref().ok_or(AppError::Locked)?;
    let conn = pool.get()?;
    chain::verify(&*conn)
}
