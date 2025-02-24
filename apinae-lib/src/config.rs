use serde::{Deserialize, Serialize};
/**
 * The configuration for the application. It contains all data that needs to be stored for the application.
 */
use std::{collections::HashMap, time::SystemTime};

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
    // The test configurations.
    pub tests: Vec<TestConfiguration>,
}

impl AppConfiguration {
    /**
     * Create a new configuration.
     *
     * `name` The name of the configuration.
     * `description` The description of the configuration.
     * `tests` The test configurations.
     *
     * The configuration.
     */
    #[must_use]
    pub fn new(name: String, description: String, tests: Vec<TestConfiguration>) -> Self {
        AppConfiguration {
            name,
            description,
            tests,
        }
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
        let string_data = serde_json::to_string_pretty(&self).map_err(|err| {
            ApplicationError::FileError(format!("Failed to convert model to string: {err}"))
        })?;
        std::fs::write(path, string_data).map_err(|err| {
            ApplicationError::FileError(format!("Failed to write model to file: {err}"))
        })?;
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
        let string_data = std::fs::read_to_string(path).map_err(|err| {
            ApplicationError::FileError(format!("Failed to read input data to string: {err}"))
        })?;
        serde_json::from_str(&string_data).map_err(|err| {
            ApplicationError::FileError(format!(
                "Failed to convert input string to config model: {err}"
            ))
        })
    }

    /**
     * Get a test by its ID.
     *
     * `id` The ID of the test.
     *
     * The test configuration.
     */
    #[must_use]
    pub fn get_test(&self, id: &str) -> Option<TestConfiguration> {
        self.tests.iter().find(|test| test.id == id).cloned()
    }
}

/**
 * Configuration for a test.
 */
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TestConfiguration {
    // The ID of the test. This is a UUID automatically generated.
    pub id: String,
    // The name of the test.
    pub name: String,
    // The description of the test.
    pub description: String,
    // The server configurations.
    pub servers: Vec<ServerConfiguration>,
    // TCP listeners
    pub listeners: Vec<TcpListenerData>
}

impl TestConfiguration {
    /**
     * Create a new test configuration.
     *
     * `name` The name of the test.
     * `description` The description of the test.
     * `servers` The server configurations.
     *
     * The test configuration.
     */
    #[must_use]
    pub fn new(name: String, description: String, servers: Vec<ServerConfiguration>, listeners: Vec<TcpListenerData>) -> Result<Self, ApplicationError> {
        let id = get_identifier()?;
        Ok(TestConfiguration {
            id,
            name,
            description,
            servers,
            listeners,
        })
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
    pub fn new(
        server_certificate: String,
        private_key: String,
        https_port: u16,
        client_certificate: Option<String>,
        supported_tls_versions: Vec<TlsVersion>,
    ) -> Self {
        HttpsConfiguration {
            server_certificate,
            private_key,
            https_port,
            client_certificate,
            supported_tls_versions,
        }
    }
}

/**
 * Configuration for a server.
 */
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ServerConfiguration {
    // The ID of the test. This is a UUID automatically generated.
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
     * The server configuration.
     */
    #[must_use]
    pub fn new(
        name: String,
        http_port: Option<u16>,
        endpoints: Vec<EndpointConfiguration>,
        https_config: Option<HttpsConfiguration>,
    ) -> Result<Self, ApplicationError> {
        let id = get_identifier()?;
        Ok(ServerConfiguration {
            id,
            name,
            http_port,
            endpoints,
            https_config,
        })
    }
}

/**
 * Configuration for an endpoint.
 */
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EndpointConfiguration {
    // The ID of the test. This is a UUID automatically generated.
    pub id: String,
    // Endpoint for the apinae API. This is a regular expression.
    pub endpoint: String,
    // The HTTP method.
    pub method: String,
    // The mock response.
    pub mock: Option<MockResponseConfiguration>,
    // The route configuration.
    pub route: Option<RouteConfiguration>,
}

impl EndpointConfiguration {
    /**
     * Create a new endpoint configuration.
     *
     * `endpoint` Endpoint for the apinae API. This is a regular expression.
     * `method` The HTTP method.
     * `mock_response` The mock response.
     * `route` The route configuration.
     *
     * The endpoint configuration.
     */
    #[must_use]
    pub fn new(
        endpoint: String,
        method: String,
        mock: Option<MockResponseConfiguration>,
        route: Option<RouteConfiguration>    
    ) -> Result<Self, ApplicationError> {
        let id = get_identifier()?;

        Ok(EndpointConfiguration {
            id,
            endpoint,
            method,
            mock,
            route,
        })
    }
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

/**
 * Configuration for a tcp connection.
 */
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TcpListenerData {
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
     */
    #[must_use]
    pub fn new(file: Option<String>, data: Option<String>, delay_write_ms: Option<u64>, port: u16, accept: bool, close_connection: CloseConnectionWhen) -> Self {
        TcpListenerData {
            file,
            data,
            delay_write_ms,
            port,
            accept,
            close_connection,
        }
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
    pub status: u16,
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
    pub fn new(
        response: Option<String>,
        status: u16,
        headers: HashMap<String, String>,
        delay: u64,
    ) -> Self {
        MockResponseConfiguration {
            response,
            status,
            headers,
            delay,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum TlsVersion {
    TLSv1_0,
    TLSv1_1,
    TLSv1_2,
    TLSv1_3,
}

/**
 * Configuration for a route.
 */
#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RouteConfiguration {
    // The URL of the endpoint.
    pub endpoint: String,
    // The proxy url. Example: `<http://localhost:8080>`
    pub proxy_url: Option<String>,
    // The log file where the requests and responses are stored.
    //TODO: Unimplemented
    pub log: Option<String>,
    // Verbose
    #[serde(default)]
    pub verbose: bool,
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
}

impl RouteConfiguration {
    /**
     * Create a new route configuration.
     *
     * `endpoint` The URL of the endpoint. Example `<http://localhost:8080>`
     * `proxy_url` Proxy to use.
     * `log` The log file where the requests and responses are stored.
     * `verbose` Verbose
     * `http1_only` HTTP/1 only
     * `accept_invalid_certs` Accept invalid certificates
     * `accept_invalid_hostnames` Accept invalid hostnames
     * `min_tls_version` Minimum TLS version
     * `max_tls_version` Maximum TLS version
     * `read_timeout` Read timeout
     * `connect_timeout` Connect timeout
     *
     */
    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::fn_params_excessive_bools)]
    #[must_use]
    pub fn new(
        endpoint: String,
        proxy_url: Option<String>,
        log: Option<String>,
        verbose: bool,
        http1_only: bool,
        accept_invalid_certs: bool,
        accept_invalid_hostnames: bool,
        min_tls_version: Option<TlsVersion>,
        max_tls_version: Option<TlsVersion>,
        read_timeout: Option<u64>,
        connect_timeout: Option<u64>,
    ) -> Self {
        RouteConfiguration {
            endpoint,
            proxy_url,
            log,
            verbose,
            http1_only,
            accept_invalid_certs,
            accept_invalid_hostnames,
            min_tls_version,
            max_tls_version,
            read_timeout,
            connect_timeout,
        }
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
    let id = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map_err(|err| {
            ApplicationError::ConfigurationError(format!("Failed to generate identifier: {err}"))
        })?;
    Ok(id.as_millis().to_string())
}

fn default_server_supported_tls_versions() -> Vec<TlsVersion> {
    vec![TlsVersion::TLSv1_2, TlsVersion::TLSv1_3]
}

fn default_as_true() -> bool {
    true
}

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
            vec![TestConfiguration::new(
                "Test".to_string(),
                "Test Description".to_string(),
                vec![ServerConfiguration::new(
                    "Server".to_string(),
                    Some(8080),
                    vec![EndpointConfiguration::new(
                        "/test".to_string(),
                        "GET".to_string(),
                        Some(MockResponseConfiguration::new(
                            Some("Test Response".to_string()),
                            200,
                            HashMap::new(),
                            0,
                        )),
                        Some(RouteConfiguration::new(
                            "/test".to_string(),
                            None,
                            None,
                            false,
                            false,
                            false,
                            false,
                            None,
                            None,
                            None,
                            None,
                        )),
                    ).unwrap()],
                    None,
                ).unwrap()],
                Vec::new(),
            ).unwrap()],
        );

        assert_eq!(configuration.name, "Test Configuration");
        assert_eq!(configuration.description, "Test Configuration Description");
        assert_eq!(configuration.tests.len(), 1);
        assert_eq!(configuration.tests[0].name, "Test");
        assert_eq!(configuration.tests[0].description, "Test Description");
        assert_eq!(configuration.tests[0].servers.len(), 1);
        assert_eq!(configuration.tests[0].servers[0].name, "Server");
        assert_eq!(configuration.tests[0].servers[0].http_port, Some(8080));
        assert_eq!(configuration.tests[0].servers[0].endpoints.len(), 1);
        assert_eq!(
            configuration.tests[0].servers[0].endpoints[0].endpoint,
            "/test"
        );
        assert_eq!(
            configuration.tests[0].servers[0].endpoints[0]
                .mock
                .as_ref()
                .unwrap()
                .response,
            Some("Test Response".to_string())
        );
        assert_eq!(
            configuration.tests[0].servers[0].endpoints[0]
                .mock
                .as_ref()
                .unwrap()
                .status,
            200
        );
        assert_eq!(
            configuration.tests[0].servers[0].endpoints[0]
                .mock
                .as_ref()
                .unwrap()
                .headers
                .len(),
            0
        );
        assert_eq!(
            configuration.tests[0].servers[0].endpoints[0]
                .mock
                .as_ref()
                .unwrap()
                .delay,
            0
        );
        assert_eq!(
            configuration.tests[0].servers[0].endpoints[0]
                .route
                .as_ref()
                .unwrap()
                .endpoint,
            "/test"
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
            vec![TestConfiguration::new(
                "Test".to_string(),
                "Test Description".to_string(),
                vec![ServerConfiguration::new(
                    "Server".to_string(),
                    Some(8080),
                    vec![EndpointConfiguration::new(
                        "/test".to_string(),
                        "GET".to_string(),
                        Some(MockResponseConfiguration::new(
                            Some("Test Response".to_string()),
                            200,
                            HashMap::new(),
                            0,
                        )),
                        Some(RouteConfiguration::new(
                            "/test".to_string(),
                            None,
                            None,
                            false,
                            false,
                            false,
                            false,
                            None,
                            None,
                            None,
                            None,
                        )),
                    ).unwrap()],
                    None,
                ).unwrap()],
                Vec::new(),
            ).unwrap()],
        );

        let serialized = serde_json::to_string(&configuration).unwrap();
        let deserialized: AppConfiguration = serde_json::from_str(&serialized).unwrap();

        assert_eq!(configuration, deserialized);
    }

    #[test]
    fn test_save_load() {
        let configuration = AppConfiguration::new(
            "Test Configuration".to_string(),
            "Test Configuration Description".to_string(),
            vec![TestConfiguration::new(
                "Test".to_string(),
                "Test Description".to_string(),
                vec![ServerConfiguration::new(
                    "Server".to_string(),
                    Some(8080),
                    vec![EndpointConfiguration::new(
                        "/test".to_string(),
                        "GET".to_string(),
                        Some(MockResponseConfiguration::new(
                            Some("Test Response".to_string()),
                            200,
                            HashMap::new(),
                            0,
                        )),
                        Some(RouteConfiguration::new(
                            "/test".to_string(),
                            None,
                            None,
                            false,
                            false,
                            false,
                            false,
                            None,
                            None,
                            None,
                            None,
                        )),                        
                    ).unwrap()],
                    None,
                ).unwrap()],
                Vec::new(),
            ).unwrap()],
        );

        let path = "/tmp/test.json";
        let _ = configuration.save(path);
        let loaded = AppConfiguration::load(path).unwrap();

        assert_eq!(configuration, loaded);
    }
}
