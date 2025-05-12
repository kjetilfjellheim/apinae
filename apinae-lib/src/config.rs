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
    // the setup configurations.
    pub setups: Vec<SetupConfiguration>,
}

impl AppConfiguration {
    /**
     * Create a new configuration.
     *
     * `name` The name of the configuration.
     * `description` The description of the configuration.
     * `setups` The setup configurations.
     *
     * The configuration.
     */
    #[must_use]
    pub fn new(name: String, description: String, setups: Vec<SetupConfiguration>) -> Self {
        AppConfiguration { name, description, setups }
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
     * Update test.
     *
     * `setup_id` The id of the setup.
     * `name` The name of the setup.
     * `description` The description of the setup.
     * `params` The parameters to pass to the setup.
     *
     * # Errors
     * An error if the setup could not be found.
     */
    pub fn update_setup(&mut self, setup_id: &str, name: &str, description: &str, params: Option<HashSet<String>>) -> Result<(), ApplicationError> {
        let test = self.get_setup(setup_id).ok_or_else(|| ApplicationError::CouldNotFind(format!("Test with id {setup_id} not found.")))?;
        test.name = name.to_string();
        test.description = description.to_string();
        test.params = params.clone();
        test.update_predefined_params(params);
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
        serde_json::from_str(&string_data).map_err(|err| ApplicationError::FileError(format!("Failed to convert input string to config model: {err}")))
    }

    /**
     * Get a setup by its ID.
     *
     * `id` The ID of the setup.
     *
     * The setup configuration.
     */
    #[must_use]
    pub fn get_setup(&mut self, setup_id: &str) -> Option<&mut SetupConfiguration> {
        self.setups.iter_mut().find(|test| test.id == setup_id)
    }

    /**
     * Delete setup by id.
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

    /**
     * Get a new listener.
     *
     * `setup_id` The id of the setup.
     * `listener_id` The id of the listener.
     *
     * The listener configuration.
     */
    pub fn get_listener(&mut self, setup_id: &str, listener_id: &str) -> Option<TcpListenerData> {
        self.get_setup(setup_id).and_then(|setup| setup.listeners.iter_mut().find(|listener| listener.id == listener_id).cloned())
    }

    /**
     * Delete listener by id.
     *
     * `setup_id` The id of the setup.
     * `listener_id` The id of the listener.
     *
     * # Errors
     * An error if the setup could not be found.
     * An error if the listener could not be found.
     *
     */
    pub fn delete_listener(&mut self, setup_id: &str, listener_id: &str) -> Result<(), ApplicationError> {
        let setup: &mut SetupConfiguration = self.get_setup(setup_id).ok_or(ApplicationError::CouldNotFind(format!("Setup with id {setup_id} not found.")))?;
        let index = setup.listeners.iter().position(|listener| listener.id == listener_id).ok_or(ApplicationError::CouldNotFind(format!("Listener with id {listener_id} not found.")))?;
        setup.listeners.remove(index);
        Ok(())
    }

    /**
     * Update the listener configuration.
     *
     * `setup_id` The id of the setup.
     * `listener_id` The id of the listener.
     * `file` The file to read from.
     * `data` The data to return. If this is set, the file will be ignored.
     * `delay_write_ms` Time to wait before writing the response.
     * `port` The port to listen on.
     * `accept` Do accept connections.
     * `close_connection` When to close the connection.
     *
     * # Errors
     * An error if the listener could not be found.
     */
    #[allow(clippy::too_many_arguments)]
    pub fn update_listener(
        &mut self,
        setup_id: &str,
        listener_id: &str,
        file: Option<String>,
        data: Option<String>,
        delay_write_ms: Option<u64>,
        port: u16,
        accept: bool,
        close_connection: CloseConnectionWhen,
    ) -> Result<(), ApplicationError> {
        let listener = self
            .get_setup(setup_id)
            .and_then(|test| test.listeners.iter_mut().find(|listener| listener.id == listener_id))
            .ok_or(ApplicationError::CouldNotFind(format!("Listener with id {listener_id} not found.")))?;
        listener.file = file;
        listener.data = data;
        listener.delay_write_ms = delay_write_ms;
        listener.port = port;
        listener.accept = accept;
        listener.close_connection = close_connection;
        Ok(())
    }

    /**
     * Get a server by its ID.
     *
     * `setup_id` The ID of the setup.
     * `server_id` The ID of the server.
     *
     * The server configuration.
     */
    pub fn get_server(&mut self, setup_id: &str, server_id: &str) -> Option<&mut ServerConfiguration> {
        self.get_setup(setup_id).and_then(|test| test.get_server(server_id))
    }

    /**
     * Delete server by id.
     *
     * `setup_id` The id of the setup.
     * `server_id` The id of the server.
     *
     * # Errors
     * An error if the setup could not be found.
     */
    pub fn delete_server(&mut self, setup_id: &str, server_id: &str) -> Result<(), ApplicationError> {
        let test: &mut SetupConfiguration = self.get_setup(setup_id).ok_or(ApplicationError::CouldNotFind(format!("Test with id {setup_id} not found.")))?;
        test.delete_server(server_id)
    }

    /**
     * Get an endpoint by its ID.
     *
     * `setup_id` The ID of the setup.
     * `server_id` The ID of the server.
     * `endpoint_id` The ID of the endpoint.
     *
     * The endpoint configuration.
     */
    pub fn get_endpoint(&mut self, setup_id: &str, server_id: &str, endpoint_id: &str) -> Option<&mut EndpointConfiguration> {
        self.get_server(setup_id, server_id).and_then(|server| server.endpoints.iter_mut().find(|endpoint| endpoint.id == endpoint_id))
    }

    /**
     * Delete endpoint by id.
     *
     * `setup_id` The id of the setup.
     * `server_id` The id of the server.
     * `endpoint_id` The id of the endpoint.
     *
     * # Errors
     * An error if the setup could not be found.
     * An error if the server could not be found.
     * An error if the endpoint could not be found.
     */
    pub fn delete_endpoint(&mut self, setup_id: &str, server_id: &str, endpoint_id: &str) -> Result<(), ApplicationError> {
        let server: &mut ServerConfiguration = self.get_server(setup_id, server_id).ok_or(ApplicationError::CouldNotFind(format!("Server with id {server_id} not found.")))?;
        server.delete_endpoint(endpoint_id)
    }

    /**
     * Update the server configuration.
     *
     * `setup_id` The id of the setup.
     * `server_id` The id of the server.
     * `endpoint_id` The id of the endpoint.
     * `path_expression` The path expression for the apinae API. This is a regular expression.
     * `method` The HTTP method.
     * `mock_response` The mock response.
     * `route` The route configuration.
     *
     * # Errors
     * An error if the setup could not be found.
     * An error if the server could not be found.
     * An error if the endpoint could not be found.
     */
    #[allow(clippy::too_many_arguments)]
    pub fn update_endpoint(
        &mut self,
        setup_id: &str,
        server_id: &str,
        endpoint_id: &str,
        path_expression: Option<String>,
        body_expression: Option<String>,
        method: Option<String>,
        endpoint_type: Option<EndpointType>,
    ) -> Result<(), ApplicationError> {
        let endpoint = self.get_endpoint(setup_id, server_id, endpoint_id).ok_or_else(|| ApplicationError::CouldNotFind(format!("Endpoint with id {endpoint_id} not found.")))?;
        endpoint.path_expression = path_expression;
        endpoint.method = method;
        endpoint.endpoint_type = endpoint_type;
        endpoint.body_expression = body_expression;
        Ok(())
    }

    /**
     * Add parameter to the setup.
     *
     * `setup_id` The id of the setup.
     * `param` The parameter to add.
     *
     * # Errors
     * An error if the setup could not be found.
     */
    pub fn add_param(&mut self, setup_id: &str, param: String) -> Result<(), ApplicationError> {
        let test = self.get_setup(setup_id).ok_or_else(|| ApplicationError::CouldNotFind(format!("Test with id {setup_id} not found.")))?;
        test.add_param(param);
        Ok(())
    }

    /**
     * Remove parameter from the setup.
     *
     * `setup_id` The id of the setup.
     * `param` The parameter to remove.
     *
     * # Errors
     * An error if the setup could not be found.
     */
    pub fn remove_param(&mut self, setup_id: &str, param: &str) -> Result<(), ApplicationError> {
        let test = self.get_setup(setup_id).ok_or_else(|| ApplicationError::CouldNotFind(format!("Test with id {setup_id} not found.")))?;
        test.remove_param(param);
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
    // Predefined sets of parameters.
    pub predefined_params: Option<Vec<PredefinedSet>>,
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
    pub fn new(name: String, description: String, servers: Vec<ServerConfiguration>, listeners: Vec<TcpListenerData>, params: Option<HashSet<String>>, predefined_params: Option<Vec<PredefinedSet>>) -> Result<Self, ApplicationError> {
        let id = get_identifier()?;
        Ok(SetupConfiguration { id, name, description, servers, listeners, params, predefined_params })
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
        if let Some(predefined_params) = &mut self.predefined_params {
            predefined_params.iter_mut().for_each(|f| {
                f.values.insert(param.clone(), String::new());
            });                        
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
        if let Some(predefined_params) = &mut self.predefined_params {
            predefined_params.iter_mut().for_each(|f| {
                f.values.remove(param);
            });                        
        }
    }

    /**
     * Update predefined parameters.
     *
     * `params` The parameters to update.
     */
    pub fn update_predefined_params(&mut self, params: Option<HashSet<String>>) {
        if let Some(predefined_params) = &mut self.predefined_params {
            predefined_params.iter_mut().for_each(|f| {
                if let Some(params) = &params {
                    for param in params.iter() {
                       if !f.values.contains_key(param) {
                            f.values.insert(param.clone(), String::new());
                        } 
                    }
                    let values = &mut f.values;
                    for param in values.clone().keys() {
                        if !params.contains(param) {
                            values.remove(param);
                        }
                    }
                }
            });
        }
    }

    /**
     * Add a predefined parameter set.
     *
     * # Errors
     * An error if the predefined set could not be created.
     */
    pub fn add_new_predefined_param_set(&mut self) -> Result<(), ApplicationError> {
        let values = if let Some(params) = &self.params {
            params.iter().map(|param| (param.clone(), String::new())).collect::<HashMap<String, String>>()
        } else {
            HashMap::<String, String>::new()
        };        
        let predefined_set = PredefinedSet::new(values)?;
        if let Some(predefined_params) = &mut self.predefined_params {
            predefined_params.push(predefined_set);
        } else {
            let predefined_params = vec![predefined_set];
            self.predefined_params = Some(predefined_params);
        }
        Ok(())
    }

    /**
     * Delete a predefined parameter set.
     *
     * `name` The name to delete
     *
     * # Errors
     * An error if the predefined set could not be found.
     */
    pub fn delete_predefined_param_set(&mut self, name: &str) -> Result<(), ApplicationError> {
        if let Some(predefined_params) = &mut self.predefined_params {
            let index = predefined_params.iter().position(|f| f.name == name).ok_or_else(|| ApplicationError::CouldNotFind(format!("Predefined set with name {name} not found.")))?;
            predefined_params.remove(index);
        }
        Ok(())
    }

    /**
     * Update a predefined parameter set.
     *
     * `old_name` The name of the predefined set to update.
     * `new_name` The new name of the predefined set.
     * `values` The values of the predefined set.
     *
     * # Errors
     * An error if the predefined set could not be found.
     */
    pub fn update_predefined_param_set(&mut self, old_name: &str, new_name: &str, values: HashMap<String, String>) -> Result<(), ApplicationError> {
        if let Some(predefined_params) = &mut self.predefined_params {
            let index = predefined_params.iter().position(|f| f.name == old_name).ok_or_else(|| ApplicationError::CouldNotFind(format!("Predefined set with name {old_name} not found.")))?;
            let mut predefined_set = predefined_params.get(index).unwrap().clone();
            predefined_set.name = new_name.to_string();
            predefined_set.values = values;
            predefined_params.remove(index);
            predefined_params.push(predefined_set);
        }
        Ok(())
    }
}

/**
 * Predefined parameters.
 */
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PredefinedSet {
    // Name of the predefined set.
    pub name: String,
    // The values of the predefined set.
    pub values: HashMap<String, String>,
}

impl PredefinedSet {

    /**
     * Create a new predefined set.
     *
     * `name` The name of the predefined set.
     * `values` The values of the predefined set.
     *
     * # Returns
     * The predefined set.
     * 
     * # Errors
     * An error if the identifier could not be generated.
     */
    pub fn new(values: HashMap<String, String>) -> Result<Self, ApplicationError> {
        let mut name = String::from("Untitled_");
        let identifier = get_identifier()?;
        name.push_str(identifier.as_str());
        Ok(PredefinedSet { name, values })
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
     * `port` The port to run the server on.
     * `endpoints` The endpoints to configure.
     *
     * # Errors
     * An error if the identifier could not be generated.
     */
    pub fn new(name: String, http_port: Option<u16>, endpoints: Vec<EndpointConfiguration>, https_config: Option<HttpsConfiguration>) -> Result<Self, ApplicationError> {
        let id = get_identifier()?;
        Ok(ServerConfiguration { id, name, http_port, endpoints, https_config })
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

    use std::vec;

    use super::*;

    /**
     * Test creating a configuration.
     */
    #[test]
    fn test_configuration() {
        let configuration = AppConfiguration::new(
            "Test Configuration".to_string(),
            "Test Configuration Description".to_string(),
            vec![SetupConfiguration::new(
                "Test".to_string(),
                "Test Description".to_string(),
                vec![ServerConfiguration::new(
                    "Server".to_string(),
                    Some(8080),
                    vec![EndpointConfiguration::new(
                        Some("/test".to_string()),
                        Some("GET".to_string()),
                        Some("Body".to_string()),
                        Some(EndpointType::Route { configuration: RouteConfiguration::new("/test".to_string(), None, None, false, false, false, None, None, None, None, Some(10), Some(100)) }),
                    )
                    .unwrap()],
                    None,                    
                )
                .unwrap()],
                Vec::new(),
                None,
                None,
            )
            .unwrap()],
        );

        assert_eq!(configuration.name, "Test Configuration");
        assert_eq!(configuration.description, "Test Configuration Description");
        assert_eq!(configuration.setups.len(), 1);
        assert_eq!(configuration.setups[0].name, "Test");
        assert_eq!(configuration.setups[0].description, "Test Description");
        assert_eq!(configuration.setups[0].servers.len(), 1);
        assert_eq!(configuration.setups[0].servers[0].name, "Server");
        assert_eq!(configuration.setups[0].servers[0].http_port, Some(8080));
        assert_eq!(configuration.setups[0].servers[0].endpoints.len(), 1);
        assert_eq!(configuration.setups[0].servers[0].endpoints[0].path_expression, Some("/test".to_owned()));
        assert_eq!(configuration.setups[0].servers[0].endpoints[0].method, Some("GET".to_owned()));
        assert_eq!(configuration.setups[0].servers[0].endpoints[0].body_expression, Some("Body".to_owned()));
        assert_eq!(
            configuration.setups[0].servers[0].endpoints[0].endpoint_type,
            Some(EndpointType::Route { configuration: RouteConfiguration::new("/test".to_string(), None, None, false, false, false, None, None, None, None, Some(10), Some(100)) })
        );
    }

    /**
     * Test serializing and deserializing the configuration.
     */
    #[test]
    fn test_serialize_deserialize() {
        let configuration = AppConfiguration::new(
            "Test Configuration".to_string(),
            "Test Configuration Description".to_string(),
            vec![SetupConfiguration::new(
                "Test".to_string(),
                "Test Description".to_string(),
                vec![ServerConfiguration::new(
                    "Server".to_string(),
                    Some(8080),
                    vec![EndpointConfiguration::new(
                        Some("/test".to_string()),
                        Some("".to_string()),
                        Some("GET".to_string()),
                        Some(EndpointType::Mock { configuration: MockResponseConfiguration::new(Some("Test Response".to_string()), String::from("200"), HashMap::new(), 0) }),
                    )
                    .unwrap()],
                    None,
                )
                .unwrap()],
                Vec::new(),
                None,
                None,
            )
            .unwrap()],
        );

        let serialized = serde_json::to_string(&configuration).unwrap();
        let deserialized: AppConfiguration = serde_json::from_str(&serialized).unwrap();

        assert_eq!(configuration, deserialized);
    }

    #[test]
    fn test_save_load() {
        let configuration = AppConfiguration::new(
            "Setup Configuration".to_string(),
            "Setup Configuration Description".to_string(),
            vec![SetupConfiguration::new(
                "Test".to_string(),
                "Test Description".to_string(),
                vec![ServerConfiguration::new(
                    "Server".to_string(),
                    Some(8080),
                    vec![EndpointConfiguration::new(
                        Some("/test".to_string()),
                        Some("".to_string()),
                        Some("GET".to_string()),
                        Some(EndpointType::Mock { configuration: MockResponseConfiguration::new(Some("Test Response".to_string()), String::from("200"), HashMap::new(), 0) }),
                    )
                    .unwrap()],
                    None,
                )
                .unwrap()],
                Vec::new(),
                None,
                None,
            )
            .unwrap()],
        );

        let path = "/tmp/test.json";
        let _ = configuration.save(path);
        let loaded = AppConfiguration::load(path).unwrap();

        assert_eq!(configuration, loaded);
    }
}
