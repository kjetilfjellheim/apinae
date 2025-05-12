mod api;
mod model;
mod state;

use api::update_predefined_set;
use state::AppData;

use crate::api::{
    add_endpoint, add_listener, add_server, add_setup, clean, confirm_dialog, delete_endpoint, delete_listener, delete_server, delete_setup, get_listeners, get_servers, get_setup, get_setups, load,
    load_settings, open_dialog, save, save_as, save_settings, start_setup, stop_setup, update_endpoint, update_listener, update_server, update_setup, get_predefined_sets, add_predefined_set, delete_predefined_set,
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
            get_setups,
            get_setup,
            update_setup,
            add_setup,
            delete_setup,
            get_servers,
            update_server,
            add_server,
            delete_server,
            get_listeners,
            update_listener,
            add_listener,
            delete_listener,
            get_predefined_sets,
            add_predefined_set,
            delete_predefined_set,
            update_predefined_set,
            add_endpoint,
            delete_endpoint,
            update_endpoint,
            confirm_dialog,
            open_dialog,
            start_setup,
            stop_setup,
            save_settings,
            load_settings
        ])
        .run(tauri::generate_context!())
        .expect("Error while running tauri application");
}
