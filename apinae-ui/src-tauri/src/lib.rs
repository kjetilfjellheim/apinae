mod api;
mod model;
mod state;

use state::AppData;
use tauri::{Manager, RunEvent};

use crate::api::{
    add_collection, add_endpoint, delete_endpoint, add_setup, get_setup, update_setup, add_http_server, add_tcp_listener, delete_http_server, delete_tcp_listener, clean, confirm_dialog, get_processes, get_setup_tree, load, load_settings, rename_collection, delete_collection, rename_setup, delete_setup, save_settings, open_dialog, save, save_as, start_process, stop_process
};

fn kill_all_child_processes(app_handle: &tauri::AppHandle) {
    let app_data = app_handle.state::<AppData>();
    let mut process_data = match app_data.process_data.lock() {
        Ok(guard) => guard,
        Err(_) => return,
    };

    for process in process_data.values_mut() {
        match process.process.try_wait() {
            Ok(Some(_)) => {}
            Ok(None) | Err(_) => {
                let _ = process.process.kill();
                let _ = process.process.wait();
            }
        }
    }
    process_data.clear();
}

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
    let app = builder
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .manage(AppData::new())
        .invoke_handler(tauri::generate_handler![
            load,
            save,
            save_as,
            clean,
            add_collection,
            add_setup,
            get_setup_tree,
            confirm_dialog,
            open_dialog,
            start_process,
            stop_process,
            get_processes,
            load_settings,
            save_settings,
            rename_collection,
            delete_collection,
            rename_setup,
            delete_setup,
            get_setup,
            update_setup,
            add_http_server,
            add_tcp_listener,
            delete_http_server,
            delete_tcp_listener,
            add_endpoint,
            delete_endpoint
        ])
        .build(tauri::generate_context!())
        .expect("Error while building tauri application");

    app.run(|app_handle, event| {
        if matches!(event, RunEvent::Exit | RunEvent::ExitRequested { .. }) {
            kill_all_child_processes(app_handle);
        }
    });
}
