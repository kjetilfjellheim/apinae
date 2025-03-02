use std::{collections::HashMap, sync::Mutex};

use apinae_lib::config::AppConfiguration;

/**
 * This struct is used to store the application data in the Tauri state.
 * It contains the application configuration, the file path of the current file, and the process data.
 */
pub struct AppData {
    // The application configuration.
    pub data: Mutex<Option<AppConfiguration>>,
    // The file path of the current file.
    pub file_path: Mutex<Option<String>>,
    // The process data.
    pub process_data: Mutex<HashMap<String, ProcessData>>,
}

impl AppData {
    /**
     * Creates a new instance of the `AppData` struct.
     * 
     * # Returns
     * The new instance of the `AppData` struct.
     */
    pub fn new() -> Self {
        Self {
            data: Mutex::new(AppConfiguration::new(String::from("Untitled"), String::new(), Vec::new()).into()),
            file_path: Mutex::new(None),
            process_data: Mutex::new(HashMap::new()),
        }
    }
}

/**
 * This struct is used to store the process data in the Tauri state.
 * It contains the process ID and the process.
 */
pub struct ProcessData {
    // The process ID.
    pub process_id: u32,
    // The process.
    pub process: std::process::Child,
}

impl ProcessData {
    /**
     * Creates a new instance of the `ProcessData` struct.
     * 
     * # Arguments
     * `process_id` - The process ID.
     * `process` - The process.
     * 
     * # Returns
     * The new instance of the `ProcessData` struct.
     */
    pub fn new(process_id: u32, process: std::process::Child) -> Self {
        Self {
            process_id,
            process
        }
    }
}