//! Time Tracker Tauri Application
//!
//! A Material Design time tracking application built with Tauri and BeerCSS.
//! Features task creation, time tracking, persistence, and export functionality.

mod commands;
mod persistence;
mod state;
mod task;

use state::AppState;

/// Runs the Tauri application with all registered commands and state.
///
/// This is the main entry point called from `main.rs`.
///
/// # Errors
/// Returns an error if the Tauri application fails to initialize or run.
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(AppState::new())
        .invoke_handler(tauri::generate_handler![
            commands::get_tasks,
            commands::add_task,
            commands::start_task,
            commands::stop_task,
            commands::reset_task,
            commands::delete_task,
            commands::export_tasks,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}