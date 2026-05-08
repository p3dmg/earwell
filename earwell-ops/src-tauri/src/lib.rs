mod audit;
mod auth;
mod commands;
mod crypto;
mod db;
mod error;
mod state;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .compact()
        .init();

    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::default().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .manage(state::AppState::new())
        .invoke_handler(tauri::generate_handler![
            commands::system::app_status,
            commands::system::setup,
            commands::system::unlock,
            commands::system::lock,
            commands::system::audit_verify_chain,
            commands::auth::login,
            commands::auth::logout,
            commands::auth::current_session,
            commands::auth::create_user,
        ])
        .run(tauri::generate_context!())
        .expect("error while running earwell-ops");
}
