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
        AppConfiguration {
            name,
            description,
            collections: vec![],
        }
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
        let index = self.collections.iter().position(|collection| collection.id == collection_id).ok_or_else(|| ApplicationError::CouldNotFind(format!("Collection with id {collection_id} not found.")))?;
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
        let mut value: serde_json::Value =
            serde_json::from_str(&string_data).map_err(|err| ApplicationError::FileError(format!("Failed to convert input string to json: {err}")))?;

        // Backward compatibility: older files stored setups at the root level.
        if value.get("collections").is_none() {
            let legacy_setups = value.get("setups").cloned();
            if let (Some(serde_json::Value::Array(setups)), Some(object)) = (legacy_setups, value.as_object_mut()) {
                object.insert(
                    "collections".to_string(),
                    serde_json::Value::Array(vec![serde_json::json!({
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
    #[must_use]
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
}

impl SetupConfiguration {
    /**
     * Create a new test configuration.
     *
     * `name` The name of the setup.
     * `description` The description of the setup.
     * `servers` The server configurations.
     * `listeners` The TCP listeners.
     * `params` The parameters to pass to the setup.
     * `predefined_params` The predefined sets of parameters.
     *
     * # Errors
     * An error if the identifier could not be generated.
     */
    pub fn new(name: String, description: String) -> Result<Self, ApplicationError> {
        let id = get_identifier()?;
        Ok(SetupConfiguration { id, name, description, servers: Vec::new(), listeners: Vec::new(), params: None })
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
     * Add a server to the setup.
     *
     * `server` The server to add.
     */
    pub fn add_server(&mut self, server: ServerConfiguration) {
        self.servers.push(server);
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
     * `path_expression` Endpoint for the apinae API. This is a regular expression.
     * `body_expression` Body expression for the apinae API. This is a regular expression.
     * `method` The HTTP method.
     * `endpoint_type` Defines how the endpoint is to be handled.
     */
    fn update(&mut self, path_expression: Option<String>, method: Option<String>, body_expression: Option<String>, endpoint_type: Option<EndpointType>) {
        self.path_expression = path_expression;
        self.body_expression = body_expression;
        self.method = method;
        self.endpoint_type = endpoint_type;
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
     * `file` The file to read from.
     * `data` The data to return. If this is set, the file will be ignored.
     * `delay_write_ms` Time to wait before writing the response.
     * `port` The port to listen on.
     * `accept` Do accept connections.
     * `close_connection` When to close the connection.
     */
    pub fn update(&mut self, file: Option<String>, data: Option<String>, delay_write_ms: Option<u64>, port: u16, accept: bool, close_connection: CloseConnectionWhen) {
        self.file = file;
        self.data = data;
        self.delay_write_ms = delay_write_ms;
        self.port = port;
        self.accept = accept;
        self.close_connection = close_connection;
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
        RouteConfiguration { url, proxy_url, log, http1_only, accept_invalid_certs, accept_invalid_hostnames, min_tls_version, max_tls_version, read_timeout, connect_timeout, delay_before, delay_after }
    }

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

    #[test]
    fn test_save_load() {
        let endpoint = EndpointConfiguration::new(
            Some("/test".to_string()),
            Some("GET".to_string()),
            Some("Body".to_string()),
            Some(EndpointType::Route { configuration: RouteConfiguration::new("/test".to_string(), None, None, false, false, false, None, None, None, None, Some(10), Some(100)) }),
        )
        .unwrap();
        let mut server = ServerConfiguration::new(
            "Test Server".to_string(),
            Some(8080),
            None,
        ).unwrap();
        server.add_endpoint(endpoint);
        let mut setup = SetupConfiguration::new(
            "Test".to_string(),
            "Test Description".to_string()).unwrap();
        setup.add_server(server);
        let mut collection = CollectionConfiguration::new(
            "Test Collection".to_string(),
            "Test Collection Description".to_string()).unwrap();
        collection.add_setup(setup);
        let mut configuration = AppConfiguration::new(
            "Setup Configuration".to_string(),
            "Setup Configuration Description".to_string()
        );
        configuration.add_collection(collection);
        let path = "/tmp/test.json";
        let _ = configuration.save(path);
        let loaded = AppConfiguration::load(path).unwrap();

        assert_eq!(configuration, loaded);
    }

}
