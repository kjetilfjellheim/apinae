use std::collections::HashSet;
use std::{collections::HashMap, path::Path};

use crate::model::{CollectionRow, HttpServerRow, ProcessRow, SetupRow};
use crate::state::{ProcessData};
use crate::AppData;
use apinae_lib::config::{EndpointConfiguration, ServerConfiguration};
use apinae_lib::{
    config::{AppConfiguration, CollectionConfiguration, SetupConfiguration},
    settings::Settings,
};
use tauri::{AppHandle, State};
use tauri_plugin_dialog::{DialogExt, FilePath, MessageDialogButtons};

/**
 * Default name for new setups, servers, listeners and endpoints.
 */
const DEFAULT_NAME: &str = "Untitled";

/**
 * Gets a setup from a collection.
 * 
 * `app_data` The application data.
 * `collectionid` The collection id.
 * `setupid` The setup id.
 * 
 * Returns:
 * The setup.
 * 
 * # Errors
 * If data cannot be locked or setup cannot be found.
 */
#[tauri::command]
pub fn get_setup(app_data: State<'_, AppData>, collectionid: &str, setupid: &str) -> Result<SetupRow, String> {
    let mut app_config = get_configuration_data(&app_data)?;
    let collection = app_config.get_collection(collectionid).ok_or("Collection not found".to_string())?;
    let setup = collection.get_setup(setupid).ok_or("Setup not found".to_string())?;
    Result::Ok(SetupRow::from(setup.clone()))
}

/**
 * Adds a parameter to a setup.
 *
 * `app_data` The application data.
 * `collectionid` The collection id.
 * `setupid` The setup id.
 * `parameter` The parameter to add.
 *
 * # Errors
 * If data cannot be locked or setup cannot be found.
 */
#[tauri::command]
pub fn update_setup(app_data: State<'_, AppData>, collectionid: &str, setupid: &str, name: String, description: String, parameters: Vec<String>) -> Result<(), String> {
    let mut app_config = get_configuration_data(&app_data)?;
    let collection = app_config.get_collection(collectionid).ok_or("Collection not found".to_string())?;
    let setup = collection.get_setup(setupid).ok_or("Setup not found".to_string())?;
    setup.name = name;
    setup.description = description;
    if parameters.is_empty() {
        setup.params = None;
    } else {
        setup.params = Some(HashSet::from_iter(parameters.into_iter()));
    }
    update_data(&app_data, Some(app_config))?;
    Result::Ok(())
}

/**
 * Loads the configuration from a file.
 *
 * `app` The Tauri application handle.
 * `app_data` The application data.
 *
 * Returns:
 * The file path of the loaded configuration.
 *
 * # Errors
 * If no file is selected.
 * If the configuration file could not be loaded.
 */
#[tauri::command]
pub async fn load(app: AppHandle, app_data: State<'_, AppData>) -> Result<(), String> {
    let file_path = app.dialog().file().blocking_pick_file();
    if let Some(file_path) = file_path {
        let file_path = get_file_path(file_path)?;
        update_data(&app_data, Some(AppConfiguration::load(&file_path).map_err(|err| err.to_string())?))?;
        update_file_path(&app_data, Some(file_path.clone()))?;
    }
    return Result::Ok(());
}

/**
 * Saves the configuration to a file.
 *
 * `app` The Tauri application handle.
 * `app_data` The application data.
 *
 * # Errors
 * If the configuration file could not be saved.
 */
#[tauri::command]
pub async fn save(app: AppHandle, app_data: State<'_, AppData>) -> Result<(), String> {
    let current_file_path = get_current_file_path(&app_data);
    if let Result::Ok(current_file_path) = current_file_path {
        let data = get_configuration_data(&app_data)?;
        data.save(&current_file_path).map_err(|err| err.to_string())?;
        Result::Ok(())
    } else {
        save_as(app, app_data).await
    }
}

/**
 * Saves the configuration to a new file.
 *
 * `app` The Tauri application handle.
 * `app_data` The application data.
 *
 * # Errors
 * If no file is selected.
 * If the configuration file could not be saved.
 */
#[tauri::command]
pub async fn save_as(app: AppHandle, app_data: State<'_, AppData>) -> Result<(), String> {
    let file_path = app.dialog().file().blocking_save_file();
    if let Some(file_path) = file_path {
        let file_path = get_file_path(file_path)?;
        let data = get_configuration_data(&app_data)?;
        data.save(&file_path).map_err(|err| err.to_string())?;
        update_file_path(&app_data, Some(file_path))?;
    } 
    Result::Ok(())
}

/**
 * Cleans the configuration.
 *
 * `app` The Tauri application handle.
 * `app_data` The application data.
 *
 * Returns:
 * The cleaned configuration.
 *
 * # Errors
 * If the user cancels the operation.
 */
#[tauri::command]
pub async fn clean(app: AppHandle, app_data: State<'_, AppData>) -> Result<AppConfiguration, String> {
    if confirm_dialog(app).await {
        let new_data = AppConfiguration::new(DEFAULT_NAME.to_owned(), String::new());
        update_data(&app_data, Some(new_data))?;
        update_file_path(&app_data, None)?;
    }
    get_configuration_data(&app_data)
}

/**
 * Shows a confirmation dialog.
 *
 * `app` The Tauri application handle.
 *
 * Returns:
 * True if the user confirms the dialog, false otherwise.
 */
#[tauri::command]
pub async fn confirm_dialog(app: AppHandle) -> bool {
    app.dialog().message("Are you sure?").title("Confirm").kind(tauri_plugin_dialog::MessageDialogKind::Warning).buttons(MessageDialogButtons::YesNo).blocking_show()
}

/**
 * Shows a open file dialog.
 *
 * `app` The Tauri application handle.
 *
 * Returns:
 * True if the user confirms the dialog, false otherwise.
 */
#[tauri::command]
pub async fn open_dialog(app: AppHandle, name: Option<String>, extension: Option<String>) -> Option<String> {
    let dialog = app.dialog().file();
    let dialog = if let Some(extension) = extension { dialog.add_filter(name.unwrap_or_default(), &[&extension]) } else { dialog };
    dialog.blocking_pick_file().map(|file_path| get_file_path(file_path).unwrap_or_default())
}

/**
 * Starts a setup.
 *
 * `app_data` The application data.
 * `setupid` The setup id.
 *
 * Returns:
 * The setup.
 *
 * # Errors
 * If the configuration data could not be locked.
 * If the setup could not be found.
 * If the setup could not be started.
 */
#[tauri::command]
pub async fn start_process(app_data: State<'_, AppData>, collectionid: &str, setupid: &str, params: HashMap<String, String>) -> Result<(), String> {
    let mut app_config = get_configuration_data(&app_data)?;
    let collection = app_config.get_collection(collectionid).ok_or("Collection not found".to_string())?;
    let collection_name = collection.name.clone();
    let setup_name = {
        let setup = collection.get_setup(setupid).ok_or("Setup not found".to_string())?;
        setup.name.clone()
    };
    let mut process_data = app_data.process_data.lock().map_err(|err| err.to_string())?;
    let file_path = get_current_file_path(&app_data)?;
    let path = Path::new(&file_path).parent().or_else(|| Some(Path::new("."))).ok_or("Invalid file path")?;
    let app_path = Settings::load().apinae_path.unwrap_or("apinae".to_owned());
    let mut process = std::process::Command::new(app_path);
    let mut process = process.arg("--file").arg(&file_path).arg("--id").arg(collectionid.to_owned() + "-" + &setupid.to_owned());
    for (key, value) in params.clone() {
        process = process.arg("--param").arg(format!("{key}={value}"));
    }
    let mut process = process.stdout(std::process::Stdio::piped()).current_dir(path.as_os_str()).spawn().map_err(|err| err.to_string())?;
    std::thread::sleep(std::time::Duration::from_millis(1000));
    if let Some(status) = process.try_wait().map_err(|err| err.to_string())? {
        return Err(format!("Process exited immediately with status: {status}"));
    }
    let process_id = process.id();
    process_data.insert(process_id, ProcessData::new(process_id, process, collection_name, setup_name, params));
    Result::Ok(())
}

/**
 * Stops a setup.
 *
 * `app_data` The application data.
 * `setupid` The setup id.
 *
 * Returns:
 * The setup.
 *
 * # Errors
 * If the configuration data could not be locked.
 * If the setup could not be found.
 * If the setup could not be stopped.
 */
#[allow(clippy::all)]
#[tauri::command]
pub async fn stop_process(app_data: State<'_, AppData>, processid: u32) -> Result<(), String> {
    let mut processes_data = app_data.process_data.lock().map_err(|err| err.to_string())?;
    match processes_data.remove(&processid) {
        Some(process_data) => {
            let mut process = process_data.process;
            let _ = process.kill().map_err(|err| err.to_string())?;
            Result::Ok(()) 
        }
        None => Result::Ok(()),
    }
}

/**
 * Adds a TCP listener to a setup.
 *
 * `app_data` The application data.
 * `collectionid` The collection id.
 * `setupid` The setup id.
 *
 * # Errors
 * If the configuration data could not be locked.
 * If the setup could not be found.
 */
#[tauri::command]
pub async fn add_tcp_listener(app_data: State<'_, AppData>, collectionid: &str, setupid: &str) -> Result<(), String> {
    let mut app_config = get_configuration_data(&app_data)?;
    let collection = app_config.get_collection(collectionid).ok_or("Collection not found".to_string())?;
    let setup = collection.get_setup(setupid).ok_or("Setup not found".to_string())?;
    let new_listener = apinae_lib::config::TcpListenerData::new(None, None, None, 0, true, apinae_lib::config::CloseConnectionWhen::Never)
        .map_err(|err| err.to_string())?;
    setup.add_listener(new_listener);
    update_data(&app_data, Some(app_config))?;
    Result::Ok(())
}

/**
 * Deletes a TCP listener from a setup.
 * 
 * `app_data` The application data.
 * `collectionid` The collection id.
 * `setupid` The setup id.
 * `listener_id` The id of the listener to delete.
 */
#[tauri::command]
pub async fn delete_tcp_listener(app: AppHandle, app_data: State<'_, AppData>, collectionid: &str, setupid: &str, listener_id: &str) -> Result<(), String> {
    if !confirm_dialog(app).await {
        return Result::Ok(());
    }
    let mut app_config = get_configuration_data(&app_data)?;
    let collection = app_config.get_collection(collectionid).ok_or("Collection not found".to_string())?;
    let setup = collection.get_setup(setupid).ok_or("Setup not found".to_string())?;
    setup.delete_listener(listener_id).map_err(|err| err.to_string())?;
    update_data(&app_data, Some(app_config))?;
    Result::Ok(())
}

/**
 * Adds an endpoint to a server.
 * 
 * `app_data` The application data.
 * `collectionid` The collection id.
 * `setupid` The setup id.
 * `server_id` The id of the server to add the endpoint to.
 * 
 * # Errors
 * If the configuration data could not be locked.
 * If the setup could not be found.
 * If the server could not be found.
 * If the endpoint could not be added.
 */
#[tauri::command]
pub fn add_endpoint(app_data: State<'_, AppData>, collectionid: &str, setupid: &str, server_id: &str) -> Result<(), String> {
    let mut app_config = get_configuration_data(&app_data)?;
    let collection = app_config.get_collection(collectionid).ok_or("Collection not found".to_string())?;
    let setup = collection.get_setup(setupid).ok_or("Setup not found".to_string())?;
    let server = setup.get_server(server_id).ok_or("Server not found".to_string())?;
    let new_endpoint = EndpointConfiguration::new(None, None, None, None).map_err(|err| err.to_string())?;
    server.add_endpoint(new_endpoint);
    update_data(&app_data, Some(app_config))?;
    Result::Ok(())
}

/**
 * Deletes an endpoint from a server.
 * 
 * `app_data` The application data.
 * `collectionid` The collection id.
 * `setupid` The setup id.
 * `server_id` The id of the server to delete the endpoint from.
 * `endpoint_id` The id of the endpoint to delete.
 * 
 * # Errors
 * If the configuration data could not be locked.
 * If the setup could not be found.
 * If the server could not be found.
 * If the endpoint could not be deleted.
 * 
 */
#[tauri::command]
pub async fn delete_endpoint(app: AppHandle, app_data: State<'_, AppData>, collectionid: &str, setupid: &str, server_id: &str, endpoint_id: &str) -> Result<(), String> {
    if !confirm_dialog(app).await {
        return Result::Ok(());
    }
    let mut app_config = get_configuration_data(&app_data)?;
    let collection = app_config.get_collection(collectionid).ok_or("Collection not found".to_string())?;
    let setup = collection.get_setup(setupid).ok_or("Setup not found".to_string())?;
    let server = setup.get_server(server_id).ok_or("Server not found".to_string())?;
    server.delete_endpoint(endpoint_id).map_err(|err| err.to_string())?;
    update_data(&app_data, Some(app_config))?;
    Result::Ok(())
}

/**
 * Adds an HTTP server to a setup.
 * 
 * `app_data` The application data.
 * `collectionid` The collection id.
 * `setupid` The setup id.
 * 
 * # Errors
 * If the configuration data could not be locked.
 * If the setup could not be found.
 */
#[tauri::command]
pub fn add_http_server(app_data: State<'_, AppData>, collectionid: &str, setupid: &str) -> Result<(), String> {
    let mut app_config = get_configuration_data(&app_data)?;
    let collection = app_config.get_collection(collectionid).ok_or("Collection not found".to_string())?;
    let setup = collection.get_setup(setupid).ok_or("Setup not found".to_string())?;
    let http_server = ServerConfiguration::new("Untitled".to_string(), None, None)
        .map_err(|err| err.to_string())?;
    setup.add_server(http_server);
    update_data(&app_data, Some(app_config))?;
    Result::Ok(())
}

/**
 * Deletes an HTTP server from a setup.
 * 
 * `app_data` The application data.
 * `collectionid` The collection id.
 * `setupid` The setup id.
 * `server_id` The id of the server to delete.
 * 
 * # Errors
 * If the configuration data could not be locked.
 * If the setup could not be found.
 * If the server could not be deleted.
 */
#[tauri::command]
pub async fn delete_http_server(app: AppHandle, app_data: State<'_, AppData>, collectionid: &str, setupid: &str, server_id: &str) -> Result<(), String> {
    if !confirm_dialog(app).await {
        return Result::Ok(());
    }
    let mut app_config = get_configuration_data(&app_data)?;
    let collection = app_config.get_collection(collectionid).ok_or("Collection not found".to_string())?;
    let setup = collection.get_setup(setupid).ok_or("Setup not found".to_string())?;
    setup.delete_server(server_id).map_err(|err| err.to_string())?;
    update_data(&app_data, Some(app_config))?;
    Result::Ok(())
}

/**
 * Gets the collections tree.
 *
 * `app_data` The application data.
 *
 * Returns:
 * The collections tree.
 *
 * # Errors
 * If the configuration data could not be locked.
 */
#[tauri::command]
pub fn get_setup_tree(app_data: State<'_, AppData>) -> Result<Vec<CollectionRow>, String> {
    let app_config = get_configuration_data(&app_data)?;
    let collections = app_config.get_collections().iter().map(|collection| CollectionRow::from(collection)).collect::<Vec<_>>();
    Result::Ok(collections)
}

/**
 * Adds a collection to the current configuration.
 *
 * `app_data` The application data.
 * `name` The collection name.
 * `description` The collection description.
 *
 * Returns:
 * The created collection row.
 *
 * # Errors
 * If data cannot be locked or collection cannot be created.
 */
#[tauri::command]
pub fn add_collection(app_data: State<'_, AppData>, name: Option<String>, description: Option<String>) -> Result<CollectionRow, String> {
    let mut app_config = get_configuration_data(&app_data)?;
    let collection = CollectionConfiguration::new(name.unwrap_or_else(|| DEFAULT_NAME.to_string()), description.unwrap_or_default())
        .map_err(|err| err.to_string())?;
    let collection_row = CollectionRow::from(&collection);
    app_config.add_collection(collection);
    update_data(&app_data, Some(app_config))?;
    Result::Ok(collection_row)
}

/**
 * Renames a collection in the current configuration.
 * 
 * `app_data` The application data.
 * `collectionid` The collection id.
 * `name` The new collection name.
 * 
 * # Errors
 * If data cannot be locked or collection cannot be found.
 */
#[tauri::command]
pub fn rename_collection(app_data: State<'_, AppData>, collectionid: &str, name: String) -> Result<(), String> {
    let mut app_config = get_configuration_data(&app_data)?;
    let collection = app_config.get_collection(collectionid).ok_or("Collection not found".to_string())?;
    collection.name = name;
    update_data(&app_data, Some(app_config))?;
    Result::Ok(())
}

/**
 * Deletes a collection from the current configuration.
 *
 * `app` The application handle.
 * `app_data` The application data.
 * `collectionid` The collection id.
 *
 * # Errors
 * If data cannot be locked or collection cannot be deleted.
 */
#[tauri::command]
pub async fn delete_collection(app: AppHandle, app_data: State<'_, AppData>, collectionid: &str) -> Result<(), String> {
    let confirmed = confirm_dialog(app).await;
    if !confirmed {
        return Result::Ok(());
    }
    let mut app_config = get_configuration_data(&app_data)?;
    app_config.delete_collection(collectionid).map_err(|err| err.to_string())?;
    update_data(&app_data, Some(app_config))?;
    Result::Ok(())
}

/**
 * Adds a setup to a collection.
 *
 * `app_data` The application data.
 * `collectionid` The collection id.
 * `name` The setup name.
 * `description` The setup description.
 *
 * Returns:
 * The created setup row.
 *
 * # Errors
 * If data cannot be locked or setup cannot be created.
 */
#[tauri::command]
pub fn add_setup(app_data: State<'_, AppData>, collectionid: &str, name: Option<String>, description: Option<String>) -> Result<SetupRow, String> {
    let mut app_config = get_configuration_data(&app_data)?;
    let collection = app_config.get_collection(collectionid).ok_or("Collection not found".to_string())?;
    let setup = SetupConfiguration::new(name.unwrap_or_else(|| DEFAULT_NAME.to_string()), description.unwrap_or_default())
        .map_err(|err| err.to_string())?;
    let setup_row = SetupRow::from(setup.clone());
    collection.add_setup(setup);
    update_data(&app_data, Some(app_config))?;
    Result::Ok(setup_row)
}

/**
 * Renames a setup in a collection.
 *
 * `app_data` The application data.
 * `collectionid` The collection id.
 * `setupid` The setup id.
 * `name` The new setup name.
 *
 * # Errors
 * If data cannot be locked or setup cannot be found.
 */
#[tauri::command]
pub fn rename_setup(app_data: State<'_, AppData>, collectionid: &str, setupid: &str, name: String) -> Result<(), String> {
    let mut app_config = get_configuration_data(&app_data)?;
    let collection = app_config.get_collection(collectionid).ok_or("Collection not found".to_string())?;
    let setup = collection.get_setup(setupid).ok_or("Setup not found".to_string())?;
    setup.name = name;
    update_data(&app_data, Some(app_config))?;
    Result::Ok(())
}

/**
 * Deletes a setup from a collection.
 *
 * `app_data` The application data.
 * `collectionid` The collection id.
 * `setupid` The setup id.
 *
 * # Errors
 * If data cannot be locked or setup cannot be deleted.
 */
#[tauri::command]
pub async fn delete_setup(app: AppHandle, app_data: State<'_, AppData>, collectionid: &str, setupid: &str) -> Result<(), String> {
    let confirmed = confirm_dialog(app).await;
    if !confirmed {
        return Result::Ok(());
    }
    let mut app_config = get_configuration_data(&app_data)?;
    let collection = app_config.get_collection(collectionid).ok_or("Collection not found".to_string())?;
    collection.delete_setup(setupid).map_err(|err| err.to_string())?;
    update_data(&app_data, Some(app_config))?;
    Result::Ok(())
}

/**
 * Gets the processes.
 *
 * `app_data` The application data.
 *
 * Returns:
 * The processes.
 *
 * # Errors
 * If the processes data could not be locked.
 */
#[tauri::command]
pub fn get_processes(app_data: State<'_, AppData>) -> Result<Vec<ProcessRow>, String> {
    let mut processes_data = app_data.process_data.lock().map_err(|err| err.to_string())?;
    processes_data.retain(|_, process_data| {
        matches!(process_data.process.try_wait(), Result::Ok(None))
    });
    let processes = processes_data.values().map(ProcessRow::from).collect::<Vec<_>>();
    Result::Ok(processes)
}

/**
 * Loads the settings.
 *
 * Returns:
 * The settings.
 */
#[tauri::command]
pub fn load_settings() -> Settings {
    Settings::load()
}

/**
 * Saves the settings.
 *
 * `settings` The settings to save.
 *
 * # Errors
 * If the settings cannot be saved.
 */
#[tauri::command]
pub fn save_settings(settings: Settings) -> Result<(), String> {
    settings.save().map_err(|err| err.to_string())
}

/**
 * Gets the configuration data.
 *
 * `app_data` The application data.
 *
 * Returns:
 * The configuration data.
 *
 * # Errors
 * If the configuration data could not be locked.
 */
fn get_configuration_data(app_data: &State<'_, AppData>) -> Result<AppConfiguration, String> {
    let lock = app_data.data.lock().map_err(|err| err.to_string())?;
    lock.clone().ok_or("No data".to_string())
}

/**
 * Gets the current file path.
 *
 * `app_data` The application data.
 *
 * Returns:
 * The current file path.
 *
 * # Errors
 * If the file path could not be locked.
 * If there is no file path.
 */
fn get_current_file_path(app_data: &State<'_, AppData>) -> Result<String, String> {
    let lock = app_data.file_path.lock().map_err(|err| err.to_string())?;
    lock.clone().ok_or("No file path".to_string())
}

/**
 * Gets the file path.
 *
 * `file_path` The file path.
 *
 * Returns:
 * The file path.
 *
 * # Errors
 * If the file path is invalid.
 */
fn get_file_path(file_path: FilePath) -> Result<String, String> {
    if let FilePath::Path(path) = file_path {
        Result::Ok(path.to_string_lossy().to_string())
    } else {
        Err("Invalid file path".to_string())
    }
}

/**
 * Update state data
 *
 * 'app_data' The application data.
 * 'new_data' The new data.
 *
 * # Errors
 * If the data could not be locked.
 */
fn update_data(app_data: &State<'_, AppData>, new_data: Option<AppConfiguration>) -> Result<(), String> {
    let mut lock = app_data.data.try_lock().map_err(|err| err.to_string())?;
    *lock = new_data;
    Result::Ok(())
}

/**
 * Updates the file path.
 *
 * `app_data` The application data.
 * `new_file_path` The new file path.
 *
 * # Errors
 * If the file path could not be locked.
 */
fn update_file_path(app_data: &State<'_, AppData>, new_file_path: Option<String>) -> Result<(), String> {
    let mut lock = app_data.file_path.try_lock().map_err(|err| err.to_string())?;
    *lock = new_file_path;
    Result::Ok(())
}
