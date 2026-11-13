mod api;
mod model;
mod state;

use state::AppData;

use crate::api::{
    clean, confirm_dialog, load, open_dialog, save, save_as, get_setup_tree
};

/**
 * This function is the entry point of the Tauri application.
 *
 * # Panics
 * If the Tauri application fails to run, this function will panic.
 */
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    /*
     * Initialize the devtools plugin if the application is running in debug mode.
     */
    #[cfg(debug_assertions)]
    let devtools = tauri_plugin_devtools::init();

    /*
     * Initialize the Tauri application.
     */
    let mut builder = tauri::Builder::default();

    /*
     * Add the devtools plugin to the Tauri application if the application is running in debug mode.
     */
    #[cfg(debug_assertions)]
    {
        builder = builder.plugin(devtools);
    }

    /*
     * Start the Tauri application.
     */
    #[allow(clippy::items_after_statements)]
    builder
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .manage(AppData::new())
        .invoke_handler(tauri::generate_handler![
            load,
            save,
            save_as,
            clean,
            get_setup_tree,
            confirm_dialog,
            open_dialog
        ])
        .run(tauri::generate_context!())
        .expect("Error while running tauri application");
}
