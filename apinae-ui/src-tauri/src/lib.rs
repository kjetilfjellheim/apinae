use std::{collections::HashMap, path::Path, sync::Mutex};

use apinae_lib::config::AppConfiguration;
use tauri_plugin_dialog::{DialogExt, FilePath, MessageDialogButtons};
use tauri::{AppHandle, State};

#[tauri::command]
async fn load(app: AppHandle, app_data: State<'_, AppData>) -> Result<String, String> {
    let file_path = app.dialog().file().blocking_pick_file();
    if let Some(file_path) = file_path {
        let file_path = get_file_path(file_path)?;
        update_data(&app_data, Some(AppConfiguration::load(&file_path).map_err(|err| err.to_string())?))?;
        update_file_path(&app_data, Some(file_path.clone()))?;
        return Ok(file_path);
    }
    Err("No file selected".to_string())
}

#[tauri::command]
async fn save(app: AppHandle, app_data: State<'_, AppData>) -> Result<(), String> {
    let current_file_path = get_current_file_path(&app_data);
    if let Ok(current_file_path) = current_file_path {
        let data = get_configuration_data(&app_data)?;
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
        let data = get_configuration_data(&app_data)?;
        data.save(&file_path).map_err(|err| err.to_string())?;
        update_file_path(&app_data, Some(file_path))?;
        Ok(())
    } else {
        Err("No file selected".to_string())
    }
}

#[tauri::command]
async fn clean(app: AppHandle, app_data: State<'_, AppData>) -> Result<AppConfiguration, String> {
    if confirm_dialog(app).await {
        let new_data = AppConfiguration::new(String::from("Untitled"), String::new(), Vec::new());
        update_data(&app_data, Some(new_data))?;
        update_file_path(&app_data, None)?;        
    }
    Ok(get_configuration_data(&app_data)?)
}

#[tauri::command]
async fn get_tests(app_data: State<'_, AppData>) -> Result<Vec<TestRowResponse>, String> {
    let data = get_configuration_data(&app_data)?;
    let process_data = app_data.process_data.lock().map_err(|err| err.to_string())?;
    let tests = data.tests.iter().map(|test| {        
        TestRowResponse::from(test)
    }).map(|mut test_row| {
        test_row.process_id = process_data.get(&test_row.id).map(|process_data| process_data.process_id);
        test_row
    }).collect();
    Ok(tests)
}

#[tauri::command]
async fn add_test(app_data: State<'_, AppData>) -> Result<(), String> {
    let mut data = get_configuration_data(&app_data)?;
    data.tests.push(apinae_lib::config::TestConfiguration::new("Untitled".to_owned(), "".to_owned(), Vec::new(), Vec::new()).map_err(|err| err.to_string())?);
    update_data(&app_data, Some(data))?;
    Ok(())
}

#[tauri::command]
async fn confirm_dialog(app: AppHandle) -> bool {
    app.dialog()
        .message("Are you sure?")
        .title("Confirm")
        .kind(tauri_plugin_dialog::MessageDialogKind::Warning)
        .buttons(MessageDialogButtons::YesNo)                
        .blocking_show()
}

#[tauri::command]
async fn update_test_data(app_data: State<'_, AppData>, test: TestRowResponse) -> Result<(), String> {
    let mut data = get_configuration_data(&app_data)?;
    let index = data.tests.iter().position(|t| t.id == test.id).ok_or("Test not found")?;
    data.tests[index].name = test.name;
    data.tests[index].description = test.description;
    update_data(&app_data, Some(data))?;
    Ok(())
}

#[tauri::command]
async fn delete_test(app_data: State<'_, AppData>, testid: &str) -> Result<(), String> {
    let mut data = get_configuration_data(&app_data)?;
    let index = data.tests.iter().position(|t| t.id == testid).ok_or("Test not found")?;
    data.tests.remove(index);
    update_data(&app_data, Some(data))?;
    Ok(())
}

#[tauri::command]
async fn start_test(app_data: State<'_, AppData>, testid: &str) -> Result<TestRowResponse, String> {
    let app_config = get_configuration_data(&app_data)?;
    let test = app_config.tests.iter().find(|t| t.id == testid).ok_or("Test not found")?;
    let mut process_data = app_data.process_data.lock().map_err(|err| err.to_string())?;
    match process_data.get(testid) {
        Some(process_data) => {            
            Ok(TestRowResponse {
                id: test.id.clone(),
                name: test.name.clone(),
                description: test.description.clone(),
                process_id: Some(process_data.process_id),
            })            
        },
        None => {
            let file_path = get_current_file_path(&app_data)?;
            let path = Path::new(&file_path).parent().or_else(|| Some(Path::new("."))).ok_or("Invalid file path")?;
            let process = std::process::Command::new("apinae")        
                .arg("--file")
                .arg(get_current_file_path(&app_data)?)
                .arg("--id")
                .arg(testid)
                .current_dir(path.as_os_str())
                .spawn()
                .map_err(|err| err.to_string())?;
            let process_id = process.id();
            process_data.insert(testid.to_owned(), ProcessData::new(process_id, process));
            Ok(TestRowResponse {
                id: test.id.clone(),
                name: test.name.clone(),
                description: test.description.clone(),
                process_id: Some(process_id),
            })
        }
    }
}

#[tauri::command]
async fn stop_test(app_data: State<'_, AppData>, testid: &str) -> Result<TestRowResponse, String> {
    let app_config = get_configuration_data(&app_data)?;
    let test = app_config.tests.iter().find(|t| t.id == testid).ok_or("Test not found")?;
    let mut processes_data = app_data.process_data.lock().map_err(|err| err.to_string())?;
    match processes_data.get_mut(testid) {
        Some(process_data) => {   
            process_data.process.kill().map_err(|err| err.to_string())?;
            processes_data.remove(testid);
            Ok(TestRowResponse {
                id: test.id.clone(),
                name: test.name.clone(),
                description: test.description.clone(),
                process_id: None,
            })
        },
        None => {
            Ok(TestRowResponse {
                id: test.id.clone(),
                name: test.name.clone(),
                description: test.description.clone(),
                process_id: None,
            })            
        }
    }
}

fn get_configuration_data(app_data: &State<'_, AppData>) -> Result<AppConfiguration, String> {
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
        .invoke_handler(tauri::generate_handler![load, save, save_as, clean, get_tests, confirm_dialog, update_test_data, delete_test, add_test, start_test, stop_test])        
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

struct AppData {
    data: Mutex<Option<AppConfiguration>>,
    file_path: Mutex<Option<String>>,
    process_data: Mutex<HashMap<String, ProcessData>>,
}

impl AppData {
    fn new() -> Self {
        Self {
            data: Mutex::new(AppConfiguration::new(String::from("Untitled"), String::new(), Vec::new()).into()),
            file_path: Mutex::new(None),
            process_data: Mutex::new(HashMap::new()),
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct TestRowResponse {
    id: String,
    name: String,
    description: String,
    process_id: Option<u32>,
}

impl From<&apinae_lib::config::TestConfiguration> for TestRowResponse {
    fn from(test: &apinae_lib::config::TestConfiguration) -> Self {
        Self {
            id: test.id.clone(),
            name: test.name.clone(),
            description: test.description.clone(),
            process_id: None,
        }
    }
}

struct ProcessData {
    process_id: u32,
    process: std::process::Child,
}

impl ProcessData {
    fn new(process_id: u32, process: std::process::Child) -> Self {
        Self {
            process_id,
            process
        }
    }
}