use std::{collections::HashMap, path::Path, sync::Mutex};

use apinae_lib::config::{AppConfiguration, CloseConnectionWhen, ServerConfiguration, TcpListenerData, TlsVersion};
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
async fn get_test(app_data: State<'_, AppData>, testid: &str) -> Result<TestRowResponse, String> {
    let data = get_configuration_data(&app_data)?;
    let test = data.tests.iter().find(|t| t.id == testid).ok_or("Test not found")?;
    Ok(test.into())
}

#[tauri::command]
async fn add_test(app_data: State<'_, AppData>) -> Result<(), String> {
    let mut data = get_configuration_data(&app_data)?;
    data.tests.push(apinae_lib::config::TestConfiguration::new("Untitled".to_owned(), "".to_owned(), Vec::new(), Vec::new()).map_err(|err| err.to_string())?);
    update_data(&app_data, Some(data))?;
    Ok(())
}

#[tauri::command]
async fn update_test(app_data: State<'_, AppData>, testid: &str, test: TestRowResponse) -> Result<(), String> {
    let mut data = get_configuration_data(&app_data)?;
    let index = data.tests.iter().position(|t| t.id == testid).ok_or("Test not found")?;
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
async fn get_test_http_servers(app_data: State<'_, AppData>, testid: &str) -> Result<Vec<HttpServerRowResponse>, String> {
    let data = get_configuration_data(&app_data)?;
    let test = data.tests.iter().find(|t| t.id == testid).ok_or("Test not found")?;
    let http_servers = test.servers.iter().map(|http_server| {
        HttpServerRowResponse::from(http_server)
    }).collect();
    Ok(http_servers)
}

#[tauri::command]
async fn update_test_http_server(app_data: State<'_, AppData>, testid: &str, serverid: &str, httpserver: HttpServerRowResponse) -> Result<(), String> {
    let mut data = get_configuration_data(&app_data)?;
    let test = data.tests.iter_mut().find(|t| t.id == testid).ok_or("Test not found")?;
    let http_server_index = test.servers.iter().position(|s| s.id == serverid).ok_or("Server not found")?;
    test.servers[http_server_index].name = httpserver.name;
    test.servers[http_server_index].http_port = httpserver.http_port;
    if let Some(https_config) = httpserver.https_config {
        let new_https_config = apinae_lib::config::HttpsConfiguration::new(
            https_config.server_certificate, 
            https_config.private_key, 
            https_config.https_port, 
            https_config.client_certificate, 
            https_config.supported_tls_versions.into_iter().map(|value| TlsVersion::from(value.as_str())).collect());
        test.servers[http_server_index].https_config = Some(new_https_config);
    } else {
        test.servers[http_server_index].https_config = None;
    }
    update_data(&app_data, Some(data))?;
    Ok(())
}

#[tauri::command]
async fn delete_test_http_server(app_data: State<'_, AppData>, testid: &str, serverid: &str) -> Result<(), String> {
    let mut data = get_configuration_data(&app_data)?;
    let test = data.tests.iter_mut().find(|t| t.id == testid).ok_or("Test not found")?;
    let server_index = test.servers.iter().position(|s| s.id == serverid).ok_or("Server not found")?;
    test.servers.remove(server_index);
    update_data(&app_data, Some(data))?;
    Ok(())
}

#[tauri::command]
async fn add_test_http_server(app_data: State<'_, AppData>, testid: &str) -> Result<(), String> {
    let mut data = get_configuration_data(&app_data)?;
    let test = data.tests.iter_mut().find(|t| t.id == testid).ok_or("Test not found")?;
    let new_server = ServerConfiguration::new("Untitled".to_owned(), None, Vec::new(), None).map_err(|err| err.to_string())?;
    let _ = test.servers.push(new_server);
    update_data(&app_data, Some(data))?;
    Ok(())
}

#[tauri::command]
async fn get_test_tcp_listeners(app_data: State<'_, AppData>, testid: &str) -> Result<Vec<TcpListenerRowResponse>, String> {
    let data = get_configuration_data(&app_data)?;
    let test = data.tests.iter().find(|t| t.id == testid).ok_or("Test not found")?;
    let tcp_listeners = test.listeners.iter().map(|tcp_listener| {
        TcpListenerRowResponse::from(tcp_listener)
    }).collect();
    Ok(tcp_listeners)
}

#[tauri::command]
async fn update_test_tcp_listener(app_data: State<'_, AppData>, testid: &str, listenerid: &str, tcplistener: TcpListenerRowResponse) -> Result<(), String> {
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
async fn delete_test_tcp_listener(app_data: State<'_, AppData>, testid: &str, listenerid: &str) -> Result<(), String> {
    let mut data = get_configuration_data(&app_data)?;
    let test = data.tests.iter_mut().find(|t| t.id == testid).ok_or("Test not found")?;
    let listener_index = test.listeners.iter().position(|s| s.id == listenerid).ok_or("Listener not found")?;
    test.listeners.remove(listener_index);
    update_data(&app_data, Some(data))?;
    Ok(())
}

#[tauri::command]
async fn add_test_tcp_listener(app_data: State<'_, AppData>, testid: &str) -> Result<(), String> {
    let mut data = get_configuration_data(&app_data)?;
    let test = data.tests.iter_mut().find(|t| t.id == testid).ok_or("Test not found")?;
    let new_listener = TcpListenerData::new(None, None, None, 0, false, CloseConnectionWhen::AfterResponse).map_err(|err| err.to_string())?;
    let _ = test.listeners.push(new_listener);
    update_data(&app_data, Some(data))?;
    Ok(())
}

#[tauri::command]
async fn add_endpoint(app_data: State<'_, AppData>, testid: &str, serverid: &str) -> Result<(), String> {
    let mut data = get_configuration_data(&app_data)?;
    let test = data.tests.iter_mut().find(|t| t.id == testid).ok_or("Test not found")?;
    let server = test.servers.iter_mut().find(|s| s.id == serverid).ok_or("Server not found")?;
    server.endpoints.push(apinae_lib::config::EndpointConfiguration::new("Untitled".to_owned(), "GET".to_owned(), None, None).map_err(|err| err.to_string())?);
    update_data(&app_data, Some(data))?;
    Ok(())
}

#[tauri::command]
async fn delete_endpoint(app_data: State<'_, AppData>, testid: &str, serverid: &str, endpointid: &str) -> Result<(), String> {
    let mut data = get_configuration_data(&app_data)?;
    let test = data.tests.iter_mut().find(|t| t.id == testid).ok_or("Test not found")?;
    let server = test.servers.iter_mut().find(|s| s.id == serverid).ok_or("Server not found")?;  
    let endpoint_index = server.endpoints.iter().position(|e| e.id == endpointid).ok_or("Endpoint not found")?;
    server.endpoints.remove(endpoint_index);
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
        .invoke_handler(tauri::generate_handler![load, save, save_as, clean, 
            get_tests, get_test, update_test, add_test, delete_test,
            get_test_http_servers, update_test_http_server, add_test_http_server, delete_test_http_server,
            get_test_tcp_listeners, update_test_tcp_listener, add_test_tcp_listener, delete_test_tcp_listener, 
            add_endpoint, delete_endpoint, 
            confirm_dialog, 
            start_test, 
            stop_test])        
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

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct HttpServerRowResponse {
    pub id: String,
    pub name: String,
    pub http_port: Option<u16>,
    pub https_config: Option<HttpsConfigurationResponse>,
    pub endpoints: Vec<EndpointRowResponse>,
}

impl From<&apinae_lib::config::ServerConfiguration> for HttpServerRowResponse {
    fn from(http_server: &apinae_lib::config::ServerConfiguration) -> Self {
        Self {
            id: http_server.id.clone(),
            name: http_server.name.clone(),
            http_port: http_server.http_port,
            https_config: http_server.https_config.as_ref().map(|https_config| https_config.into()),
            endpoints: http_server.endpoints.iter().map(|endpoint| endpoint.into()).collect(),
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct EndpointRowResponse {
    pub id: String,
    pub endpoint: String,
    pub method: String,
    pub mock: Option<MockRowResponse>,
    pub route: Option<RouteRowResponse>,
}

impl From<&apinae_lib::config::EndpointConfiguration> for EndpointRowResponse {
    fn from(endpoint: &apinae_lib::config::EndpointConfiguration) -> Self {
        Self {
            id: endpoint.id.clone(),
            endpoint: endpoint.endpoint.clone(),
            method: endpoint.method.clone(),
            mock: endpoint.mock.as_ref().map(|mock| mock.into()),
            route: endpoint.route.as_ref().map(|route| route.into()),
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct MockRowResponse {
    pub response: Option<String>,
    pub status: u16,
    pub headers: HashMap<String, String>,
    pub delay: u64,
}

impl From<&apinae_lib::config::MockResponseConfiguration> for MockRowResponse {
    fn from(mock: &apinae_lib::config::MockResponseConfiguration) -> Self {
        Self {
            response: mock.response.clone(),
            status: mock.status,
            headers: mock.headers.clone(),
            delay: mock.delay,
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct RouteRowResponse {
    pub endpoint: String,
    pub proxy_url: Option<String>,
    pub verbose: bool,
    pub http1_only: bool,
    pub accept_invalid_certs: bool,
    pub accept_invalid_hostnames: bool,
    pub min_tls_version: Option<String>,
    pub max_tls_version: Option<String>,
    pub read_timeout: Option<u64>,
    pub connect_timeout: Option<u64>,
}

impl From<&apinae_lib::config::RouteConfiguration> for RouteRowResponse {
    fn from(route: &apinae_lib::config::RouteConfiguration) -> Self {        
        Self {
            endpoint: route.endpoint.clone(),
            proxy_url: route.proxy_url.clone(),
            verbose: route.verbose,
            http1_only: route.http1_only,
            accept_invalid_certs: route.accept_invalid_certs,
            accept_invalid_hostnames: route.accept_invalid_hostnames,
            min_tls_version: route.min_tls_version.clone().map(|value| String::from(value)),
            max_tls_version: route.max_tls_version.clone().map(|value| String::from(value)),
            read_timeout: route.read_timeout,
            connect_timeout: route.connect_timeout,
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct HttpsConfigurationResponse {
    pub server_certificate: String,
    pub private_key: String,
    pub https_port: u16,
    pub client_certificate: Option<String>,
    pub supported_tls_versions: Vec<String>,
}

impl From<&apinae_lib::config::HttpsConfiguration> for HttpsConfigurationResponse {
    fn from(https_config: &apinae_lib::config::HttpsConfiguration) -> Self {
        Self {
            server_certificate: https_config.server_certificate.clone(),
            private_key: https_config.private_key.clone(),
            https_port: https_config.https_port,
            client_certificate: https_config.client_certificate.clone(),
            supported_tls_versions: https_config.clone().supported_tls_versions.into_iter().map(|value| String::from(value)).collect(),
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct TcpListenerRowResponse {
    pub id: String,
    pub file: Option<String>,
    pub data: Option<String>,
    pub delay_write_ms: Option<u64>,
    pub port: u16,
    pub accept: bool,
    pub close_connection: String,
}

impl From<&apinae_lib::config::TcpListenerData> for TcpListenerRowResponse {
    fn from(tcp_listener: &apinae_lib::config::TcpListenerData) -> Self {
        Self {
            id: tcp_listener.id.clone(),
            file: tcp_listener.file.clone(),
            data: tcp_listener.data.clone(),
            delay_write_ms: tcp_listener.delay_write_ms,
            port: tcp_listener.port,
            accept: tcp_listener.accept,
            close_connection: String::from(tcp_listener.clone().close_connection)
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