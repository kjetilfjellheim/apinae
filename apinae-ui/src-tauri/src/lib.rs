use std::sync::Mutex;

use apinae_lib::config::{AppConfiguration, TestConfiguration};
use tauri_plugin_dialog::{DialogExt, FilePath, MessageDialogButtons};
use tauri::{AppHandle, State};

#[tauri::command]
async fn load(app: AppHandle, app_data: State<'_, AppData>) -> Result<(String, AppConfiguration), String> {
    let file_path = app.dialog().file().blocking_pick_file();
    if let Some(file_path) = file_path {
        let file_path = get_file_path(file_path)?;
        let data = AppConfiguration::load(&file_path).map_err(|err| err.to_string())?;
        update_file_path(&app_data, Some(file_path.clone()))?;
        return Ok((file_path, data));
    }
    Err("No file selected".to_string())
}

#[tauri::command]
async fn save(app: AppHandle, app_data: State<'_, AppData>) -> Result<(), String> {
    let current_file_path = get_current_file_path(&app_data);
    if let Ok(current_file_path) = current_file_path {
        let data = get_data(&app_data)?;
        data.save(&current_file_path).map_err(|err| err.to_string())?;
        Ok(())
    } else {
        save_as(app, app_data).await
    }
}

#[tauri::command]
async fn save_as(app: AppHandle, app_data: State<'_, AppData>) -> Result<(), String> {
    let file_path = app.dialog().file().blocking_save_file();
    if let Some(file_path) = file_path {
        let file_path = get_file_path(file_path)?;
        let data = get_data(&app_data)?;
        data.save(&file_path).map_err(|err| err.to_string())?;
        update_file_path(&app_data, Some(file_path))?;
        Ok(())
    } else {
        Err("No file selected".to_string())
    }
}

#[tauri::command]
async fn clean(app: AppHandle, app_data: State<'_, AppData>) -> Result<AppConfiguration, String> {
    if confirm_dialog(&app) {
        let new_data = AppConfiguration::new(String::from("Untitled"), String::new(), Vec::new());
        update_data(&app_data, Some(new_data))?;
        update_file_path(&app_data, None)?;        
    }
    Ok(get_data(&app_data)?)
}

#[tauri::command]
async fn get_test(app_data: State<'_, AppData>, test_id: &str) -> Result<TestConfiguration, String> {
    let data = get_data(&app_data)?;
    let test = data.get_test(test_id).ok_or("Test not found".to_string())?;
    Ok(test)
}

fn get_data(app_data: &State<'_, AppData>) -> Result<AppConfiguration, String> {
    let lock = app_data.data.lock().map_err(|err| err.to_string())?;
    lock.clone().ok_or("No data".to_string())
}

fn get_current_file_path(app_data: &State<'_, AppData>) -> Result<String, String> {
    let lock = app_data.file_path.lock().map_err(|err| err.to_string())?;
    lock.clone().ok_or("No file path".to_string())
}

fn get_file_path(file_path: FilePath) -> Result<String, String> {
    if let FilePath::Path(path) = file_path {
        Ok(path.to_string_lossy().to_string())
    } else {
        Err("Invalid file path".to_string())
    }
}

fn confirm_dialog(app: &AppHandle) -> bool {
    app.dialog()
        .message("Are you sure?")
        .title("Confirm")
        .kind(tauri_plugin_dialog::MessageDialogKind::Warning)
        .buttons(MessageDialogButtons::YesNo)                
        .blocking_show()
}

fn update_data(app_data: &State<'_, AppData>, new_data: Option<AppConfiguration>) -> Result<(), String> {
    let mut lock = app_data.data.try_lock().map_err(|err| err.to_string())?;
    *lock = new_data;
    Ok(())
}

fn update_file_path(app_data: &State<'_, AppData>, new_file_path: Option<String>) -> Result<(), String> {
    let mut lock = app_data.file_path.try_lock().map_err(|err| err.to_string())?;
    *lock = new_file_path;
    Ok(())
}

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
        .invoke_handler(tauri::generate_handler![load, save, save_as, clean, get_test])        
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

struct AppData {
    data: Mutex<Option<AppConfiguration>>,
    file_path: Mutex<Option<String>>,
}

impl AppData {
    fn new() -> Self {
        Self {
            data: Mutex::new(AppConfiguration::new(String::from("Untitled"), String::new(), Vec::new()).into()),
            file_path: Mutex::new(None),
        }
    }
}