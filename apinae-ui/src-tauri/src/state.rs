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
    pub process_data: Mutex<HashMap<u32, ProcessData>>,
}

impl AppData {
    /**
     * Creates a new instance of the `AppData` struct.
     *
     * # Returns
     * The new instance of the `AppData` struct.
     */
    pub fn new() -> Self {
        Self { data: Mutex::new(AppConfiguration::new(String::from("Untitled"), String::new()).into()), file_path: Mutex::new(None), process_data: Mutex::new(HashMap::new()) }
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
    // The name of the collection.
    pub collection_name: String,
    // The name of the setup.
    pub setup_name: String,
    // The parameters of the process.
    pub params: HashMap<String, String>,
}

impl ProcessData {
    /**
     * Creates a new instance of the `ProcessData` struct.
     *
     * # Arguments
     * `process_id` - The process ID.
     * `process` - The process.
     * `collection_name` - The name of the collection.
     * `setup_name` - The name of the setup.
     * `params` - The parameters of the setup.
     *
     * # Returns
     * The new instance of the `ProcessData` struct.
     */
    pub fn new(process_id: u32, process: std::process::Child, collection_name: String, setup_name: String, params: HashMap<String, String>) -> Self {
        Self { process_id, process, collection_name, setup_name, params }
    }
}
