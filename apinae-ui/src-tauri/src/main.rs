// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

/**
 * This is the main entry point for the Tauri application.
 */
fn main() {
    apinae_ui_lib::run()
}
