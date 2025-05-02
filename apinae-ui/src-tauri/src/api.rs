use std::collections::HashSet;
use std::{collections::HashMap, path::Path};

use crate::model::PredefinedSet;
use crate::AppData;
use crate::{
    model::{EndpointRow, HttpServerRow, TcpListenerRow, TestRow},
    state::ProcessData,
};
use apinae_lib::{
    config::{AppConfiguration, CloseConnectionWhen, EndpointConfiguration, EndpointType, HttpsConfiguration, MockResponseConfiguration, ServerConfiguration, TcpListenerData, TestConfiguration},
    settings::Settings,
};
use tauri::{AppHandle, State};
use tauri_plugin_dialog::{DialogExt, FilePath, MessageDialogButtons};

/**
 * Default name for new tests, servers, listeners and endpoints.
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
 * Gets the tests.
 *
 * `app_data` The application data.
 *
 * Returns:
 * The tests.
 *
 * # Errors
 * If the configuration data could not be locked.
 */
#[tauri::command]
pub async fn get_tests(app_data: State<'_, AppData>) -> Result<Vec<TestRow>, String> {
    let data = get_configuration_data(&app_data)?;
    let process_data = app_data.process_data.lock().map_err(|err| err.to_string())?;
    let tests = data
        .tests
        .iter()
        .map(|test| TestRow::from(test.clone()))
        .map(|mut test_row| {
            test_row.process_id = process_data.get(&test_row.id).map(|process_data| process_data.process_id);
            test_row
        })
        .collect();
    Ok(tests)
}

/**
 * Gets a test.
 *
 * `app_data` The application data.
 * `testid` The test id.
 *
 * Returns:
 * The test.
 *
 * # Errors
 * If the test could not be found.
 */
#[tauri::command]
pub async fn get_test(app_data: State<'_, AppData>, testid: &str) -> Result<TestRow, String> {
    let mut data = get_configuration_data(&app_data)?;
    let test = data.get_test(testid).ok_or("Could not find test")?;
    Ok(TestRow::from(test.clone()))
}

/**
 * Adds a test.
 *
 * `app_data` The application data.
 *
 * # Errors
 * If the configuration data could not be locked.
 * If the test could not be added.
 */
#[tauri::command]
pub async fn add_test(app_data: State<'_, AppData>) -> Result<(), String> {
    let mut data = get_configuration_data(&app_data)?;
    data.tests.push(TestConfiguration::new(DEFAULT_NAME.to_owned(), String::new(), Vec::new(), Vec::new(), None, None).map_err(|err| err.to_string())?);
    update_data(&app_data, Some(data))?;
    Ok(())
}

/**
 * Updates a test.
 *
 * `app_data` The application data.
 * `testid` The test id.
 * `test` The test.
 *
 * # Errors
 * If the configuration data could not be locked.
 * If the test could not be updated.
 */
#[tauri::command]
pub async fn update_test(app_data: State<'_, AppData>, testid: &str, test: TestRow) -> Result<(), String> {
    let mut data = get_configuration_data(&app_data)?;
    data.update_test(testid, test.name.as_str(), test.description.as_str(), test.params.map(|f| f.iter().cloned().collect::<HashSet<_>>())).map_err(|err| err.to_string())?;
    update_data(&app_data, Some(data))?;
    Ok(())
}

/**
 * Deletes a test.
 *
 * `app_data` The application data.
 * `testid` The test id.
 *
 * # Errors
 * If the configuration data could not be locked.
 * If the test could not be deleted.
 */
#[tauri::command]
pub async fn delete_test(app_data: State<'_, AppData>, testid: &str) -> Result<(), String> {
    let mut data = get_configuration_data(&app_data)?;
    data.delete_test(testid).map_err(|err| err.to_string())?;
    update_data(&app_data, Some(data))?;
    Ok(())
}

/**
 * Gets the predefined sets.
 *
 * `app_data` The application data.
 * `testid` The test id.
 *
 * Returns:
 * The predefined sets.
 *
 * # Errors
 * If the configuration data could not be locked.
 * If the test could not be found.
 */
#[tauri::command]
pub async fn get_predefined_sets(app_data: State<'_, AppData>, testid: &str) -> Result<Vec<PredefinedSet>, String> {
    let mut data = get_configuration_data(&app_data)?;
    let test = data.get_test(testid).ok_or("Test not found")?;
    let mut sets: Vec<PredefinedSet> = Vec::new();    
    if let Some(predefined_params) = &test.predefined_params {
        for predefined_set in predefined_params {
            let set = PredefinedSet::new(&predefined_set.name.clone(), predefined_set.values.clone());
            sets.push(set);
        }
    }
    Ok(sets)
}

/**
 * Gets the servers.
 *
 * `app_data` The application data.
 * `testid` The test id.
 *
 * Returns:
 * The servers.
 *
 * # Errors
 * If the configuration data could not be locked.
 * If the test could not be found.
 */
#[tauri::command]
pub async fn get_servers(app_data: State<'_, AppData>, testid: &str) -> Result<Vec<HttpServerRow>, String> {
    let mut data = get_configuration_data(&app_data)?;
    let test = data.get_test(testid).ok_or("Test not found")?;
    let http_servers = test.servers.iter().map(HttpServerRow::from).collect();
    Ok(http_servers)
}

/**
 * Updates a server.
 *
 * `app_data` The application data.
 * `testid` The test id.
 * `serverid` The server id.
 * `httpserver` The server.
 *
 * # Errors
 * If the configuration data could not be locked.
 * If the test could not be found.
 * If the server could not be updated.
 */
#[tauri::command]
pub async fn update_server(app_data: State<'_, AppData>, testid: &str, serverid: &str, httpserver: HttpServerRow) -> Result<(), String> {
    let mut data = get_configuration_data(&app_data)?;
    let server = data.get_server(testid, serverid).ok_or("Test not found")?;
    let https_config: Option<HttpsConfiguration> = httpserver.https_config.map(std::convert::Into::into);
    server.update(httpserver.name, httpserver.http_port, https_config);
    update_data(&app_data, Some(data))?;
    Ok(())
}

/**
 * Deletes a server.
 *
 * `app_data` The application data.
 * `testid` The test id.
 * `serverid` The server id.
 *
 * # Errors
 * If the configuration data could not be locked.
 * If the test could not be found.
 * If the server could not be deleted.
 */
#[tauri::command]
pub async fn delete_server(app_data: State<'_, AppData>, testid: &str, serverid: &str) -> Result<(), String> {
    let mut data = get_configuration_data(&app_data)?;
    data.delete_server(testid, serverid).map_err(|err| err.to_string())?;
    update_data(&app_data, Some(data))?;
    Ok(())
}

/**
 * Adds a server.
 *
 * `app_data` The application data.
 * `testid` The test id.
 *
 * # Errors
 * If the configuration data could not be locked.
 * If the test could not be found.
 * If the server could not be added.
 */
#[tauri::command]
pub async fn add_server(app_data: State<'_, AppData>, testid: &str) -> Result<(), String> {
    let mut data = get_configuration_data(&app_data)?;
    let test = data.get_test(testid).ok_or("Test not found")?;
    test.servers.push(ServerConfiguration::new(DEFAULT_NAME.to_owned(), None, Vec::new(), None).map_err(|err| err.to_string())?);
    update_data(&app_data, Some(data))?;
    Ok(())
}

/**
 * Gets the listeners.
 *
 * `app_data` The application data.
 * `testid` The test id.
 *
 * Returns:
 * The listeners.
 *
 * # Errors
 * If the configuration data could not be locked.
 * If the test could not be found.
 */
#[tauri::command]
pub async fn get_listeners(app_data: State<'_, AppData>, testid: &str) -> Result<Vec<TcpListenerRow>, String> {
    let data = get_configuration_data(&app_data)?;
    let test = data.tests.iter().find(|t| t.id == testid).ok_or("Test not found")?;
    let tcp_listeners = test.listeners.iter().map(TcpListenerRow::from).collect();
    Ok(tcp_listeners)
}

/**
 * Updates a listener.
 *
 * `app_data` The application data.
 * `testid` The test id.
 * `listenerid` The listener id.
 * `tcplistener` The listener.
 *
 * # Errors
 * If the configuration data could not be locked.
 * If the test could not be found.
 * If the listener could not be updated.
 */
#[tauri::command]
pub async fn update_listener(app_data: State<'_, AppData>, testid: &str, listenerid: &str, tcplistener: TcpListenerRow) -> Result<(), String> {
    let mut data = get_configuration_data(&app_data)?;
    data.update_listener(
        testid,
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
 * `testid` The test id.
 * `listenerid` The listener id.
 *
 * # Errors
 * If the configuration data could not be locked.
 * If the test could not be found.
 * If the listener could not be deleted.
 */
#[tauri::command]
pub async fn delete_listener(app_data: State<'_, AppData>, testid: &str, listenerid: &str) -> Result<(), String> {
    let mut data = get_configuration_data(&app_data)?;
    data.delete_listener(testid, listenerid).map_err(|err| err.to_string())?;
    update_data(&app_data, Some(data))?;
    Ok(())
}

/**
 * Adds a listener.
 *
 * `app_data` The application data.
 * `testid` The test id.
 *
 * # Errors
 * If the configuration data could not be locked.
 * If the test could not be found.
 * If the listener could not be added.
 */
#[tauri::command]
pub async fn add_listener(app_data: State<'_, AppData>, testid: &str) -> Result<(), String> {
    let mut data = get_configuration_data(&app_data)?;
    let test = data.get_test(testid).ok_or("Test not found")?;
    let () = test.listeners.push(TcpListenerData::new(None, None, None, 8000, false, CloseConnectionWhen::AfterResponse).map_err(|err| err.to_string())?);
    update_data(&app_data, Some(data))?;
    Ok(())
}

/**
 * Adds an endpoint.
 *
 * `app_data` The application data.
 * `testid` The test id.
 * `serverid` The server id.
 *
 * # Errors
 * If the configuration data could not be locked.
 * If the test could not be found.
 * If the server could not be found.
 * If the endpoint could not be added.
 */
#[tauri::command]
pub async fn add_endpoint(app_data: State<'_, AppData>, testid: &str, serverid: &str) -> Result<(), String> {
    let mut data = get_configuration_data(&app_data)?;
    let server = data.get_server(testid, serverid).ok_or("Server not found")?;
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
 * `testid` The test id.
 * `serverid` The server id.
 * `endpointid` The endpoint id.
 *
 * # Errors
 * If the configuration data could not be locked.
 * If the test could not be found.
 * If the server could not be found.
 * If the endpoint could not be deleted.
 */
#[tauri::command]
pub async fn delete_endpoint(app_data: State<'_, AppData>, testid: &str, serverid: &str, endpointid: &str) -> Result<(), String> {
    let mut data = get_configuration_data(&app_data)?;
    data.delete_endpoint(testid, serverid, endpointid).map_err(|err| err.to_string())?;
    update_data(&app_data, Some(data))?;
    Ok(())
}

/**
 * Updates an endpoint.
 *
 * `app_data` The application data.
 * `testid` The test id.
 * `serverid` The server id.
 * `endpointid` The endpoint id.
 * `endpoint` The endpoint.
 *
 * # Errors
 * If the configuration data could not be locked.
 * If the test could not be found.
 * If the server could not be found.
 * If the endpoint could not be updated.
 */
#[allow(clippy::manual_map)]
#[tauri::command]
pub async fn update_endpoint(app_data: State<'_, AppData>, testid: &str, serverid: &str, endpointid: &str, endpoint: EndpointRow) -> Result<(), String> {
    let mut data = get_configuration_data(&app_data)?;
    let endpoint_type = if let Some(mock_response) = &endpoint.mock {
        Some(EndpointType::Mock { configuration: mock_response.into() })
    } else if let Some(route_response) = &endpoint.route {
        Some(EndpointType::Route { configuration: route_response.into() })
    } else {
        None
    };
    data.update_endpoint(testid, serverid, endpointid, endpoint.path_expression, endpoint.body_expression, endpoint.method, endpoint_type).map_err(|err| err.to_string())?;
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
 * Starts a test.
 *
 * `app_data` The application data.
 * `testid` The test id.
 *
 * Returns:
 * The test.
 *
 * # Errors
 * If the configuration data could not be locked.
 * If the test could not be found.
 * If the test could not be started.
 */
#[tauri::command]
pub async fn start_test(app_data: State<'_, AppData>, testid: &str) -> Result<TestRow, String> {
    let app_config = get_configuration_data(&app_data)?;
    let test = app_config.tests.iter().find(|t| t.id == testid).ok_or("Test not found")?;
    let mut process_data = app_data.process_data.lock().map_err(|err| err.to_string())?;
    if let Some(process_data) = process_data.get(testid) {
        Ok(TestRow {
            id: test.id.clone(),
            name: test.name.clone(),
            description: test.description.clone(),
            process_id: Some(process_data.process_id),
            params: test.clone().params.map(|f| f.iter().cloned().collect::<Vec<_>>()),
        })
    } else {
        let file_path = get_current_file_path(&app_data)?;
        let path = Path::new(&file_path).parent().or_else(|| Some(Path::new("."))).ok_or("Invalid file path")?;
        let app_path = Settings::load().apinae_path.unwrap_or("apinae".to_owned());
        let process =
            std::process::Command::new(app_path).arg("--file").arg(get_current_file_path(&app_data)?).arg("--id").arg(testid).current_dir(path.as_os_str()).spawn().map_err(|err| err.to_string())?;
        let process_id = process.id();
        process_data.insert(testid.to_owned(), ProcessData::new(process_id, process));
        Ok(TestRow::new(test.id.as_str(), test.name.as_str(), test.description.as_str(), Some(process_id), test.params.clone()))
    }
}

/**
 * Stops a test.
 *
 * `app_data` The application data.
 * `testid` The test id.
 *
 * Returns:
 * The test.
 *
 * # Errors
 * If the configuration data could not be locked.
 * If the test could not be found.
 * If the test could not be stopped.
 */
#[allow(clippy::all)]
#[tauri::command]
pub async fn stop_test(app_data: State<'_, AppData>, testid: &str) -> Result<TestRow, String> {
    let app_config = get_configuration_data(&app_data)?;
    let test = app_config.tests.iter().find(|t| t.id == testid).ok_or("Test not found")?;
    let mut processes_data = app_data.process_data.lock().map_err(|err| err.to_string())?;
    match processes_data.get_mut(testid) {
        Some(process_data) => {
            process_data.process.kill().map_err(|err| err.to_string())?;
            processes_data.remove(testid);
            Ok(TestRow {
                id: test.id.clone(),
                name: test.name.clone(),
                description: test.description.clone(),
                process_id: None,
                params: test.clone().params.map(|f| f.iter().cloned().collect::<Vec<_>>()),
            })
        }
        None => Ok(TestRow {
            id: test.id.clone(),
            name: test.name.clone(),
            description: test.description.clone(),
            process_id: None,
            params: test.clone().params.map(|f| f.iter().cloned().collect::<Vec<_>>()),
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
