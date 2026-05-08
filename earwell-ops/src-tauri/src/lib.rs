pub mod audit;
pub mod auth;
pub mod commands;
pub mod crypto;
pub mod db;
mod error;
mod state;

pub use error::{AppError, AppResult};
pub use state::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(
            tauri_plugin_log::Builder::new()
                .level(tauri_plugin_log::log::LevelFilter::Info)
                .build(),
        )
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(AppState::new())
        .invoke_handler(tauri::generate_handler![
            commands::app_status,
            commands::setup,
            commands::unlock,
            commands::lock,
            commands::audit_verify_chain,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
