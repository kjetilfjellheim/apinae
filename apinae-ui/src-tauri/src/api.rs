use std::path::Path;

use crate::{model::{EndpointRow, HttpServerRow, TcpListenerRow, TestRow}, state::ProcessData};
use tauri::{AppHandle, State};
use crate::AppData;
use apinae_lib::config::{AppConfiguration, CloseConnectionWhen, EndpointConfiguration, HttpsConfiguration, ServerConfiguration, TcpListenerData};
use tauri_plugin_dialog::{DialogExt, FilePath, MessageDialogButtons};

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

#[tauri::command]
pub async fn clean(app: AppHandle, app_data: State<'_, AppData>) -> Result<AppConfiguration, String> {
    if confirm_dialog(app).await {
        let new_data = AppConfiguration::new(String::from("Untitled"), String::new(), Vec::new());
        update_data(&app_data, Some(new_data))?;
        update_file_path(&app_data, None)?;        
    }
    Ok(get_configuration_data(&app_data)?)
}

#[tauri::command]
pub async fn get_tests(app_data: State<'_, AppData>) -> Result<Vec<TestRow>, String> {
    let data = get_configuration_data(&app_data)?;
    let process_data = app_data.process_data.lock().map_err(|err| err.to_string())?;
    let tests = data.tests.iter().map(|test| {        
        TestRow::from(test.clone())
    }).map(|mut test_row| {
        test_row.process_id = process_data.get(&test_row.id).map(|process_data| process_data.process_id);
        test_row
    }).collect();
    Ok(tests)
}

#[tauri::command]
pub async fn get_test(app_data: State<'_, AppData>, testid: &str) -> Result<TestRow, String> {
    let mut data = get_configuration_data(&app_data)?;
    let test = data.get_test(testid).ok_or("Could not find test")?;
    Ok(TestRow::from(test.clone()))
}

#[tauri::command]
pub async fn add_test(app_data: State<'_, AppData>) -> Result<(), String> {
    let mut data = get_configuration_data(&app_data)?;
    data.tests.push(apinae_lib::config::TestConfiguration::new("Untitled".to_owned(), "".to_owned(), Vec::new(), Vec::new()).map_err(|err| err.to_string())?);
    update_data(&app_data, Some(data))?;
    Ok(())
}

#[tauri::command]
pub async fn update_test(app_data: State<'_, AppData>, testid: &str, test: TestRow) -> Result<(), String> {
    let mut data = get_configuration_data(&app_data)?;
    data.update_test(testid, test.name.as_str(), test.description.as_str()).map_err(|err| err.to_string())?;    
    update_data(&app_data, Some(data))?;
    Ok(())
}

#[tauri::command]
pub async fn delete_test(app_data: State<'_, AppData>, testid: &str) -> Result<(), String> {
    let mut data = get_configuration_data(&app_data)?;
    data.delete_test(testid).map_err(|err| err.to_string())?;
    update_data(&app_data, Some(data))?;
    Ok(())
}

#[tauri::command]
pub async fn get_servers(app_data: State<'_, AppData>, testid: &str) -> Result<Vec<HttpServerRow>, String> {
    let mut data = get_configuration_data(&app_data)?;
    let test = data.get_test(testid).ok_or("Test not found")?;
    let http_servers = test.servers.iter().map(|http_server| {
        HttpServerRow::from(http_server)
    }).collect();
    Ok(http_servers)
}

#[tauri::command]
pub async fn update_server(app_data: State<'_, AppData>, testid: &str, serverid: &str, httpserver: HttpServerRow) -> Result<(), String> {
    let mut data = get_configuration_data(&app_data)?;
    let server = data.get_server(testid, serverid).ok_or("Test not found")?;
    let https_config: Option<HttpsConfiguration> = httpserver.https_config.map(|https_row| {
        https_row.into()
    });
    server.update(httpserver.name, httpserver.http_port, https_config);
    update_data(&app_data, Some(data))?;
    Ok(())
}

#[tauri::command]
pub async fn delete_server(app_data: State<'_, AppData>, testid: &str, serverid: &str) -> Result<(), String> {
    let mut data = get_configuration_data(&app_data)?;
    data.delete_server(testid, serverid).map_err(|err| err.to_string())?;
    update_data(&app_data, Some(data))?;
    Ok(())
}

#[tauri::command]
pub async fn add_server(app_data: State<'_, AppData>, testid: &str) -> Result<(), String> {
    let mut data = get_configuration_data(&app_data)?;
    let test = data.get_test(testid).ok_or("Test not found")?;
    test.servers.push(ServerConfiguration::new("Untitled".to_owned(), None, Vec::new(), None).map_err(|err| err.to_string())?);
    update_data(&app_data, Some(data))?;
    Ok(())
}

#[tauri::command]
pub async fn get_listeners(app_data: State<'_, AppData>, testid: &str) -> Result<Vec<TcpListenerRow>, String> {
    let data = get_configuration_data(&app_data)?;
    let test = data.tests.iter().find(|t| t.id == testid).ok_or("Test not found")?;
    let tcp_listeners = test.listeners.iter().map(|tcp_listener| {
        TcpListenerRow::from(tcp_listener)
    }).collect();
    Ok(tcp_listeners)
}

#[tauri::command]
pub async fn update_listener(app_data: State<'_, AppData>, testid: &str, listenerid: &str, tcplistener: TcpListenerRow) -> Result<(), String> {
    let mut data = get_configuration_data(&app_data)?;
    let test = data.tests.iter_mut().find(|t| t.id == testid).ok_or("Test not found")?;
    let tcp_listener_index = test.listeners.iter_mut().position(|s| s.id == listenerid).ok_or("Listener not found")?;
    test.listeners[tcp_listener_index].port = tcplistener.port;
    test.listeners[tcp_listener_index].file = tcplistener.file;
    test.listeners[tcp_listener_index].data = tcplistener.data;
    test.listeners[tcp_listener_index].delay_write_ms = tcplistener.delay_write_ms;
    test.listeners[tcp_listener_index].accept = tcplistener.accept;
    test.listeners[tcp_listener_index].close_connection = CloseConnectionWhen::from(tcplistener.close_connection.as_str());    
    update_data(&app_data, Some(data))?;
    Ok(())
}

#[tauri::command]
pub async fn delete_listener(app_data: State<'_, AppData>, testid: &str, listenerid: &str) -> Result<(), String> {
    let mut data = get_configuration_data(&app_data)?;
    data.delete_listener(testid, listenerid).map_err(|err| err.to_string())?;
    update_data(&app_data, Some(data))?;
    Ok(())
}

#[tauri::command]
pub async fn add_listener(app_data: State<'_, AppData>, testid: &str) -> Result<(), String> {
    let mut data = get_configuration_data(&app_data)?;
    let test = data.get_test(testid).ok_or("Test not found")?;
    let _ = test.listeners.push(TcpListenerData::new(None, None, None, 0, false, CloseConnectionWhen::AfterResponse).map_err(|err| err.to_string())?);
    update_data(&app_data, Some(data))?;
    Ok(())
}

#[tauri::command]
pub async fn add_endpoint(app_data: State<'_, AppData>, testid: &str, serverid: &str) -> Result<(), String> {
    let mut data = get_configuration_data(&app_data)?;
    let server = data.get_server(testid, serverid).ok_or("Server not found")?;
    server.endpoints.push(EndpointConfiguration::new("Untitled".to_owned(), "".to_owned(), None, None).map_err(|err| err.to_string())?);
    update_data(&app_data, Some(data))?;
    Ok(())
}

#[tauri::command]
pub async fn delete_endpoint(app_data: State<'_, AppData>, testid: &str, serverid: &str, endpointid: &str) -> Result<(), String> {
    let mut data = get_configuration_data(&app_data)?;
    data.delete_endpoint(testid, serverid, endpointid).map_err(|err| err.to_string())?;
    update_data(&app_data, Some(data))?;
    Ok(())  
}

#[tauri::command]
pub async fn update_endpoint(app_data: State<'_, AppData>, testid: &str, serverid: &str, endpointid: &str, endpoint: EndpointRow) -> Result<(), String> {
    let mut data = get_configuration_data(&app_data)?;
    let mock_response = endpoint.mock.map(|mock_response| {
        mock_response.into()
    });
    let route_response = endpoint.route.map(|route_response| {
        route_response.into()
    });
    data.update_endpoint(testid, serverid, endpointid, endpoint.path_expression.as_str(), endpoint.method.as_str(), mock_response, route_response).map_err(|err| err.to_string())?;
    update_data(&app_data, Some(data))?;
    Ok(())
}

#[tauri::command]
pub async fn confirm_dialog(app: AppHandle) -> bool {
    app.dialog()
        .message("Are you sure?")
        .title("Confirm")
        .kind(tauri_plugin_dialog::MessageDialogKind::Warning)
        .buttons(MessageDialogButtons::YesNo)                
        .blocking_show()
}

#[tauri::command]
pub async fn start_test(app_data: State<'_, AppData>, testid: &str) -> Result<TestRow, String> {
    let app_config = get_configuration_data(&app_data)?;
    let test = app_config.tests.iter().find(|t| t.id == testid).ok_or("Test not found")?;
    let mut process_data = app_data.process_data.lock().map_err(|err| err.to_string())?;
    match process_data.get(testid) {
        Some(process_data) => {            
            Ok(TestRow {
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
            Ok(TestRow {
                id: test.id.clone(),
                name: test.name.clone(),
                description: test.description.clone(),
                process_id: Some(process_id),
            })
        }
    }
}

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
            })
        },
        None => {
            Ok(TestRow {
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
