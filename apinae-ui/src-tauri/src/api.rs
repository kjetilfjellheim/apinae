use std::collections::HashSet;
use std::{collections::HashMap, path::Path};

use crate::model::{CollectionRow, EndpointRow, HttpServerRow, ProcessRow, SetupRow, TcpListenerRow};
use crate::state::ProcessData;
use crate::AppData;
use apinae_lib::config::{EndpointConfiguration, EndpointType, MockResponseConfiguration, ServerConfiguration};
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
 * Adds a predefined set to a setup.
 *
 * `app_data` The application data.
 * `collectionid` The collection id.
 * `setupid` The setup id.
 * `name` The name of the predefined set to add.
 *
 * # Errors
 * If data cannot be locked or setup cannot be found.
 */
#[tauri::command]
pub fn add_predefined_set(app_data: State<'_, AppData>, collectionid: &str, setupid: &str, name: &str) -> Result<(), String> {
    let mut app_config = get_configuration_data(&app_data)?;
    let collection = app_config.get_collection(collectionid).ok_or("Collection not found".to_string())?;
    let setup = collection.get_setup(setupid).ok_or("Setup not found".to_string())?;
    setup.add_predefined_set(name.to_string(), Vec::new()).map_err(|err| err.to_string())?;
    update_data(&app_data, Some(app_config))?;
    Result::Ok(())
}

/**
 * Deletes a predefined set from a setup.
 *
 * `app_data` The application data.
 * `collectionid` The collection id.
 * `setupid` The setup id.
 * `name` The name of the predefined set to delete.
 *
 * # Errors
 * If data cannot be locked or setup cannot be found.
 */
#[tauri::command]
pub fn delete_predefined_set(app_data: State<'_, AppData>, collectionid: &str, setupid: &str, name: &str) -> Result<(), String> {
    let mut app_config = get_configuration_data(&app_data)?;
    let collection = app_config.get_collection(collectionid).ok_or("Collection not found".to_string())?;
    let setup = collection.get_setup(setupid).ok_or("Setup not found".to_string())?;
    setup.remove_predefined_set(name);
    update_data(&app_data, Some(app_config))?;
    Result::Ok(())
}

/**
 * Updates a predefined set in a setup.
 *
 * `app_data` The application data.
 * `collectionid` The collection id.
 * `setupid` The setup id.
 * `name` The name of the predefined set to update.
 * `values` The new values for the predefined set.
 *
 * # Errors
 * If data cannot be locked or setup cannot be found.
 */
#[tauri::command]
pub fn update_predefined_set(app_data: State<'_, AppData>, collectionid: &str, setupid: &str, name: &str, values: HashMap<String, String>) -> Result<(), String> {
    let mut app_config = get_configuration_data(&app_data)?;
    let collection = app_config.get_collection(collectionid).ok_or("Collection not found".to_string())?;
    let setup = collection.get_setup(setupid).ok_or("Setup not found".to_string())?;
    setup.update_predefined_set(name, values).map_err(|err| err.to_string())?;
    update_data(&app_data, Some(app_config))?;
    Result::Ok(())
}

/**
 * Gets a setup from a collection.
 *
 * `app_data` The application data.
 * `collectionid` The collection id.
 * `setupid` The setup id.
 *
 * Returns:
 * The setup row.
 *
 * # Errors
 * If data cannot be locked or setup cannot be found.
 */
#[tauri::command]
pub fn get_setup(app_data: State<'_, AppData>, collectionid: &str, setupid: &str) -> Result<SetupRow, String> {
    let mut app_config = get_configuration_data(&app_data)?;
    let collection = app_config.get_collection(collectionid).ok_or("Collection not found".to_string())?;
    let setup = collection.get_setup(setupid).ok_or("Setup not found".to_string())?;
    if let Some(predefined_sets) = setup.predefined_sets.as_mut() {
        match setup.params.clone() {
            Some(params) => {
                for predefined_set in predefined_sets.values_mut() {
                    predefined_set.retain(|key, _| params.contains(key));
                    for param in &params {
                        if !predefined_set.contains_key(param) {
                            predefined_set.insert(param.clone(), String::new());
                        }
                    }
                }
            }
            None => {
                for predefined_set in predefined_sets.values_mut() {
                    predefined_set.clear();
                }
            }
        }
    }
    Result::Ok(SetupRow::from(setup.clone()))
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
    let setup = SetupConfiguration::new(name.unwrap_or_else(|| DEFAULT_NAME.to_string()), description.unwrap_or_default()).map_err(|err| err.to_string())?;
    let setup_row = SetupRow::from(setup.clone());
    collection.add_setup(setup);
    update_data(&app_data, Some(app_config))?;
    Result::Ok(setup_row)
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
    setup.update(name.clone(), description.clone());
    if parameters.is_empty() {
        setup.params = None;
    } else {
        setup.params = Some(HashSet::from_iter(parameters));
    }
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
    let new_listener = apinae_lib::config::TcpListenerData::new(None, None, None, 0, true, apinae_lib::config::CloseConnectionWhen::Never).map_err(|err| err.to_string())?;
    setup.add_listener(new_listener);
    update_data(&app_data, Some(app_config))?;
    Result::Ok(())
}

/**
 * Updates a TCP listener in a setup.
 *
 * `app_data` The application data.
 * `collectionid` The collection id.
 * `setupid` The setup id.
 * `listener_id` The id of the listener to update.
 *
 * # Errors
 * If the configuration data could not be locked.
 * If the setup could not be found.
 * If the listener could not be found.
 */
#[tauri::command]
pub fn update_tcp_listener(app_data: State<'_, AppData>, collectionid: &str, setupid: &str, listenerid: &str, tcplistener: TcpListenerRow) -> Result<(), String> {
    let mut app_config = get_configuration_data(&app_data)?;
    let collection = app_config.get_collection(collectionid).ok_or("Collection not found".to_string())?;
    let setup = collection.get_setup(setupid).ok_or("Setup not found".to_string())?;
    let listener = setup.get_listener(listenerid).ok_or("Listener not found".to_string())?;
    listener.update(TcpListenerRow::into(tcplistener));
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
pub async fn delete_tcp_listener(app: AppHandle, app_data: State<'_, AppData>, collectionid: &str, setupid: &str, listenerid: &str) -> Result<(), String> {
    if !confirm_dialog(app).await {
        return Result::Ok(());
    }
    let mut app_config = get_configuration_data(&app_data)?;
    let collection = app_config.get_collection(collectionid).ok_or("Collection not found".to_string())?;
    let setup = collection.get_setup(setupid).ok_or("Setup not found".to_string())?;
    setup.delete_listener(listenerid).map_err(|err| err.to_string())?;
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
    let http_server = ServerConfiguration::new("Untitled".to_string(), None, None).map_err(|err| err.to_string())?;
    setup.add_server(http_server);
    update_data(&app_data, Some(app_config))?;
    Result::Ok(())
}

/**
 * Updates an HTTP server in a setup.
 *
 * `app_data` The application data.
 * `collectionid` The collection id.
 * `setupid` The setup id.
 * `server_id` The id of the server to update.
 *
 * # Errors
 * If the configuration data could not be locked.
 * If the setup could not be found.
 * If the server could not be found.
 * If the server could not be updated.
 */
#[tauri::command]
pub fn update_http_server(app_data: State<'_, AppData>, collectionid: &str, setupid: &str, serverid: &str, http_server: HttpServerRow) -> Result<(), String> {
    let mut app_config = get_configuration_data(&app_data)?;
    let collection = app_config.get_collection(collectionid).ok_or("Collection not found".to_string())?;
    let setup = collection.get_setup(setupid).ok_or("Setup not found".to_string())?;
    let server = setup.get_server(serverid).ok_or("Server not found".to_string())?;
    server.update(http_server.name, http_server.http_port, http_server.https_config.map(|config| config.into()));
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
pub async fn delete_http_server(app: AppHandle, app_data: State<'_, AppData>, collectionid: &str, setupid: &str, serverid: &str) -> Result<(), String> {
    if !confirm_dialog(app).await {
        return Result::Ok(());
    }
    let mut app_config = get_configuration_data(&app_data)?;
    let collection = app_config.get_collection(collectionid).ok_or("Collection not found".to_string())?;
    let setup = collection.get_setup(setupid).ok_or("Setup not found".to_string())?;
    setup.delete_server(serverid).map_err(|err| err.to_string())?;
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
pub fn add_endpoint(app_data: State<'_, AppData>, collectionid: &str, setupid: &str, serverid: &str) -> Result<(), String> {
    let mut app_config = get_configuration_data(&app_data)?;
    let collection = app_config.get_collection(collectionid).ok_or("Collection not found".to_string())?;
    let setup = collection.get_setup(setupid).ok_or("Setup not found".to_string())?;
    let server = setup.get_server(serverid).ok_or("Server not found".to_string())?;
    let new_endpoint = EndpointConfiguration::new(None, None, None, Some(EndpointType::Mock { configuration: MockResponseConfiguration::new(None, "200".to_string(), HashMap::new(), 0) }))
        .map_err(|err| err.to_string())?;
    server.add_endpoint(new_endpoint);
    update_data(&app_data, Some(app_config))?;
    Result::Ok(())
}

/**
 * Updates an endpoint in a server.
 *
 * `app_data` The application data.
 * `collectionid` The collection id.
 * `setupid` The setup id.
 * `server_id` The id of the server to update the endpoint in.
 * `endpoint_id` The id of the endpoint to update.
 *
 * # Errors
 * If the configuration data could not be locked.
 * If the setup could not be found.
 * If the server could not be found.
 * If the endpoint could not be found.
 * If the endpoint could not be updated.
 */
#[tauri::command]
pub fn update_endpoint(app_data: State<'_, AppData>, collectionid: &str, setupid: &str, serverid: &str, endpointid: &str, endpoint: EndpointRow) -> Result<(), String> {
    let mut app_config = get_configuration_data(&app_data)?;
    let collection = app_config.get_collection(collectionid).ok_or("Collection not found".to_string())?;
    let setup = collection.get_setup(setupid).ok_or("Setup not found".to_string())?;
    let server = setup.get_server(serverid).ok_or("Server not found".to_string())?;
    let current_endpoint = server.get_endpoint(endpointid).ok_or("Endpoint not found".to_string())?;
    current_endpoint.update((&endpoint).into());
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
pub async fn delete_endpoint(app: AppHandle, app_data: State<'_, AppData>, collectionid: &str, setupid: &str, serverid: &str, endpointid: &str) -> Result<(), String> {
    if !confirm_dialog(app).await {
        return Result::Ok(());
    }
    let mut app_config = get_configuration_data(&app_data)?;
    let collection = app_config.get_collection(collectionid).ok_or("Collection not found".to_string())?;
    let setup = collection.get_setup(setupid).ok_or("Setup not found".to_string())?;
    let server = setup.get_server(serverid).ok_or("Server not found".to_string())?;
    server.delete_endpoint(endpointid).map_err(|err| err.to_string())?;
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
    Result::Ok(())
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
pub async fn start_process(app_data: State<'_, AppData>, collectionid: &str, setupid: &str, params: Option<HashMap<String, String>>, predefined_set_name: Option<String>) -> Result<(), String> {
    let mut app_config = get_configuration_data(&app_data)?;
    let collection = app_config.get_collection(collectionid).ok_or("Collection not found".to_string())?;
    let collection_name = collection.name.clone();
    let setup = collection.get_setup(setupid).ok_or("Setup not found".to_string())?;
    let mut process_data = app_data.process_data.lock().map_err(|err| err.to_string())?;
    let file_path = get_current_file_path(&app_data)?;
    let path = Path::new(&file_path).parent().or_else(|| Some(Path::new("."))).ok_or("Invalid file path")?;
    let app_path = Settings::load().apinae_path.unwrap_or("apinae".to_owned());
    let mut process = std::process::Command::new(app_path);
    let mut process = process.arg("--file").arg(&file_path).arg("--id").arg(collectionid.to_owned() + "-" + setupid);
    if let Some(params) = params.clone() {
        for (key, value) in params {
            process = process.arg("--param").arg(format!("{key}={value}"));
        }
    }
    if let Some(predefined_set_name) = &predefined_set_name {
        process = process.arg("--predefined-set").arg(predefined_set_name);
    }
    let mut process = process.stdout(std::process::Stdio::piped()).current_dir(path.as_os_str()).spawn().map_err(|err| err.to_string())?;
    std::thread::sleep(std::time::Duration::from_millis(1000));
    if let Some(status) = process.try_wait().map_err(|err| err.to_string())? {
        return Err(format!("Process exited immediately with status: {status}"));
    }
    let process_id = process.id();
    // Store the parameters used to start the process, either from the provided params or from the predefined set
    let params: HashMap<String, String> = if let Some(params) = params {
        params
    } else if let Some(predefined_set_name) = &predefined_set_name {
        let predefined_sets = setup.predefined_sets.as_ref().ok_or("Setup has no predefined sets".to_string())?;
        let predefined_set = predefined_sets.get(predefined_set_name).ok_or("Predefined set not found".to_string())?;
        predefined_set.clone()
    } else {
        HashMap::new()
    };
    process_data.insert(process_id, ProcessData::new(process_id, process, collection_name, setup.name.clone(), params));
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
    let collections = app_config.get_collections().iter().map(CollectionRow::from).collect::<Vec<_>>();
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
    let collection = CollectionConfiguration::new(name.unwrap_or_else(|| DEFAULT_NAME.to_string()), description.unwrap_or_default()).map_err(|err| err.to_string())?;
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
    processes_data.retain(|_, process_data| matches!(process_data.process.try_wait(), Result::Ok(None)));
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::model::MockRow;
    use apinae_lib::config::{MockResponseConfiguration, TcpListenerData};
    use tauri::Manager;

    fn mock_app() -> tauri::App<tauri::test::MockRuntime> {
        tauri::test::mock_builder().manage(AppData::new()).build(tauri::test::mock_context(tauri::test::noop_assets())).unwrap()
    }

    fn seed_state(app: &tauri::App<tauri::test::MockRuntime>) -> (String, String, String, String, String) {
        let state = app.state::<AppData>();

        let mut listener = TcpListenerData::new(None, Some("response".to_string()), Some(10), 9000, true, apinae_lib::config::CloseConnectionWhen::Never).unwrap();
        listener.id = "listener-1".to_string();

        let endpoint = EndpointConfiguration::new(
            Some("^/test$".to_string()),
            Some("GET".to_string()),
            None,
            Some(EndpointType::Mock { configuration: MockResponseConfiguration::new(Some("ok".to_string()), "200".to_string(), HashMap::new(), 0) }),
        )
        .unwrap();
        let endpoint_id = endpoint.id.clone();

        let mut server = ServerConfiguration::new("Server".to_string(), Some(8080), None).unwrap();
        server.id = "server-1".to_string();
        server.add_endpoint(endpoint);

        let mut setup = SetupConfiguration::new("Setup".to_string(), "Setup description".to_string()).unwrap();
        setup.id = "setup-1".to_string();
        setup.params = Some(HashSet::from(["host".to_string(), "port".to_string()]));
        setup.predefined_sets = Some(HashMap::from([("Production".to_string(), HashMap::from([("host".to_string(), "example.com".to_string()), ("port".to_string(), "8080".to_string())]))]));
        setup.add_server(server);
        setup.add_listener(listener);

        let mut collection = CollectionConfiguration::new("Collection".to_string(), "Collection description".to_string()).unwrap();
        collection.id = "collection-1".to_string();
        collection.add_setup(setup);

        let mut app_config = AppConfiguration::new("App".to_string(), "Description".to_string());
        app_config.add_collection(collection);
        update_data(&state, Some(app_config)).unwrap();

        ("collection-1".to_string(), "setup-1".to_string(), "server-1".to_string(), "listener-1".to_string(), endpoint_id)
    }

    #[test]
    fn app_state_helpers_update_and_read_configuration() {
        let app = mock_app();
        let state = app.state::<AppData>();

        assert!(get_configuration_data(&state).is_ok());
        assert!(get_current_file_path(&state).is_err());

        update_file_path(&state, Some("test.json".to_string())).unwrap();
        assert_eq!(get_current_file_path(&state).unwrap(), "test.json");

        update_data(&state, None).unwrap();
        assert_eq!(get_configuration_data(&state).unwrap_err(), "No data");
    }

    #[test]
    fn collection_commands_add_rename_tree_and_delete() {
        let app = mock_app();
        let state = app.state::<AppData>();

        let collection = add_collection(state.clone(), Some("Collection".to_string()), Some("Description".to_string())).unwrap();
        assert_eq!(collection.name, "Collection");
        assert_eq!(collection.description, "Description");

        rename_collection(state.clone(), &collection.id, "Renamed".to_string()).unwrap();
        let tree = get_setup_tree(state.clone()).unwrap();
        assert_eq!(tree.len(), 1);
        assert_eq!(tree[0].name, "Renamed");
    }

    #[test]
    fn setup_commands_add_update_get_and_missing_errors() {
        let app = mock_app();
        let state = app.state::<AppData>();

        let collection = add_collection(state.clone(), Some("Collection".to_string()), Some("Description".to_string())).unwrap();
        let setup = add_setup(state.clone(), &collection.id, Some("Setup".to_string()), Some("Description".to_string())).unwrap();

        update_setup(state.clone(), &collection.id, &setup.id, "Renamed Setup".to_string(), "New description".to_string(), vec!["host".to_string(), "port".to_string()]).unwrap();

        let retrieved = get_setup(state.clone(), &collection.id, &setup.id).unwrap();
        assert_eq!(retrieved.name, "Renamed Setup");
        assert_eq!(retrieved.description, "New description");
        assert_eq!(retrieved.params.as_ref().unwrap(), &["host".to_string(), "port".to_string()]);

        assert_eq!(get_setup(state.clone(), "missing", &setup.id).unwrap_err(), "Collection not found");
        assert_eq!(get_setup(state.clone(), &collection.id, "missing").unwrap_err(), "Setup not found");
    }

    #[test]
    fn predefined_set_commands_normalize_values() {
        let app = mock_app();
        let (collection_id, setup_id, _, _, _) = seed_state(&app);
        let state = app.state::<AppData>();

        update_predefined_set(state.clone(), &collection_id, &setup_id, "Production", HashMap::from([("host".to_string(), "api.example.com".to_string())])).unwrap();

        let setup = get_setup(state.clone(), &collection_id, &setup_id).unwrap();
        let production = &setup.predefined_sets.unwrap()["Production"];
        assert_eq!(production.get("host").unwrap(), "api.example.com");
        assert_eq!(production.get("port").unwrap(), "");

        delete_predefined_set(state.clone(), &collection_id, &setup_id, "Production").unwrap();
        let setup = get_setup(state.clone(), &collection_id, &setup_id).unwrap();
        assert!(setup.predefined_sets.is_none());

        add_predefined_set(state.clone(), &collection_id, &setup_id, "Development").unwrap();
        let setup = get_setup(state.clone(), &collection_id, &setup_id).unwrap();
        assert!(setup.predefined_sets.as_ref().unwrap().contains_key("Development"));
    }

    #[tokio::test]
    async fn tcp_listener_commands_add_update_and_get() {
        let app = mock_app();
        let (collection_id, setup_id, _, _, _) = seed_state(&app);
        let state = app.state::<AppData>();

        add_tcp_listener(state.clone(), &collection_id, &setup_id).await.unwrap();
        let setup = get_setup(state.clone(), &collection_id, &setup_id).unwrap();
        let listener_id = setup.listeners.as_ref().unwrap().last().unwrap().id.clone();

        let listener =
            TcpListenerRow { id: listener_id.clone(), file: None, data: Some("updated".to_string()), delay_write_ms: Some(50), port: 9100, accept: true, close_connection: "Never".to_string() };
        update_tcp_listener(state.clone(), &collection_id, &setup_id, &listener_id, listener).unwrap();

        let setup = get_setup(state.clone(), &collection_id, &setup_id).unwrap();
        let listeners = setup.listeners.unwrap();
        let listener = listeners.iter().find(|item| item.id == listener_id).unwrap();
        assert_eq!(listener.port, 9100);
        assert_eq!(listener.data.as_deref(), Some("updated"));

        assert_eq!(
            update_tcp_listener(
                state.clone(),
                &collection_id,
                &setup_id,
                "missing",
                TcpListenerRow::from(&TcpListenerData::new(None, None, None, 1, true, apinae_lib::config::CloseConnectionWhen::Never).unwrap())
            )
            .unwrap_err(),
            "Listener not found"
        );
    }

    #[test]
    fn http_server_and_endpoint_commands_mutate_configuration() {
        let app = mock_app();
        let (collection_id, setup_id, _, _, endpoint_id) = seed_state(&app);
        let state = app.state::<AppData>();

        add_http_server(state.clone(), &collection_id, &setup_id).unwrap();
        let setup = get_setup(state.clone(), &collection_id, &setup_id).unwrap();
        let server_id = setup.servers.as_ref().unwrap().last().unwrap().id.clone();

        let server_row = HttpServerRow { id: server_id.clone(), name: "Renamed Server".to_string(), http_port: Some(8181), https_config: None, endpoints: Vec::new() };
        update_http_server(state.clone(), &collection_id, &setup_id, &server_id, server_row).unwrap();

        add_endpoint(state.clone(), &collection_id, &setup_id, &server_id).unwrap();

        let endpoint_row = EndpointRow {
            id: endpoint_id.clone(),
            path_expression: Some("^/updated$".to_string()),
            method: Some("POST".to_string()),
            body_expression: Some("body".to_string()),
            mock: Some(MockRow { response: Some("updated".to_string()), status: "201".to_string(), headers: "Content-Type: application/json\n".to_string(), delay: 5 }),
            route: None,
        };

        assert!(update_endpoint(state.clone(), &collection_id, &setup_id, "server-1", &endpoint_id, endpoint_row).is_ok());
        let setup = get_setup(state.clone(), &collection_id, &setup_id).unwrap();
        let server = setup.servers.as_ref().unwrap().iter().find(|item| item.id == "server-1").unwrap();
        assert_eq!(server.endpoints[0].path_expression.as_deref(), Some("^/updated$"));
    }

    #[tokio::test]
    async fn process_commands_list_stop_and_handle_missing_process() {
        let app = mock_app();
        let state = app.state::<AppData>();

        assert!(get_processes(state.clone()).unwrap().is_empty());
        assert!(stop_process(state.clone(), 999).await.is_ok());

        #[cfg(windows)]
        let child = std::process::Command::new("cmd").args(["/C", "exit 0"]).spawn().unwrap();
        #[cfg(not(windows))]
        let child = std::process::Command::new("sh").args(["-c", "sleep 30"]).spawn().unwrap();

        let process_id = child.id();
        let process_data = ProcessData::new(process_id, child, "Collection".to_string(), "Setup".to_string(), HashMap::new());
        state.process_data.lock().unwrap().insert(process_id, process_data);

        let processes = get_processes(state.clone()).unwrap();
        assert_eq!(processes.len(), 1);
        stop_process(state.clone(), process_id).await.unwrap();
        assert!(get_processes(state.clone()).unwrap().is_empty());
    }

    #[test]
    fn settings_and_file_path_helpers() {
        let _app = mock_app();

        assert_eq!(load_settings(), Settings::load());
        assert!(save_settings(Settings::default()).is_ok());

        assert_eq!(get_file_path(FilePath::Path(std::path::PathBuf::from("test.json"))).unwrap(), "test.json");
    }
}
