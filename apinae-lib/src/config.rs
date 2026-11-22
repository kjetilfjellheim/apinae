use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    time::SystemTime,
};

use crate::error::ApplicationError;

/**
 * The configuration for the application.
 */
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AppConfiguration {
    // The name of the configuration.
    pub name: String,
    // The description of the configuration.
    pub description: String,
    // The collection configurations.
    pub collections: Vec<CollectionConfiguration>,
}

impl AppConfiguration {
    /**
     * Create a new configuration.
     *
     * `name` The name of the configuration.
     * `description` The description of the configuration.
     *
     * The configuration.
     */
    #[must_use]
    pub fn new(name: String, description: String) -> Self {
        AppConfiguration { name, description, collections: vec![] }
    }

    /**
     * Update the configuration.
     *
     * `name` The name of the configuration.
     * `description` The description of the configuration.
     */
    pub fn update(&mut self, name: String, description: String) {
        self.name = name;
        self.description = description;
    }

    /**
     * Add a collection to the configuration.
     *
     * `collection` The collection to add.
     */
    pub fn add_collection(&mut self, collection: CollectionConfiguration) {
        self.collections.push(collection);
    }

    /**
     * Get a mutable reference to a collection by its id.
     *
     * `collection_id` The id of the collection.
     *
     * The collection if found.
     */
    pub fn get_collection(&mut self, collection_id: &str) -> Option<&mut CollectionConfiguration> {
        self.collections.iter_mut().find(|collection| collection.id == collection_id)
    }

    /**
     * Get all collections in the configuration.
     *
     * The collections in the configuration.
     */
    pub fn get_collections(&self) -> &Vec<CollectionConfiguration> {
        &self.collections
    }

    /**
     * Delete a collection by its id.
     *
     * `collection_id` The id of the collection.
     *
     * Ok if the collection was deleted successfully.
     *
     * # Errors
     * An error if the collection could not be found.
     */
    pub fn delete_collection(&mut self, collection_id: &str) -> Result<(), ApplicationError> {
        let index =
            self.collections.iter().position(|collection| collection.id == collection_id).ok_or_else(|| ApplicationError::CouldNotFind(format!("Collection with id {collection_id} not found.")))?;
        self.collections.remove(index);
        Ok(())
    }

    /**
     * Save the configuration to a file.
     *
     * `path` The path to save the configuration to.
     *
     * Ok if the configuration was saved successfully.
     *
     * # Errors
     * An error if the configuration could not be saved.
     */
    pub fn save(&self, path: &str) -> Result<(), ApplicationError> {
        let string_data = serde_json::to_string_pretty(&self).map_err(|err| ApplicationError::FileError(format!("Failed to convert model to string: {err}")))?;
        std::fs::write(path, string_data).map_err(|err| ApplicationError::FileError(format!("Failed to write model to file: {err}")))?;
        Ok(())
    }

    /**
     * Load the configuration from a file.
     *
     * `path` The path to load the configuration from.
     *
     * The configuration.
     *
     * # Errors
     * An error if the configuration could not be loaded.
     */
    pub fn load(path: &str) -> Result<Self, ApplicationError> {
        let string_data = std::fs::read_to_string(path).map_err(|err| ApplicationError::FileError(format!("Failed to read input data to string: {err}")))?;
        let mut value: serde_json::Value = serde_json::from_str(&string_data).map_err(|err| ApplicationError::FileError(format!("Failed to convert input string to json: {err}")))?;

        // Backward compatibility: older files stored setups at the root level.
        if value.get("collections").is_none() {
            let legacy_setups = value.get("setups").cloned();
            if let (Some(serde_json::Value::Array(setups)), Some(object)) = (legacy_setups, value.as_object_mut()) {
                let collection_id = get_identifier()?;
                object.insert(
                    "collections".to_string(),
                    serde_json::Value::Array(vec![serde_json::json!({
                        "id": collection_id,
                        "name": "Default",
                        "description": "",
                        "setups": setups
                    })]),
                );
                object.remove("setups");
            }
        }

        serde_json::from_value(value).map_err(|err| ApplicationError::FileError(format!("Failed to convert input json to config model: {err}")))
    }
}

/**
 * Configuration collection.
 */
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CollectionConfiguration {
    // Collection id
    pub id: String,
    // Collection name.
    pub name: String,
    // Collection description.
    pub description: String,
    // Setups for this collection.
    pub setups: Vec<SetupConfiguration>,
}

impl CollectionConfiguration {
    /**
     * Create a new collection.
     *
     * `name` The name of the collection.
     * `description` The description of the collection.
     * `setups` The setups for the collection.
     */
    pub fn new(name: String, description: String) -> Result<Self, ApplicationError> {
        let id = get_identifier()?;
        Ok(CollectionConfiguration { id, name, description, setups: Vec::new() })
    }

    /**
     * Update the collection.
     *
     * `name` The name of the collection.
     * `description` The description of the collection.
     */
    pub fn update(&mut self, name: String, description: String) {
        self.name = name;
        self.description = description;
    }

    /**
     * Add a setup to the collection.
     *
     * `setup` The setup to add.
     */
    pub fn add_setup(&mut self, setup: SetupConfiguration) {
        self.setups.push(setup);
    }

    /**
     * Get all setups in the collection.
     *
     * The setups in the collection.
     */
    pub fn get_setups(&self) -> &Vec<SetupConfiguration> {
        &self.setups
    }

    /**
     * Get a mutable reference to a setup by its id.
     *
     * `setup_id` The id of the setup.
     *
     * The setup if found.
     */
    pub fn get_setup(&mut self, setup_id: &str) -> Option<&mut SetupConfiguration> {
        self.setups.iter_mut().find(|setup| setup.id == setup_id)
    }

    /**
     * Delete a setup by its id.
     *
     * `setup_id` The id of the setup.
     *
     * # Errors
     * An error if the setup could not be found.
     */
    pub fn delete_setup(&mut self, setup_id: &str) -> Result<(), ApplicationError> {
        let index = self.setups.iter().position(|setup| setup.id == setup_id).ok_or_else(|| ApplicationError::CouldNotFind(format!("Setup with id {setup_id} not found.")))?;
        self.setups.remove(index);
        Ok(())
    }
}

/**
 * Configuration for a test.
 */
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetupConfiguration {
    // The ID of the setup. This is a UUID automatically generated.
    pub id: String,
    // The name of the setup.
    pub name: String,
    // The description of the setup.
    pub description: String,
    // The server configurations.
    pub servers: Vec<ServerConfiguration>,
    // TCP listeners
    pub listeners: Vec<TcpListenerData>,
    // The parameters to pass to the setup.
    pub params: Option<HashSet<String>>,
    // Predefined sets of parameters for the setup.
    pub predefined_sets: Option<HashMap<String, HashMap<String, String>>>,
}

impl SetupConfiguration {
    /**
     * Create a new test configuration.
     *
     * `name` The name of the setup.
     * `description` The description of the setup.
     *
     * # Errors
     * An error if the identifier could not be generated.
     */
    pub fn new(name: String, description: String) -> Result<Self, ApplicationError> {
        let id = get_identifier()?;
        Ok(SetupConfiguration { id, name, description, servers: Vec::new(), listeners: Vec::new(), params: None, predefined_sets: None })
    }

    /**
     * Update the setup configuration.
     *
     * `name` The name of the setup.
     * `description` The description of the setup.
     */
    pub fn update(&mut self, name: String, description: String) {
        self.name = name;
        self.description = description;
    }

    /**
     * Get server by ID.
     *
     * `server_id` The ID of the server.
     */
    pub fn get_server(&mut self, server_id: &str) -> Option<&mut ServerConfiguration> {
        self.servers.iter_mut().find(|server| server.id == server_id)
    }

    /**
     * Delete server by id.
     *
     * `server_id` The id of the server.
     *
     * # Errors
     * An error if the setup could not be found.
     * An error if the server could not be found.
     */
    pub fn delete_server(&mut self, server_id: &str) -> Result<(), ApplicationError> {
        let index = self.servers.iter().position(|server| server.id == server_id).ok_or_else(|| ApplicationError::CouldNotFind(format!("Server with id {server_id} not found.")))?;
        self.servers.remove(index);
        Ok(())
    }

    /**
     * Delete a TCP listener by its id.
     *
     * `listener_id` The id of the listener.
     *
     * # Errors
     * An error if the listener could not be found.
     */
    pub fn delete_listener(&mut self, listener_id: &str) -> Result<(), ApplicationError> {
        let index = self.listeners.iter().position(|listener| listener.id == listener_id).ok_or_else(|| ApplicationError::CouldNotFind(format!("Listener with id {listener_id} not found.")))?;
        self.listeners.remove(index);
        Ok(())
    }

    /**
     * Add a TCP listener to the setup.
     *
     * `listener` The TCP listener to add.
     */
    pub fn add_listener(&mut self, listener: TcpListenerData) {
        self.listeners.push(listener);
    }

    /**
     * Add a server to the setup.
     *
     * `server` The server to add.
     */
    pub fn add_server(&mut self, server: ServerConfiguration) {
        self.servers.push(server);
    }

    /**
     * Get a mutable reference to a TCP listener by its id.
     *
     * `listener_id` The id of the listener.
     *
     * The listener if found.
     */
    pub fn get_listener(&mut self, listener_id: &str) -> Option<&mut TcpListenerData> {
        self.listeners.iter_mut().find(|listener| listener.id == listener_id)
    }

    /**
     * Add a parameter to the setup.
     *
     * `param` The parameter to add.
     */
    pub fn add_param(&mut self, param: String) {
        if let Some(params) = &mut self.params {
            params.insert(param.clone());
        } else {
            let mut params = HashSet::new();
            params.insert(param.clone());
            self.params = Some(params);
        }
    }

    /**
     * Delete a parameter from the setup.
     *
     * `param` The parameter to delete.
     */
    pub fn remove_param(&mut self, param: &str) {
        if let Some(params) = &mut self.params {
            params.remove(param);
            if params.is_empty() {
                self.params = None;
            }
        }
    }

    /**
     * Add a predefined set to the setup.
     *
     * `set_name` The name of the predefined set to add.
     */
    pub fn add_predefined_set(&mut self, set_name: String, params: Vec<String>) -> Result<(), ApplicationError> {
        if let Some(predefined_sets) = &mut self.predefined_sets {
            predefined_sets.insert(set_name.clone(), params.iter().map(|param| (param.clone(), String::new())).collect());
        } else {
            let mut predefined_sets = HashMap::new();
            predefined_sets.insert(set_name.clone(), params.iter().map(|param| (param.clone(), String::new())).collect());
            self.predefined_sets = Some(predefined_sets);
        }
        Ok(())
    }

    /**
     * Delete a predefined set from the setup.
     *
     * `set_name` The name of the predefined set to delete.
     */
    pub fn remove_predefined_set(&mut self, set_name: &str) {
        if let Some(predefined_sets) = &mut self.predefined_sets {
            predefined_sets.remove(set_name);
            if predefined_sets.is_empty() {
                self.predefined_sets = None;
            }
        }
    }

    /**
     * Add a parameter to a predefined set.
     *
     * `set_name` The name of the predefined set.
     * `values` The key-value pairs of parameters to update.
     *
     * # Errors
     * An error if the predefined set could not be found.
     */
    pub fn update_predefined_set(&mut self, set_name: &str, values: HashMap<String, String>) -> Result<(), ApplicationError> {
        if let Some(predefined_sets) = &mut self.predefined_sets {
            if let Some(_set) = predefined_sets.get_mut(set_name) {
                predefined_sets.insert(set_name.to_string(), values);
                return Ok(());
            }
        }
        Err(ApplicationError::CouldNotFind(format!("Predefined set with name {set_name} not found.")))
    }
}

/**
 * Configuration for an https server.
 */
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HttpsConfiguration {
    // The path to the certificate.
    pub server_certificate: String,
    // The path to the private key.
    pub private_key: String,
    // The https port
    pub https_port: u16,
    // The path to the client certificate.
    pub client_certificate: Option<String>,
    // The supported versions. Only 1.2 and 1.3 are supported.
    #[serde(default = "default_server_supported_tls_versions")]
    pub supported_tls_versions: Vec<TlsVersion>,
}

impl HttpsConfiguration {
    /**
     * Create a new https configuration.
     *
     * `certificate` The path to the certificate.
     * `private_key` The path to the private key.
     * `https_port` The https port.
     *
     * The https configuration.
     */
    #[must_use]
    pub fn new(server_certificate: String, private_key: String, https_port: u16, client_certificate: Option<String>, supported_tls_versions: Vec<TlsVersion>) -> Self {
        HttpsConfiguration { server_certificate, private_key, https_port, client_certificate, supported_tls_versions }
    }
}

/**
 * Configuration for a server.
 */
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ServerConfiguration {
    // The ID of the setup. This is a UUID automatically generated.
    pub id: String,
    // The name of the server.
    pub name: String,
    // The port to run the server on.
    pub http_port: Option<u16>,
    // The endpoints to configure.
    pub endpoints: Vec<EndpointConfiguration>,
    // The https configuration.
    pub https_config: Option<HttpsConfiguration>,
}

impl ServerConfiguration {
    /**
     * Create a new server configuration.
     *
     * `name` The name of the server.
     * `http_port` The port to run the server on.
     * `https_config` The https configuration.
     *
     * # Errors
     * An error if the identifier could not be generated.
     */
    pub fn new(name: String, http_port: Option<u16>, https_config: Option<HttpsConfiguration>) -> Result<Self, ApplicationError> {
        let id = get_identifier()?;
        Ok(ServerConfiguration { id, name, http_port, endpoints: Vec::new(), https_config })
    }

    /**
     * Update the server configuration.
     *
     * `name` The name of the server.
     * `http_port` The port to run the server on.
     */
    pub fn update(&mut self, name: String, http_port: Option<u16>, https_config: Option<HttpsConfiguration>) {
        self.name = name;
        self.http_port = http_port;
        self.https_config = https_config;
    }

    /**
     * Get endpoint by id.
     *
     * `endpoint_id` The id of the endpoint.
     */
    pub fn get_endpoint(&mut self, endpoint_id: &str) -> Option<&mut EndpointConfiguration> {
        self.endpoints.iter_mut().find(|endpoint| endpoint.id == endpoint_id)
    }

    /**
     * Add endpoint to server.
     *
     * `endpoint` The endpoint to add.
     */
    pub fn add_endpoint(&mut self, endpoint: EndpointConfiguration) {
        self.endpoints.push(endpoint);
    }

    /**
     * Delete endpoint by id.
     *
     * `endpoint_id` The id of the endpoint.
     *
     * # Errors
     * An error if the setup could not be found.
     * An error if the server could not be found.
     * An error if the endpoint could not be found.
     */
    pub fn delete_endpoint(&mut self, endpoint_id: &str) -> Result<(), ApplicationError> {
        let index = self.endpoints.iter().position(|endpoint| endpoint.id == endpoint_id).ok_or_else(|| ApplicationError::CouldNotFind(format!("Endpoint with id {endpoint_id} not found.")))?;
        self.endpoints.remove(index);
        Ok(())
    }
}

/**
 * Configuration for an endpoint.
 */
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EndpointConfiguration {
    // The ID of the setup. This is a UUID automatically generated.
    pub id: String,
    // Path expression for the apinae API. This is a regular expression.
    pub path_expression: Option<String>,
    // Body expression for the apinae API. This is a regular expression.
    pub body_expression: Option<String>,
    // The HTTP method.
    pub method: Option<String>,
    // Defines how the endpoint is to be handled.
    pub endpoint_type: Option<EndpointType>,
}

impl EndpointConfiguration {
    /**
     * Create a new endpoint configuration.
     *
     * `path_expression` Endpoint for the apinae API. This is a regular expression.
     * `body_expression` Body expression for the apinae API. This is a regular expression.
     * `method` The HTTP method.
     * `endpoint_type` Defines how the endpoint is to be handled.
     *
     * # Errors
     * An error if the identifier could not be generated.
     */
    pub fn new(path_expression: Option<String>, method: Option<String>, body_expression: Option<String>, endpoint_type: Option<EndpointType>) -> Result<Self, ApplicationError> {
        let id = get_identifier()?;
        Ok(EndpointConfiguration { id, path_expression, body_expression, method, endpoint_type })
    }

    /**
     * Update the endpoint configuration.
     *
     * endpoint_configuration The new endpoint configuration.
     */
    pub fn update(&mut self, endpoint_configuration: EndpointConfiguration) {
        self.path_expression = endpoint_configuration.path_expression;
        self.body_expression = endpoint_configuration.body_expression;
        self.method = endpoint_configuration.method;
        self.endpoint_type = endpoint_configuration.endpoint_type;
    }
}

/**
 * The type of the endpoint. Determines what type of handling to use.
 */
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub enum EndpointType {
    Mock { configuration: MockResponseConfiguration },
    Route { configuration: RouteConfiguration },
}

/**
 * When to close the connection.
 */
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum CloseConnectionWhen {
    BeforeRead,
    AfterRead,
    AfterResponse,
    Never,
}

impl From<CloseConnectionWhen> for String {
    /**
     * Convert a `CloseConnectionWhen` to a string.
     */
    fn from(close_connection: CloseConnectionWhen) -> Self {
        match close_connection {
            CloseConnectionWhen::BeforeRead => "BeforeRead".to_string(),
            CloseConnectionWhen::AfterRead => "AfterRead".to_string(),
            CloseConnectionWhen::AfterResponse => "AfterResponse".to_string(),
            CloseConnectionWhen::Never => "Never".to_string(),
        }
    }
}

impl From<&str> for CloseConnectionWhen {
    /**
     * Convert a string to a `CloseConnectionWhen`.
     */
    fn from(close_connection: &str) -> Self {
        match close_connection {
            "BeforeRead" => CloseConnectionWhen::BeforeRead,
            "AfterRead" => CloseConnectionWhen::AfterRead,
            "Never" => CloseConnectionWhen::Never,
            _ => CloseConnectionWhen::AfterResponse,
        }
    }
}

/**
 * Configuration for a tcp connection.
 */
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TcpListenerData {
    // The id of the listener.
    pub id: String,
    // The file to read from.
    pub file: Option<String>,
    // The data to return. If this is set, the file will be ignored.
    pub data: Option<String>,
    // Time to wait before writing the response.
    pub delay_write_ms: Option<u64>,
    // The port to listen on.
    pub port: u16,
    // Do accept connections. Default is true.
    #[serde(default = "default_as_true")]
    pub accept: bool,
    // When to close the connection. Default is AfterResponse.
    #[serde(default = "default_close_connection_when")]
    pub close_connection: CloseConnectionWhen,
}

impl TcpListenerData {
    /**
     * Create a new tcp configuration.
     *
     * The tcp configuration.
     * `file` The file to read from.
     * `data` The data to return. If this is set, the file will be ignored.
     * `delay_write_ms` Time to wait before writing the response.
     * `port` The port to listen on.
     * `accept` Do accept connections.
     * `close_connection` When to close the connection.
     *
     * # Errors
     * An error if the identifier could not be generated.
     */
    pub fn new(file: Option<String>, data: Option<String>, delay_write_ms: Option<u64>, port: u16, accept: bool, close_connection: CloseConnectionWhen) -> Result<Self, ApplicationError> {
        let id = get_identifier()?;
        Ok(TcpListenerData { id, file, data, delay_write_ms, port, accept, close_connection })
    }

    /**
     * Update the tcp configuration.
     *
     * `tcp_listener_data` The new TCP listener data.
     */
    pub fn update(&mut self, tcp_listener_data: TcpListenerData) {
        self.file = tcp_listener_data.file;
        self.data = tcp_listener_data.data;
        self.delay_write_ms = tcp_listener_data.delay_write_ms;
        self.port = tcp_listener_data.port;
        self.accept = tcp_listener_data.accept;
        self.close_connection = tcp_listener_data.close_connection;
    }
}

/**
 * Configuration for a mock response.
 */
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MockResponseConfiguration {
    // The response to return when the mock is called.
    pub response: Option<String>,
    // The status code to return when the mock is called.
    pub status: String,
    // The headers to return when the mock is called.
    pub headers: HashMap<String, String>,
    // Time to wait in milliseconds before returning the response.
    pub delay: u64,
}

impl MockResponseConfiguration {
    /**
     * Create a new mock response configuration.
     *
     * `response` The response to return when the mock is called.
     * `status` The status code to return when the mock is called.
     * `headers` The headers to return when the mock is called.
     * `delay` Time to wait before returning the response.
     *
     * The mock response configuration.
     */
    #[must_use]
    pub fn new(response: Option<String>, status: String, headers: HashMap<String, String>, delay: u64) -> Self {
        MockResponseConfiguration { response, status, headers, delay }
    }

    /**
     * Update the mock response configuration.
     *
     * `response` The response to return when the mock is called.
     * `status` The status code to return when the mock is called.
     * `headers` The headers to return when the mock is called.
     * `delay` Time to wait before returning the response.
     */
    pub fn update(&mut self, response: Option<String>, status: String, headers: HashMap<String, String>, delay: u64) {
        self.response = response;
        self.status = status;
        self.headers = headers;
        self.delay = delay;
    }
}

/**
 * The supported TLS versions.
 */
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum TlsVersion {
    TLSv1_0,
    TLSv1_1,
    TLSv1_2,
    TLSv1_3,
}

impl From<TlsVersion> for String {
    /**
     * Convert a `TlsVersion` to a string.
     */
    fn from(version: TlsVersion) -> Self {
        match version {
            TlsVersion::TLSv1_0 => "TLSv1.0".to_string(),
            TlsVersion::TLSv1_1 => "TLSv1.1".to_string(),
            TlsVersion::TLSv1_2 => "TLSv1.2".to_string(),
            TlsVersion::TLSv1_3 => "TLSv1.3".to_string(),
        }
    }
}

impl From<String> for TlsVersion {
    /**
     * Convert a string to a `TlsVersion`. Used when converting to the file model.
     */
    fn from(version: String) -> Self {
        let version = version.as_str();
        match version {
            "TLSv1.0" => TlsVersion::TLSv1_0,
            "TLSv1.1" => TlsVersion::TLSv1_1,
            "TLSv1.2" => TlsVersion::TLSv1_2,
            _ => TlsVersion::TLSv1_3,
        }
    }
}

/**
 * Configuration for a route.
 */
#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RouteConfiguration {
    // The URL of the endpoint.
    pub url: String,
    // The proxy url. Example: `<http://localhost:8080>`
    pub proxy_url: Option<String>,
    // The log file where the requests and responses are stored.
    //TODO: Unimplemented
    pub log: Option<String>,
    // HTTP/1 only
    #[serde(default)]
    pub http1_only: bool,
    // Accept invalid certificates
    #[serde(default)]
    pub accept_invalid_certs: bool,
    // Accept invalid hostnames
    #[serde(default)]
    pub accept_invalid_hostnames: bool,
    // Minimum TLS version
    pub min_tls_version: Option<TlsVersion>,
    // Maximum TLS version
    pub max_tls_version: Option<TlsVersion>,
    // Read timeout
    pub read_timeout: Option<u64>,
    // Connect timeout
    pub connect_timeout: Option<u64>,
    // Delay before request in milliseconds
    pub delay_before: Option<u64>,
    // Delay after request in milliseconds
    pub delay_after: Option<u64>,
}

impl RouteConfiguration {
    /**
     * Create a new route configuration.
     *
     * `url` The URL of the endpoint. Example `<http://localhost:8080>`
     * `proxy_url` Proxy to use.
     * `log` The log file where the requests and responses are stored.
     * `http1_only` HTTP/1 only
     * `accept_invalid_certs` Accept invalid certificates
     * `accept_invalid_hostnames` Accept invalid hostnames
     * `min_tls_version` Minimum TLS version
     * `max_tls_version` Maximum TLS version
     * `read_timeout` Read timeout
     * `connect_timeout` Connect timeout
     * `delay_before` Delay before request in milliseconds
     * `delay_after` Delay after request in milliseconds
     *
     */
    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::fn_params_excessive_bools)]
    #[must_use]
    pub fn new(
        url: String,
        proxy_url: Option<String>,
        log: Option<String>,
        http1_only: bool,
        accept_invalid_certs: bool,
        accept_invalid_hostnames: bool,
        min_tls_version: Option<TlsVersion>,
        max_tls_version: Option<TlsVersion>,
        read_timeout: Option<u64>,
        connect_timeout: Option<u64>,
        delay_before: Option<u64>,
        delay_after: Option<u64>,
    ) -> Self {
        RouteConfiguration {
            url,
            proxy_url,
            log,
            http1_only,
            accept_invalid_certs,
            accept_invalid_hostnames,
            min_tls_version,
            max_tls_version,
            read_timeout,
            connect_timeout,
            delay_before,
            delay_after,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn update(
        &mut self,
        url: String,
        proxy_url: Option<String>,
        log: Option<String>,
        http1_only: bool,
        accept_invalid_certs: bool,
        accept_invalid_hostnames: bool,
        min_tls_version: Option<TlsVersion>,
        max_tls_version: Option<TlsVersion>,
        read_timeout: Option<u64>,
        connect_timeout: Option<u64>,
        delay_before: Option<u64>,
        delay_after: Option<u64>,
    ) {
        self.url = url;
        self.proxy_url = proxy_url;
        self.log = log;
        self.http1_only = http1_only;
        self.accept_invalid_certs = accept_invalid_certs;
        self.accept_invalid_hostnames = accept_invalid_hostnames;
        self.min_tls_version = min_tls_version;
        self.max_tls_version = max_tls_version;
        self.read_timeout = read_timeout;
        self.connect_timeout = connect_timeout;
        self.delay_before = delay_before;
        self.delay_after = delay_after;
    }
}

/**
 * Get new identifier.
 *
 * The identifier.
 *
 * # Errors
 * An error if the identifier could not be generated.
 */
fn get_identifier() -> Result<String, ApplicationError> {
    let id = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).map_err(|err| ApplicationError::ConfigurationError(format!("Failed to generate identifier: {err}")))?;
    Ok(id.as_millis().to_string())
}

/**
 * Default server supported tls versions.
 */
fn default_server_supported_tls_versions() -> Vec<TlsVersion> {
    vec![TlsVersion::TLSv1_2, TlsVersion::TLSv1_3]
}

/**
 * Default as true.
 */
fn default_as_true() -> bool {
    true
}

/**
 * Default close connection when.
 */
fn default_close_connection_when() -> CloseConnectionWhen {
    CloseConnectionWhen::AfterResponse
}

#[cfg(test)]
mod test {
    use super::*;

    fn temp_json_path(name: &str) -> String {
        let nanos = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos();
        std::env::temp_dir().join(format!("apinae_{name}_{}_{}.json", std::process::id(), nanos)).to_string_lossy().to_string()
    }

    fn sample_endpoint() -> EndpointConfiguration {
        EndpointConfiguration::new(
            Some("^/test$".to_string()),
            Some("GET".to_string()),
            Some("body".to_string()),
            Some(EndpointType::Mock {
                configuration: MockResponseConfiguration::new(Some("response".to_string()), "200".to_string(), HashMap::from([("Content-Type".to_string(), "application/json".to_string())]), 10),
            }),
        )
        .unwrap()
    }

    #[test]
    fn app_configuration_methods() {
        let mut app = AppConfiguration::new("Original".to_string(), "Original description".to_string());
        assert_eq!(app.name, "Original");
        assert_eq!(app.description, "Original description");
        assert!(app.collections.is_empty());

        app.update("Updated".to_string(), "Updated description".to_string());
        assert_eq!(app.name, "Updated");
        assert_eq!(app.description, "Updated description");

        let collection = CollectionConfiguration::new("Collection".to_string(), "Collection description".to_string()).unwrap();
        let missing_id = "missing".to_string();
        let collection_id = collection.id.clone();
        app.add_collection(collection);
        assert_eq!(app.get_collections().len(), 1);
        assert!(app.get_collection(&collection_id).is_some());
        assert!(app.get_collection(&missing_id).is_none());

        assert!(app.delete_collection(&collection_id).is_ok());
        assert!(app.collections.is_empty());
        assert!(matches!(
            app.delete_collection(&collection_id),
            Err(ApplicationError::CouldNotFind(message)) if message.contains(&collection_id)
        ));
    }

    #[test]
    fn app_save_load_and_file_errors() {
        let mut app = AppConfiguration::new("Save test".to_string(), "Description".to_string());
        let mut collection = CollectionConfiguration::new("Collection".to_string(), "Description".to_string()).unwrap();
        collection.add_setup(SetupConfiguration::new("Setup".to_string(), "Description".to_string()).unwrap());
        app.add_collection(collection);

        let path = temp_json_path("save_load");
        app.save(&path).unwrap();
        assert_eq!(AppConfiguration::load(&path).unwrap(), app);
        let _ = std::fs::remove_file(&path);

        let missing_path = temp_json_path("missing");
        assert!(matches!(AppConfiguration::load(&missing_path), Err(ApplicationError::FileError(_))));

        let invalid_path = temp_json_path("invalid");
        std::fs::write(&invalid_path, "not json").unwrap();
        assert!(matches!(AppConfiguration::load(&invalid_path), Err(ApplicationError::FileError(_))));
        let _ = std::fs::remove_file(&invalid_path);

        let directory_path = temp_json_path("directory");
        std::fs::create_dir(&directory_path).unwrap();
        assert!(matches!(app.save(&directory_path), Err(ApplicationError::FileError(_))));
        let _ = std::fs::remove_dir(&directory_path);
    }

    #[test]
    fn app_load_legacy_root_setup() {
        let path = temp_json_path("legacy");
        std::fs::write(
            &path,
            r#"{
            "name": "Legacy",
            "description": "Legacy description",
            "setups": [{
                "id": "legacy-setup",
                "name": "Setup",
                "description": "Setup description",
                "servers": [],
                "listeners": [],
                "params": null,
                "predefinedSets": null
            }]
        }"#,
        )
        .unwrap();

        let loaded = AppConfiguration::load(&path).unwrap();
        assert_eq!(loaded.name, "Legacy");
        assert_eq!(loaded.collections.len(), 1);
        assert_eq!(loaded.collections[0].name, "Default");
        assert_eq!(loaded.collections[0].setups[0].id, "legacy-setup");
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn collection_configuration_methods() {
        let mut collection = CollectionConfiguration::new("Collection".to_string(), "Description".to_string()).unwrap();
        collection.update("Renamed".to_string(), "New description".to_string());
        assert_eq!(collection.name, "Renamed");
        assert_eq!(collection.description, "New description");

        let setup = SetupConfiguration::new("Setup".to_string(), "Description".to_string()).unwrap();
        let setup_id = setup.id.clone();
        collection.add_setup(setup);
        assert_eq!(collection.get_setups().len(), 1);
        assert!(collection.get_setup(&setup_id).is_some());
        assert!(collection.get_setup("missing").is_none());

        assert!(collection.delete_setup(&setup_id).is_ok());
        assert!(collection.get_setups().is_empty());
        assert!(matches!(collection.delete_setup(&setup_id), Err(ApplicationError::CouldNotFind(_))));
    }

    #[test]
    fn setup_configuration_server_listener_and_param_methods() {
        let mut setup = SetupConfiguration::new("Setup".to_string(), "Description".to_string()).unwrap();
        setup.update("Renamed".to_string(), "New description".to_string());
        assert_eq!(setup.name, "Renamed");
        assert_eq!(setup.description, "New description");

        let server = ServerConfiguration::new("Server".to_string(), Some(8080), None).unwrap();
        let server_id = server.id.clone();
        let listener = TcpListenerData::new(None, Some("data".to_string()), Some(5), 9000, true, CloseConnectionWhen::Never).unwrap();
        let listener_id = listener.id.clone();

        setup.add_server(server.clone());
        setup.add_listener(listener.clone());
        assert!(setup.get_server(&server_id).is_some());
        assert!(setup.get_listener(&listener_id).is_some());

        setup.add_param("host".to_string());
        setup.add_param("port".to_string());
        assert_eq!(setup.params.as_ref().unwrap().len(), 2);
        setup.remove_param("host");
        assert!(!setup.params.as_ref().unwrap().contains("host"));
        setup.remove_param("port");
        assert!(setup.params.is_none());

        assert!(setup.delete_server(&server_id).is_ok());
        assert!(matches!(setup.delete_server(&server_id), Err(ApplicationError::CouldNotFind(_))));
        assert!(setup.delete_listener(&listener_id).is_ok());
        assert!(matches!(setup.delete_listener(&listener_id), Err(ApplicationError::CouldNotFind(_))));
    }

    #[test]
    fn setup_predefined_set_methods() {
        let mut setup = SetupConfiguration::new("Setup".to_string(), "Description".to_string()).unwrap();
        setup.add_param("host".to_string());
        setup.add_param("port".to_string());
        setup.add_predefined_set("Production".to_string(), vec!["host".to_string(), "port".to_string()]).unwrap();

        let set = &setup.predefined_sets.as_ref().unwrap()["Production"];
        assert_eq!(set.get("host"), Some(&String::new()));
        assert_eq!(set.get("port"), Some(&String::new()));

        let values = HashMap::from([("host".to_string(), "example.com".to_string()), ("port".to_string(), "8080".to_string())]);
        setup.update_predefined_set("Production", values.clone()).unwrap();
        assert_eq!(setup.predefined_sets.as_ref().unwrap()["Production"], values);

        assert!(matches!(setup.update_predefined_set("Missing", HashMap::new()), Err(ApplicationError::CouldNotFind(_))));

        setup.remove_predefined_set("Production");
        assert!(setup.predefined_sets.is_none());
        setup.remove_predefined_set("Missing");
    }

    #[test]
    fn https_configuration_methods() {
        let https = HttpsConfiguration::new("server.crt".to_string(), "server.key".to_string(), 8443, Some("client.crt".to_string()), vec![TlsVersion::TLSv1_2, TlsVersion::TLSv1_3]);
        assert_eq!(https.server_certificate, "server.crt");
        assert_eq!(https.private_key, "server.key");
        assert_eq!(https.https_port, 8443);
        assert_eq!(https.client_certificate.as_deref(), Some("client.crt"));
        assert_eq!(https.supported_tls_versions, vec![TlsVersion::TLSv1_2, TlsVersion::TLSv1_3]);
    }

    #[test]
    fn server_configuration_methods() {
        let mut server = ServerConfiguration::new("Server".to_string(), Some(8080), None).unwrap();
        server.update("Renamed".to_string(), Some(8081), None);
        assert_eq!(server.name, "Renamed");
        assert_eq!(server.http_port, Some(8081));

        let endpoint = sample_endpoint();
        let endpoint_id = endpoint.id.clone();
        server.add_endpoint(endpoint);
        assert!(server.get_endpoint(&endpoint_id).is_some());
        assert!(server.get_endpoint("missing").is_none());
        assert!(server.delete_endpoint(&endpoint_id).is_ok());
        assert!(matches!(server.delete_endpoint(&endpoint_id), Err(ApplicationError::CouldNotFind(_))));
    }

    #[test]
    fn endpoint_configuration_methods() {
        let mut endpoint = sample_endpoint();
        assert_eq!(endpoint.path_expression.as_deref(), Some("^/test$"));
        let replacement = EndpointConfiguration::new(Some("^/updated$".to_string()), Some("POST".to_string()), Some("new body".to_string()), None).unwrap();
        endpoint.update(replacement);
        assert_eq!(endpoint.path_expression.as_deref(), Some("^/updated$"));
        assert_eq!(endpoint.method.as_deref(), Some("POST"));
        assert_eq!(endpoint.body_expression.as_deref(), Some("new body"));
        assert!(endpoint.endpoint_type.is_none());
    }

    #[test]
    fn tcp_listener_configuration_methods_and_close_string_conversion() {
        let mut listener = TcpListenerData::new(Some("input.txt".to_string()), Some("response".to_string()), Some(25), 9000, false, CloseConnectionWhen::AfterRead).unwrap();

        assert_eq!(listener.file.as_deref(), Some("input.txt"));
        assert_eq!(listener.data.as_deref(), Some("response"));
        assert_eq!(listener.delay_write_ms, Some(25));
        assert_eq!(listener.port, 9000);
        assert!(!listener.accept);

        listener.update(TcpListenerData::new(None, Some("updated".to_string()), Some(50), 9001, true, CloseConnectionWhen::AfterResponse).unwrap());
        assert_eq!(listener.file, None);
        assert_eq!(listener.data.as_deref(), Some("updated"));
        assert_eq!(listener.delay_write_ms, Some(50));
        assert_eq!(listener.port, 9001);
        assert!(listener.accept);

        assert_eq!(String::from(CloseConnectionWhen::BeforeRead), "BeforeRead");
        assert_eq!(String::from(CloseConnectionWhen::AfterRead), "AfterRead");
        assert_eq!(String::from(CloseConnectionWhen::AfterResponse), "AfterResponse");
        assert_eq!(String::from(CloseConnectionWhen::Never), "Never");
        assert_eq!(CloseConnectionWhen::from("BeforeRead"), CloseConnectionWhen::BeforeRead);
        assert_eq!(CloseConnectionWhen::from("AfterRead"), CloseConnectionWhen::AfterRead);
        assert_eq!(CloseConnectionWhen::from("AfterResponse"), CloseConnectionWhen::AfterResponse);
        assert_eq!(CloseConnectionWhen::from("Never"), CloseConnectionWhen::Never);
        assert_eq!(CloseConnectionWhen::from("Unknown"), CloseConnectionWhen::AfterResponse);
    }

    #[test]
    fn mock_response_configuration_methods() {
        let mut mock = MockResponseConfiguration::new(Some("old".to_string()), "200".to_string(), HashMap::from([("A".to_string(), "1".to_string())]), 10);
        assert_eq!(mock.response.as_deref(), Some("old"));
        assert_eq!(mock.status, "200");
        assert_eq!(mock.delay, 10);

        mock.update(Some("new".to_string()), "201".to_string(), HashMap::from([("B".to_string(), "2".to_string())]), 20);
        assert_eq!(mock.response.as_deref(), Some("new"));
        assert_eq!(mock.status, "201");
        assert_eq!(mock.headers, HashMap::from([("B".to_string(), "2".to_string())]));
        assert_eq!(mock.delay, 20);
    }

    #[test]
    fn tls_version_conversions() {
        assert_eq!(String::from(TlsVersion::TLSv1_0), "TLSv1.0");
        assert_eq!(String::from(TlsVersion::TLSv1_1), "TLSv1.1");
        assert_eq!(String::from(TlsVersion::TLSv1_2), "TLSv1.2");
        assert_eq!(String::from(TlsVersion::TLSv1_3), "TLSv1.3");

        assert_eq!(TlsVersion::from("TLSv1.0".to_string()), TlsVersion::TLSv1_0);
        assert_eq!(TlsVersion::from("TLSv1.1".to_string()), TlsVersion::TLSv1_1);
        assert_eq!(TlsVersion::from("TLSv1.2".to_string()), TlsVersion::TLSv1_2);
        assert_eq!(TlsVersion::from("TLSv1.3".to_string()), TlsVersion::TLSv1_3);
        assert_eq!(TlsVersion::from("Unknown".to_string()), TlsVersion::TLSv1_3);
    }

    #[test]
    fn route_configuration_methods() {
        let mut route = RouteConfiguration::new(
            "https://example.com".to_string(),
            Some("http://proxy".to_string()),
            Some("route.log".to_string()),
            true,
            true,
            true,
            Some(TlsVersion::TLSv1_2),
            Some(TlsVersion::TLSv1_3),
            Some(100),
            Some(200),
            Some(300),
            Some(400),
        );
        assert_eq!(route.url, "https://example.com");

        route.update("https://updated.example.com".to_string(), None, None, false, false, false, Some(TlsVersion::TLSv1_3), None, Some(500), Some(600), Some(700), Some(800));
        assert_eq!(route.url, "https://updated.example.com");
        assert_eq!(route.proxy_url, None);
        assert!(!route.http1_only);
        assert!(!route.accept_invalid_certs);
        assert!(!route.accept_invalid_hostnames);
        assert_eq!(route.max_tls_version, None);
        assert_eq!(route.read_timeout, Some(500));
        assert_eq!(route.connect_timeout, Some(600));
        assert_eq!(route.delay_before, Some(700));
        assert_eq!(route.delay_after, Some(800));
    }

    #[test]
    fn default_serde_helpers() {
        assert_eq!(default_server_supported_tls_versions(), vec![TlsVersion::TLSv1_2, TlsVersion::TLSv1_3]);
        assert!(default_as_true());
        assert_eq!(default_close_connection_when(), CloseConnectionWhen::AfterResponse);
        assert!(get_identifier().is_ok());
    }
}
