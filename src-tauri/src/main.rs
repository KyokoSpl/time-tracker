//! Time Tracker Tauri Application Entry Point
//!
//! This is the main entry point for the Time Tracker desktop application.
//! It delegates to the library's run function which initializes the Tauri runtime.

// Prevents additional console window on Windows in release builds
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

fn main() {
    time_tracker_tauri_lib::run()
}