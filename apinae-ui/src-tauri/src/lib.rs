mod api;
mod model;
mod state;

use state::AppData;

use crate::api::{
    add_endpoint, add_listener, add_server, add_test, clean, confirm_dialog, delete_endpoint, delete_listener, delete_server, delete_test, get_listeners, get_servers, get_test, get_tests, load,
    load_settings, open_dialog, save, save_as, save_settings, start_test, stop_test, update_endpoint, update_listener, update_server, update_test,
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
            get_tests,
            get_test,
            update_test,
            add_test,
            delete_test,
            get_servers,
            update_server,
            add_server,
            delete_server,
            get_listeners,
            update_listener,
            add_listener,
            delete_listener,
            add_endpoint,
            delete_endpoint,
            update_endpoint,
            confirm_dialog,
            open_dialog,
            start_test,
            stop_test,
            save_settings,
            load_settings
        ])
        .run(tauri::generate_context!())
        .expect("Error while running tauri application");
}
