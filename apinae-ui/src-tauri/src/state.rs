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

#[cfg(test)]
mod test {
    use super::*;
    use std::process::Command;

    fn spawn_child() -> std::process::Child {
        #[cfg(windows)]
        {
            Command::new("cmd").args(["/C", "exit 0"]).spawn().unwrap()
        }

        #[cfg(not(windows))]
        {
            Command::new("sh").args(["-c", "exit 0"]).spawn().unwrap()
        }
    }

    #[test]
    fn app_data_new_initializes_default_state() {
        let app_data = AppData::new();

        let data = app_data.data.lock().unwrap();
        let app_configuration = data.as_ref().expect("App configuration should be initialized");
        assert_eq!(app_configuration.name, "Untitled");
        assert_eq!(app_configuration.description, "");
        assert!(app_configuration.collections.is_empty());

        drop(data);

        assert!(app_data.file_path.lock().unwrap().is_none());
        assert!(app_data.process_data.lock().unwrap().is_empty());
    }

    #[test]
    fn process_data_new_preserves_metadata_and_params() {
        let child = spawn_child();
        let process_id = child.id();
        let params = HashMap::from([("host".to_string(), "example.com".to_string()), ("port".to_string(), "8080".to_string())]);

        let mut process_data = ProcessData::new(process_id, child, "Test Collection".to_string(), "Test Setup".to_string(), params.clone());

        assert_eq!(process_data.process_id, process_id);
        assert_eq!(process_data.collection_name, "Test Collection");
        assert_eq!(process_data.setup_name, "Test Setup");
        assert_eq!(process_data.params, params);

        let _ = process_data.process.kill();
        let _ = process_data.process.wait();
    }

    #[test]
    fn app_data_process_data_can_store_process_data() {
        let app_data = AppData::new();
        let child = spawn_child();
        let process_id = child.id();
        let process_data = ProcessData::new(process_id, child, "Collection".to_string(), "Setup".to_string(), HashMap::new());

        app_data.process_data.lock().unwrap().insert(process_id, process_data);

        let mut process_data = app_data.process_data.lock().unwrap();
        let stored = process_data.remove(&process_id).unwrap();
        let mut child = stored.process;
        let _ = child.kill();
        let _ = child.wait();
    }
}
