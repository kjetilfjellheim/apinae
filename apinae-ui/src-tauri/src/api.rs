use std::{collections::HashMap, path::Path};

use crate::AppData;
use crate::model::CollectionRow;
use crate::{
    model::SetupRow
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
        return Result::Ok(file_path);
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
        Result::Ok(())
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
pub async fn start_setup(app_data: State<'_, AppData>, setupid: &str, params: HashMap<String, String>) -> Result<(), String> {
    // let app_config = get_configuration_data(&app_data)?;
    // let setup = app_config.find_setup(setupid).ok_or("Setup not found")?;
    // let mut process_data = app_data.process_data.lock().map_err(|err| err.to_string())?;
    // if let Some(process_data) = process_data.get(setupid) {
    //     Ok(SetupRow {
    //         id: setup.id.clone(),
    //         name: setup.name.clone(),
    //         description: setup.description.clone(),
    //         process_id: Some(process_data.process_id),
    //         params: setup.clone().params.map(|f| f.iter().cloned().collect::<Vec<_>>()),
    //     })
    // } else {
    //     let file_path = get_current_file_path(&app_data)?;
    //     let path = Path::new(&file_path).parent().or_else(|| Some(Path::new("."))).ok_or("Invalid file path")?;
    //     let app_path = Settings::load().apinae_path.unwrap_or("apinae".to_owned());
    //     let mut process = std::process::Command::new(app_path);
    //     let mut process = process.arg("--file").arg(get_current_file_path(&app_data)?).arg("--id").arg(setupid);
    //     for (key, value) in params {
    //         process = process.arg("--param").arg(format!("{key}={value}"));
    //     }
    //     let process = process.current_dir(path.as_os_str()).spawn().map_err(|err| err.to_string())?;
    //     let process_id = process.id();
    //     process_data.insert(setupid.to_owned(), ProcessData::new(process_id, process));
    //     Ok(SetupRow::new(setup.id.as_str(), setup.name.as_str(), setup.description.as_str(), Some(process_id), setup.params.clone()))
    //}
    Err("".to_string())
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
    // let app_config = get_configuration_data(&app_data)?;
    // let setup = app_config.find_setup(setupid).ok_or("Setup not found")?;
    // let mut processes_data = app_data.process_data.lock().map_err(|err| err.to_string())?;
    // match processes_data.get_mut(setupid) {
    //     Some(process_data) => {
    //         process_data.process.kill().map_err(|err| err.to_string())?;
    //         processes_data.remove(setupid);
    //         Ok(SetupRow {
    //             id: setup.id.clone(),
    //             name: setup.name.clone(),
    //             description: setup.description.clone(),
    //             process_id: None,
    //             params: setup.clone().params.map(|f| f.iter().cloned().collect::<Vec<_>>()),
    //         })
    //     }
    //     None => Ok(SetupRow {
    //         id: setup.id.clone(),
    //         name: setup.name.clone(),
    //         description: setup.description.clone(),
    //         process_id: None,
    //         params: setup.clone().params.map(|f| f.iter().cloned().collect::<Vec<_>>()),
    //     }),
    // }
    Err("".to_string())
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
    let collections = app_config.get_collections().iter().map(|collection| {
        CollectionRow::from(collection)
    }).collect::<Vec<_>>();
    Result::Ok(collections)
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



