mod api;
mod model;
mod state;

use state::AppData;

use crate::api::{add_endpoint, add_test, add_server, add_listener, clean, confirm_dialog, delete_endpoint, delete_test, delete_server, delete_listener, get_test, get_servers, get_listeners, get_tests, load, save, save_as, start_test, stop_test, update_endpoint, update_test, update_server, update_listener};

/**
 * This function is the entry point of the Tauri application.
 */
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {

    #[cfg(debug_assertions)] // only enable instrumentation in development builds
    let devtools = tauri_plugin_devtools::init();

    let mut builder = tauri::Builder::default();

    #[cfg(debug_assertions)]
    {
        builder = builder.plugin(devtools);
    }

    builder
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .manage(AppData::new())
        .invoke_handler(tauri::generate_handler![load, save, save_as, clean, 
            get_tests, get_test, update_test, add_test, delete_test,
            get_servers, update_server, add_server, delete_server,
            get_listeners, update_listener, add_listener, delete_listener, 
            add_endpoint, delete_endpoint, update_endpoint,
            confirm_dialog, 
            start_test, 
            stop_test])        
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
