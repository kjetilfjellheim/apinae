use std::collections::HashSet;
use std::{collections::HashMap, path::Path};

use crate::model::PredefinedSet;
use crate::AppData;
use crate::{
    model::{EndpointRow, HttpServerRow, TcpListenerRow, SetupRow},
    state::ProcessData,
};
use apinae_lib::{
    config::{AppConfiguration, CloseConnectionWhen, EndpointConfiguration, EndpointType, HttpsConfiguration, MockResponseConfiguration, ServerConfiguration, TcpListenerData, SetupConfiguration},
    settings::Settings,
};
use tauri::{AppHandle, State};
use tauri_plugin_dialog::{DialogExt, FilePath, MessageDialogButtons};

/**
 * Default name for new setups, servers, listeners and endpoints.
 */
const DEFAULT_NAME: &str = "Untitled";
/**
 * Default status code for new mock responses.
 */
const DEFAULT_STATUS_CODE: &str = "200";
/**
 * Default delay for new mock responses.
 */
const DEFAULT_DELAY: u64 = 0;

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
pub async fn load(app: AppHandle, app_data: State<'_, AppData>) -> Result<String, String> {
    let file_path = app.dialog().file().blocking_pick_file();
    if let Some(file_path) = file_path {
        let file_path = get_file_path(file_path)?;
        update_data(&app_data, Some(AppConfiguration::load(&file_path).map_err(|err| err.to_string())?))?;
        update_file_path(&app_data, Some(file_path.clone()))?;
        return Ok(file_path);
    }
    Err("No file selected".to_string())
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
    if let Ok(current_file_path) = current_file_path {
        let data = get_configuration_data(&app_data)?;
        data.save(&current_file_path).map_err(|err| err.to_string())?;
        Ok(())
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
        Ok(())
    } else {
        Err("No file selected".to_string())
    }
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
        let new_data = AppConfiguration::new(DEFAULT_NAME.to_owned(), String::new(), Vec::new());
        update_data(&app_data, Some(new_data))?;
        update_file_path(&app_data, None)?;
    }
    get_configuration_data(&app_data)
}

/**
 * Gets the setups.
 *
 * `app_data` The application data.
 *
 * Returns:
 * The setups.
 *
 * # Errors
 * If the configuration data could not be locked.
 */
#[tauri::command]
pub async fn get_setups(app_data: State<'_, AppData>) -> Result<Vec<SetupRow>, String> {
    let data = get_configuration_data(&app_data)?;
    let process_data = app_data.process_data.lock().map_err(|err| err.to_string())?;
    let setups = data
        .setups
        .iter()
        .map(|setup| SetupRow::from(setup.clone()))
        .map(|mut setup_row| {
            setup_row.process_id = process_data.get(&setup_row.id).map(|process_data| process_data.process_id);
            setup_row
        })
        .collect();
    Ok(setups)
}

/**
 * Gets a setup.
 *
 * `app_data` The application data.
 * `setupid` The setup id.
 *
 * Returns:
 * The setup.
 *
 * # Errors
 * If the setup could not be found.
 */
#[tauri::command]
pub async fn get_setup(app_data: State<'_, AppData>, setupid: &str) -> Result<SetupRow, String> {
    let mut data = get_configuration_data(&app_data)?;
    let setup = data.get_setup(setupid).ok_or("Could not find setup")?;
    Ok(SetupRow::from(setup.clone()))
}

/**
 * Adds a setup.
 *
 * `app_data` The application data.
 *
 * # Errors
 * If the configuration data could not be locked.
 * If the setup could not be added.
 */
#[tauri::command]
pub async fn add_setup(app_data: State<'_, AppData>) -> Result<(), String> {
    let mut data = get_configuration_data(&app_data)?;
    data.setups.push(SetupConfiguration::new(DEFAULT_NAME.to_owned(), String::new(), Vec::new(), Vec::new(), None, None).map_err(|err| err.to_string())?);
    update_data(&app_data, Some(data))?;
    Ok(())
}

/**
 * Updates a setup.
 *
 * `app_data` The application data.
 * `setupid` The setup id.
 * `setup` The setup.
 *
 * # Errors
 * If the configuration data could not be locked.
 * If the setup could not be updated.
 */
#[tauri::command]
pub async fn update_setup(app_data: State<'_, AppData>, setupid: &str, setup: SetupRow) -> Result<(), String> {
    let mut data = get_configuration_data(&app_data)?;
    data.update_setup(setupid, setup.name.as_str(), setup.description.as_str(), setup.params.map(|f| f.iter().cloned().collect::<HashSet<_>>())).map_err(|err| err.to_string())?;
    update_data(&app_data, Some(data))?;
    Ok(())
}

/**
 * Deletes a setup.
 *
 * `app_data` The application data.
 * `setupid` The setup id.
 *
 * # Errors
 * If the configuration data could not be locked.
 * If the setup could not be deleted.
 */
#[tauri::command]
pub async fn delete_setup(app_data: State<'_, AppData>, setupid: &str) -> Result<(), String> {
    let mut data = get_configuration_data(&app_data)?;
    data.delete_setup(setupid).map_err(|err| err.to_string())?;
    update_data(&app_data, Some(data))?;
    Ok(())
}

/**
 * Gets the predefined sets.
 *
 * `app_data` The application data.
 * `setupid` The setup id.
 *
 * Returns:
 * The predefined sets.
 *
 * # Errors
 * If the configuration data could not be locked.
 * If the setup could not be found.
 */
#[tauri::command]
pub async fn get_predefined_sets(app_data: State<'_, AppData>, setupid: &str) -> Result<Vec<PredefinedSet>, String> {
    let mut data = get_configuration_data(&app_data)?;
    let setup = data.get_setup(setupid).ok_or("Setup not found")?;
    let mut sets: Vec<PredefinedSet> = Vec::new();    
    if let Some(predefined_params) = &setup.predefined_params {
        for predefined_set in predefined_params {
            let set = PredefinedSet::new(&predefined_set.name.clone(), predefined_set.values.clone());
            sets.push(set);
        }
    }
    Ok(sets)
}

/**
 * Adds a predefined set.
 * 
 * `app_data` The application data.
 * `setupid` The setup id.
 * 
 * # Errors
 * If the configuration data could not be locked.
 * If the setup could not be found.
 * If the predefined set could not be added.
 */
#[tauri::command]
pub async fn add_predefined_set(app_data: State<'_, AppData>, setupid: &str) -> Result<(), String> {
    let mut data = get_configuration_data(&app_data)?;
    let setup = data.get_setup(setupid).ok_or("Setup not found")?;
    setup.add_new_predefined_param_set().map_err(|err| err.to_string())?;
    update_data(&app_data, Some(data))?;
    Ok(()) 
}

/**
 * Deletes a predefined set.
 *
 * `app_data` The application data.
 * `setupid` The setup id.
 * `name` The name of the predefined set.
 *
 * # Errors
 * If the configuration data could not be locked.
 * If the setup could not be found.
 * If the predefined set could not be deleted.
 */
#[tauri::command]
pub fn delete_predefined_set(app_data: State<'_, AppData>, setupid: &str, name: &str) -> Result<(), String> {
    let mut data = get_configuration_data(&app_data)?;
    let setup = data.get_setup(setupid).ok_or("Setup not found")?;
    setup.delete_predefined_param_set(name).map_err(|err| err.to_string())?;
    update_data(&app_data, Some(data))?;
    Ok(())
}

/**
 * Updates a predefined set.
 *
 * `app_data` The application data.
 * `setupid` The setup id.
 * `old_name` The old name of the predefined set.
 * `predified_set` The predefined set.
 *
 * # Errors
 * If the configuration data could not be locked.
 * If the setup could not be found.
 * If the predefined set could not be updated.
 */
#[tauri::command]
pub fn update_predefined_set(app_data: State<'_, AppData>, setupid: &str, old_name: &str, predefined_set: PredefinedSet) -> Result<(), String> {
    let mut data = get_configuration_data(&app_data)?;
    let setup = data.get_setup(setupid).ok_or("Setup not found")?;
    setup.update_predefined_param_set(old_name, &predefined_set.name, predefined_set.values.clone()).map_err(|err| err.to_string())?;
    update_data(&app_data, Some(data))?;
    Ok(())
}

/**
 * Gets the servers.
 *
 * `app_data` The application data.
 * `setupid` The setup id.
 *
 * Returns:
 * The servers.
 *
 * # Errors
 * If the configuration data could not be locked.
 * If the setup could not be found.
 */
#[tauri::command]
pub async fn get_servers(app_data: State<'_, AppData>, setupid: &str) -> Result<Vec<HttpServerRow>, String> {
    let mut data = get_configuration_data(&app_data)?;
    let setup = data.get_setup(setupid).ok_or("Setup not found")?;
    let http_servers = setup.servers.iter().map(HttpServerRow::from).collect();
    Ok(http_servers)
}

/**
 * Updates a server.
 *
 * `app_data` The application data.
 * `setupid` The setup id.
 * `serverid` The server id.
 * `httpserver` The server.
 *
 * # Errors
 * If the configuration data could not be locked.
 * If the setup could not be found.
 * If the server could not be updated.
 */
#[tauri::command]
pub async fn update_server(app_data: State<'_, AppData>, setupid: &str, serverid: &str, httpserver: HttpServerRow) -> Result<(), String> {
    let mut data = get_configuration_data(&app_data)?;
    let server = data.get_server(setupid, serverid).ok_or("Setup not found")?;
    let https_config: Option<HttpsConfiguration> = httpserver.https_config.map(std::convert::Into::into);
    server.update(httpserver.name, httpserver.http_port, https_config);
    update_data(&app_data, Some(data))?;
    Ok(())
}

/**
 * Deletes a server.
 *
 * `app_data` The application data.
 * `setupid` The setup id.
 * `serverid` The server id.
 *
 * # Errors
 * If the configuration data could not be locked.
 * If the setup could not be found.
 * If the server could not be deleted.
 */
#[tauri::command]
pub async fn delete_server(app_data: State<'_, AppData>, setupid: &str, serverid: &str) -> Result<(), String> {
    let mut data = get_configuration_data(&app_data)?;
    data.delete_server(setupid, serverid).map_err(|err| err.to_string())?;
    update_data(&app_data, Some(data))?;
    Ok(())
}

/**
 * Adds a server.
 *
 * `app_data` The application data.
 * `setupid` The setup id.
 *
 * # Errors
 * If the configuration data could not be locked.
 * If the setup could not be found.
 * If the server could not be added.
 */
#[tauri::command]
pub async fn add_server(app_data: State<'_, AppData>, setupid: &str) -> Result<(), String> {
    let mut data = get_configuration_data(&app_data)?;
    let setup = data.get_setup(setupid).ok_or("Setup not found")?;
    setup.servers.push(ServerConfiguration::new(DEFAULT_NAME.to_owned(), None, Vec::new(), None).map_err(|err| err.to_string())?);
    update_data(&app_data, Some(data))?;
    Ok(())
}

/**
 * Gets the listeners.
 *
 * `app_data` The application data.
 * `setupid` The setup id.
 *
 * Returns:
 * The listeners.
 *
 * # Errors
 * If the configuration data could not be locked.
 * If the setup could not be found.
 */
#[tauri::command]
pub async fn get_listeners(app_data: State<'_, AppData>, setupid: &str) -> Result<Vec<TcpListenerRow>, String> {
    let data = get_configuration_data(&app_data)?;
    let setup = data.setups.iter().find(|t| t.id == setupid).ok_or("Setup not found")?;
    let tcp_listeners = setup.listeners.iter().map(TcpListenerRow::from).collect();
    Ok(tcp_listeners)
}

/**
 * Updates a listener.
 *
 * `app_data` The application data.
 * `setupid` The setup id.
 * `listenerid` The listener id.
 * `tcplistener` The listener.
 *
 * # Errors
 * If the configuration data could not be locked.
 * If the setup could not be found.
 * If the listener could not be updated.
 */
#[tauri::command]
pub async fn update_listener(app_data: State<'_, AppData>, setupid: &str, listenerid: &str, tcplistener: TcpListenerRow) -> Result<(), String> {
    let mut data = get_configuration_data(&app_data)?;
    data.update_listener(
        setupid,
        listenerid,
        tcplistener.file,
        tcplistener.data,
        tcplistener.delay_write_ms,
        tcplistener.port,
        tcplistener.accept,
        CloseConnectionWhen::from(tcplistener.close_connection.as_str()),
    )
    .map_err(|err| err.to_string())?;
    update_data(&app_data, Some(data))?;
    Ok(())
}

/**
 * Deletes a listener.
 *
 * `app_data` The application data.
 * `setupid` The setup id.
 * `listenerid` The listener id.
 *
 * # Errors
 * If the configuration data could not be locked.
 * If the setup could not be found.
 * If the listener could not be deleted.
 */
#[tauri::command]
pub async fn delete_listener(app_data: State<'_, AppData>, setupid: &str, listenerid: &str) -> Result<(), String> {
    let mut data = get_configuration_data(&app_data)?;
    data.delete_listener(setupid, listenerid).map_err(|err| err.to_string())?;
    update_data(&app_data, Some(data))?;
    Ok(())
}

/**
 * Adds a listener.
 *
 * `app_data` The application data.
 * `setupid` The setup id.
 *
 * # Errors
 * If the configuration data could not be locked.
 * If the setup could not be found.
 * If the listener could not be added.
 */
#[tauri::command]
pub async fn add_listener(app_data: State<'_, AppData>, setupid: &str) -> Result<(), String> {
    let mut data = get_configuration_data(&app_data)?;
    let setup = data.get_setup(setupid).ok_or("Setup not found")?;
    let () = setup.listeners.push(TcpListenerData::new(None, None, None, 8000, false, CloseConnectionWhen::AfterResponse).map_err(|err| err.to_string())?);
    update_data(&app_data, Some(data))?;
    Ok(())
}

/**
 * Adds an endpoint.
 *
 * `app_data` The application data.
 * `setupid` The setup id.
 * `serverid` The server id.
 *
 * # Errors
 * If the configuration data could not be locked.
 * If the setup could not be found.
 * If the server could not be found.
 * If the endpoint could not be added.
 */
#[tauri::command]
pub async fn add_endpoint(app_data: State<'_, AppData>, setupid: &str, serverid: &str) -> Result<(), String> {
    let mut data = get_configuration_data(&app_data)?;
    let server = data.get_server(setupid, serverid).ok_or("Server not found")?;
    server.endpoints.push(
        EndpointConfiguration::new(
            Some("/".to_owned()),
            Some("GET".to_owned()),
            Some(String::new()),
            Some(EndpointType::Mock { configuration: MockResponseConfiguration::new(None, DEFAULT_STATUS_CODE.to_string(), HashMap::new(), DEFAULT_DELAY) }),
        )
        .map_err(|err| err.to_string())?,
    );
    update_data(&app_data, Some(data))?;
    Ok(())
}

/**
 * Deletes an endpoint.
 *
 * `app_data` The application data.
 * `setupid` The setup id.
 * `serverid` The server id.
 * `endpointid` The endpoint id.
 *
 * # Errors
 * If the configuration data could not be locked.
 * If the setup could not be found.
 * If the server could not be found.
 * If the endpoint could not be deleted.
 */
#[tauri::command]
pub async fn delete_endpoint(app_data: State<'_, AppData>, setupid: &str, serverid: &str, endpointid: &str) -> Result<(), String> {
    let mut data = get_configuration_data(&app_data)?;
    data.delete_endpoint(setupid, serverid, endpointid).map_err(|err| err.to_string())?;
    update_data(&app_data, Some(data))?;
    Ok(())
}

/**
 * Updates an endpoint.
 *
 * `app_data` The application data.
 * `setupid` The setup id.
 * `serverid` The server id.
 * `endpointid` The endpoint id.
 * `endpoint` The endpoint.
 *
 * # Errors
 * If the configuration data could not be locked.
 * If the setup could not be found.
 * If the server could not be found.
 * If the endpoint could not be updated.
 */
#[allow(clippy::manual_map)]
#[tauri::command]
pub async fn update_endpoint(app_data: State<'_, AppData>, setupid: &str, serverid: &str, endpointid: &str, endpoint: EndpointRow) -> Result<(), String> {
    let mut data = get_configuration_data(&app_data)?;
    let endpoint_type = if let Some(mock_response) = &endpoint.mock {
        Some(EndpointType::Mock { configuration: mock_response.into() })
    } else if let Some(route_response) = &endpoint.route {
        Some(EndpointType::Route { configuration: route_response.into() })
    } else {
        None
    };
    data.update_endpoint(setupid, serverid, endpointid, endpoint.path_expression, endpoint.body_expression, endpoint.method, endpoint_type).map_err(|err| err.to_string())?;
    update_data(&app_data, Some(data))?;
    Ok(())
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
pub async fn start_setup(app_data: State<'_, AppData>, setupid: &str, params: HashMap<String, String>) -> Result<SetupRow, String> {
    let app_config = get_configuration_data(&app_data)?;
    let setup = app_config.setups.iter().find(|t| t.id == setupid).ok_or("Setup not found")?;
    let mut process_data = app_data.process_data.lock().map_err(|err| err.to_string())?;
    if let Some(process_data) = process_data.get(setupid) {
        Ok(SetupRow {
            id: setup.id.clone(),
            name: setup.name.clone(),
            description: setup.description.clone(),
            process_id: Some(process_data.process_id),
            params: setup.clone().params.map(|f| f.iter().cloned().collect::<Vec<_>>()),
        })
    } else {
        let file_path = get_current_file_path(&app_data)?;
        let path = Path::new(&file_path).parent().or_else(|| Some(Path::new("."))).ok_or("Invalid file path")?;
        let app_path = Settings::load().apinae_path.unwrap_or("apinae".to_owned());
        let mut process = std::process::Command::new(app_path);
        let mut process = process.arg("--file").arg(get_current_file_path(&app_data)?).arg("--id").arg(setupid);
        for (key, value) in params {
            process = process.arg("--param").arg(format!("{key}={value}"));
        }
        let process = process.current_dir(path.as_os_str()).spawn().map_err(|err| err.to_string())?;
        let process_id = process.id();
        process_data.insert(setupid.to_owned(), ProcessData::new(process_id, process));
        Ok(SetupRow::new(setup.id.as_str(), setup.name.as_str(), setup.description.as_str(), Some(process_id), setup.params.clone()))
    }
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
pub async fn stop_setup(app_data: State<'_, AppData>, setupid: &str) -> Result<SetupRow, String> {
    let app_config = get_configuration_data(&app_data)?;
    let setup = app_config.setups.iter().find(|t| t.id == setupid).ok_or("Setup not found")?;
    let mut processes_data = app_data.process_data.lock().map_err(|err| err.to_string())?;
    match processes_data.get_mut(setupid) {
        Some(process_data) => {
            process_data.process.kill().map_err(|err| err.to_string())?;
            processes_data.remove(setupid);
            Ok(SetupRow {
                id: setup.id.clone(),
                name: setup.name.clone(),
                description: setup.description.clone(),
                process_id: None,
                params: setup.clone().params.map(|f| f.iter().cloned().collect::<Vec<_>>()),
            })
        }
        None => Ok(SetupRow {
            id: setup.id.clone(),
            name: setup.name.clone(),
            description: setup.description.clone(),
            process_id: None,
            params: setup.clone().params.map(|f| f.iter().cloned().collect::<Vec<_>>()),
        }),
    }
}

/**
 * Saves the settings.
 *
 * `settings` The settings being saved.
 *
 * # Errors
 * If the settings could not be saved.
 */
#[tauri::command]
pub async fn save_settings(settings: Settings) -> Result<(), String> {
    settings.save().map_err(|err| err.to_string())
}

/**
 * Loads the settings.
 *
 * Returns:
 * The settings.
 */
#[tauri::command]
pub async fn load_settings() -> Result<Settings, String> {
    Ok(Settings::load())
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
        Ok(path.to_string_lossy().to_string())
    } else {
        Err("Invalid file path".to_string())
    }
}

/**
 * Updates the data.
 *
 * `app_data` The application data.
 * `new_data` The new data.
 *
 * # Errors
 * If the data could not be locked.
 */
fn update_data(app_data: &State<'_, AppData>, new_data: Option<AppConfiguration>) -> Result<(), String> {
    let mut lock = app_data.data.try_lock().map_err(|err| err.to_string())?;
    *lock = new_data;
    Ok(())
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
    Ok(())
}
