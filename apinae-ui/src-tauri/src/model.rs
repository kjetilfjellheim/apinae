use apinae_lib::config::{EndpointConfiguration, EndpointType, HttpsConfiguration, MockResponseConfiguration, RouteConfiguration, ServerConfiguration, TcpListenerData, TestConfiguration, TlsVersion};

/**
 * This struct represents a test row for both request and responses.
 */
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestRow {
    // The unique identifier of the test.
    pub id: String,
    // The name of the test.
    pub name: String,
    // The description of the test.
    pub description: String,
    // The process id of the test.
    pub process_id: Option<u32>,
}

impl TestRow {
    /**
     * Create a new test row.
     *
     * # Arguments
     * `id` - The unique identifier of the test.
     * `name` - The name of the test.
     * `description` - The description of the test.
     * `process_id` - The process id of the test.
     *
     * # Returns
     * `TestRow` - The test row.
     */
    pub fn new(id: &str, name: &str, description: &str, process_id: Option<u32>) -> Self {
        Self { id: id.to_string(), name: name.to_string(), description: description.to_string(), process_id }
    }
}

impl From<TestConfiguration> for TestRow {
    /**
     * Convert a test configuration to a test row.
     */
    fn from(test: TestConfiguration) -> Self {
        Self { id: test.id.clone(), name: test.name.clone(), description: test.description.clone(), process_id: None }
    }
}

/**
 * This struct represents a http server row for both request and responses.
 */
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HttpServerRow {
    // The unique identifier of the http server.
    pub id: String,
    // The name of the http server.
    pub name: String,
    // The http port of the http server.
    pub http_port: Option<u16>,
    // The https configuration of the http server.
    //TODO: Change to https configuration row.
    pub https_config: Option<HttpsRow>,
    // The endpoints of the http server.
    pub endpoints: Vec<EndpointRow>,
}

impl From<&ServerConfiguration> for HttpServerRow {
    /**
     * Convert a server configuration to a http server row.
     */
    fn from(http_server: &ServerConfiguration) -> Self {
        Self {
            id: http_server.id.clone(),
            name: http_server.name.clone(),
            http_port: http_server.http_port,
            https_config: http_server.https_config.as_ref().map(std::convert::Into::into),
            endpoints: http_server.endpoints.iter().map(std::convert::Into::into).collect(),
        }
    }
}

/**
 * This struct represents a tcp listener row for both request and responses.
 */
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EndpointRow {
    // The unique identifier of the endpoint.
    pub id: String,
    // The path expression of the endpoint.
    pub path_expression: Option<String>,
    // The method of the endpoint.
    pub method: Option<String>,
    // The body expression.
    pub body_expression: Option<String>,
    // The mock response of the endpoint.
    pub mock: Option<MockRow>,
    // The route configuration of the endpoint.
    pub route: Option<RouteRow>,
}

impl From<&EndpointConfiguration> for EndpointRow {
    /**
     * Convert an endpoint configuration to an endpoint row.
     */
    fn from(endpoint_config: &EndpointConfiguration) -> Self {
        Self {
            id: endpoint_config.id.clone(),
            path_expression: endpoint_config.path_expression.clone(),
            method: endpoint_config.method.clone(),
            body_expression: endpoint_config.body_expression.clone(),
            mock: endpoint_config.endpoint_type.as_ref().and_then(|endpoint_type| match endpoint_type {
                EndpointType::Mock { configuration } => Some(MockRow::from(configuration)),
                EndpointType::Route { configuration: _ } => None,
            }),
            route: endpoint_config.endpoint_type.as_ref().and_then(|endpoint_type| match endpoint_type {
                EndpointType::Route { configuration } => Some(RouteRow::from(configuration)),
                EndpointType::Mock { configuration: _ } => None,
            }),
        }
    }
}

/**
 * This struct represents a mock row for both request and responses.
 */
#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MockRow {
    // The response of the mock.
    pub response: Option<String>,
    // The status response of the mock.
    pub status: String,
    // The headers of the mock.
    pub headers: String,
    // The delay for writing responses.
    pub delay: u64,
}

impl From<&MockResponseConfiguration> for MockRow {
    /**
     * Convert a mock response configuration to a mock row.
     */
    fn from(mock: &MockResponseConfiguration) -> Self {
        Self {
            response: mock.response.clone(),
            status: mock.status.clone(),
            headers: mock.headers.iter().fold(String::new(), |mut output, val| {
                output.push_str(&format!("{}: {}\n", val.0, val.1));
                output
            }),
            delay: mock.delay,
        }
    }
}

impl From<&MockRow> for MockResponseConfiguration {
    /**
     * Convert a mock row to a mock response configuration.
     */
    fn from(mock: &MockRow) -> Self {
        MockResponseConfiguration::new(
            mock.response.clone(),
            mock.status.clone(),
            mock.headers
                .split('\n')
                .filter(|header| !header.is_empty() && header.contains(':'))
                .map(|header| {
                    let mut parts = header.split(':');
                    let key = parts.next().unwrap_or("").trim();
                    let value = parts.next().unwrap_or("").trim();
                    (String::from(key), String::from(value))
                })
                .collect(),
            mock.delay,
        )
    }
}

/**
 * This struct represents a route row for both request and responses.
 */
#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RouteRow {
    // The url to route to.
    pub url: String,
    // The proxy to use.
    pub proxy_url: Option<String>,
    // The flag to use only http1.
    pub http1_only: bool,
    // The flag to accept invalid certificates.
    pub accept_invalid_certs: bool,
    // The flag to accept invalid hostnames.
    pub accept_invalid_hostnames: bool,
    // The minimum tls version.
    pub min_tls_version: Option<String>,
    // The maximum tls version.
    pub max_tls_version: Option<String>,
    // The read timeout.
    pub read_timeout: Option<u64>,
    // The connect timeout.
    pub connect_timeout: Option<u64>,
}

impl From<&RouteConfiguration> for RouteRow {
    /**
     * Convert a route configuration to a route row.
     */
    fn from(route: &RouteConfiguration) -> Self {
        Self {
            url: route.url.clone(),
            proxy_url: route.proxy_url.clone(),
            http1_only: route.http1_only,
            accept_invalid_certs: route.accept_invalid_certs,
            accept_invalid_hostnames: route.accept_invalid_hostnames,
            min_tls_version: route.min_tls_version.clone().map(String::from),
            max_tls_version: route.max_tls_version.clone().map(String::from),
            read_timeout: route.read_timeout,
            connect_timeout: route.connect_timeout,
        }
    }
}

impl From<&RouteRow> for RouteConfiguration {
    /**
     * Convert a route row to a route configuration.
     */
    fn from(route: &RouteRow) -> Self {
        RouteConfiguration::new(
            route.url.clone(),
            route.proxy_url.clone(),
            None,
            route.http1_only,
            route.accept_invalid_certs,
            route.accept_invalid_hostnames,
            route.min_tls_version.clone().map(TlsVersion::from),
            route.max_tls_version.clone().map(TlsVersion::from),
            route.read_timeout,
            route.connect_timeout,
        )
    }
}

/**
 * This struct represents a https configuration.
 */
#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct HttpsRow {
    // The path to the server certificate.
    pub server_certificate: String,
    // The path to the private key.
    pub private_key: String,
    // The https port.
    pub https_port: u16,
    // The path to the client certificate.
    pub client_certificate: Option<String>,
    // The supported tls versions.
    pub supported_tls_versions: Vec<String>,
}

impl From<&HttpsConfiguration> for HttpsRow {
    /**
     * Convert a https configuration to a https configuration.
     */
    fn from(https_config: &HttpsConfiguration) -> Self {
        Self {
            server_certificate: https_config.server_certificate.clone(),
            private_key: https_config.private_key.clone(),
            https_port: https_config.https_port,
            client_certificate: https_config.client_certificate.clone(),
            supported_tls_versions: https_config.clone().supported_tls_versions.into_iter().map(String::from).collect(),
        }
    }
}

impl From<HttpsRow> for HttpsConfiguration {
    /**
     * Convert a https row to a https configuration.
     */
    fn from(https_row: HttpsRow) -> Self {
        HttpsConfiguration::new(
            https_row.server_certificate,
            https_row.private_key,
            https_row.https_port,
            https_row.client_certificate,
            https_row.supported_tls_versions.into_iter().map(TlsVersion::from).collect(),
        )
    }
}

/**
 * This struct represents a tcp listener object.
 */
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TcpListenerRow {
    // The unique identifier of the tcp listener.
    pub id: String,
    // The file to use for response.
    pub file: Option<String>,
    // The data to use for response.
    pub data: Option<String>,
    // The delay for writing responses.
    pub delay_write_ms: Option<u64>,
    // The port of the tcp listener.
    pub port: u16,
    // The flag to accept connections.
    pub accept: bool,
    // The flag to close the connection.
    pub close_connection: String,
}

impl From<&TcpListenerData> for TcpListenerRow {
    /**
     * Convert a tcp listener data to a tcp listener row.
     */
    fn from(tcp_listener: &TcpListenerData) -> Self {
        Self {
            id: tcp_listener.id.clone(),
            file: tcp_listener.file.clone(),
            data: tcp_listener.data.clone(),
            delay_write_ms: tcp_listener.delay_write_ms,
            port: tcp_listener.port,
            accept: tcp_listener.accept,
            close_connection: String::from(tcp_listener.clone().close_connection),
        }
    }
}

mod test {

    #[allow(unused_imports)]
    use std::collections::HashMap;

    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_test_row_from_test_configuration() {
        let test_config = TestConfiguration::new("name".to_owned(), "description".to_owned(), Vec::new(), Vec::new(), None).unwrap();

        let test_row = TestRow::from(test_config);

        assert_eq!(test_row.name, "name");
        assert_eq!(test_row.description, "description");
    }

    #[test]
    fn test_http_server_row_from_server_configuration() {
        let server_config = ServerConfiguration::new("name".to_owned(), Some(8080), Vec::new(), None).unwrap();

        let http_server_row = HttpServerRow::from(&server_config);

        assert_eq!(http_server_row.name, "name");
        assert_eq!(http_server_row.https_config, None);
        assert_eq!(http_server_row.http_port, Some(8080));
    }

    #[test]
    fn test_endpoint_row_from_endpoint_configuration() {
        let endpoint_config = EndpointConfiguration::new(Some("path".to_owned()), Some("method".to_owned()), None, None).unwrap();

        let endpoint_row = EndpointRow::from(&endpoint_config);

        assert_eq!(endpoint_row.path_expression, Some("path".to_string()));
        assert_eq!(endpoint_row.method, Some("method".to_string()));
        assert_eq!(endpoint_row.mock, None);
        assert_eq!(endpoint_row.route, None);
    }

    #[test]
    fn test_mock_row_from_mock_response_configuration() {
        let mock_config = MockResponseConfiguration::new(None, String::from("200"), HashMap::new(), 0);

        let mock_row = MockRow::from(&mock_config);

        assert_eq!(mock_row.response, None);
        assert_eq!(mock_row.status, String::from("200"));
        assert_eq!(mock_row.headers, String::new());
        assert_eq!(mock_row.delay, 0);
    }

    #[test]
    fn test_route_row_from_route_configuration() {
        let route_config = RouteConfiguration::new("url".to_owned(), None, None, false, false, false, Some(TlsVersion::TLSv1_2), Some(TlsVersion::TLSv1_1), None, None);

        let route_row = RouteRow::from(&route_config);

        assert_eq!(route_row.url, "url");
        assert_eq!(route_row.proxy_url, None);
        assert!(!route_row.http1_only);
        assert!(!route_row.accept_invalid_certs);
        assert!(!route_row.accept_invalid_hostnames);
        assert_eq!(route_row.min_tls_version, Some("TLSv1.2".to_owned()));
        assert_eq!(route_row.max_tls_version, Some("TLSv1.1".to_owned()));
        assert_eq!(route_row.read_timeout, None);
        assert_eq!(route_row.connect_timeout, None);
    }

    /**
     * Test the conversion from a https row to a https configuration.
     */
    #[test]
    fn test_https_row_from_https_configuration() {
        let https_config = HttpsConfiguration::new("server_certificate".to_owned(), "private_key".to_owned(), 443, None, Vec::new());
        let https_row = HttpsRow::from(&https_config);

        assert_eq!(https_row.server_certificate, "server_certificate");
        assert_eq!(https_row.private_key, "private_key");
        assert_eq!(https_row.https_port, 443);
        assert_eq!(https_row.client_certificate, None);
        assert_eq!(https_row.supported_tls_versions, Vec::<String>::new());
    }

    /**
     * Test the conversion from a mock row to a mock response configuration with
     * headers and response.
     */
    #[test]
    fn test_from_mockrow_to_mockresponseconfiguration() {
        let mock_row = MockRow { response: Some("response".to_owned()), status: String::from("200"), headers: "header: value\nheader2:\n \n".to_owned(), delay: 0 };

        let mock_config = MockResponseConfiguration::from(&mock_row);

        assert_eq!(mock_config.response, Some("response".to_owned()));
        assert_eq!(mock_config.status, String::from("200"));
        assert_eq!(mock_config.headers.get("header"), Some(&"value".to_owned()));
        assert_eq!(mock_config.headers.get("header2"), Some(&String::new()));
        assert_eq!(mock_config.delay, 0);
    }

    /**
     * Test the conversion from a mock row to a mock response configuration
     * with no headers and empty response.
     */
    #[test]
    fn test_from_mockrow_to_mockresponseconfiguration_no_header() {
        let mock_row = MockRow { response: None, status: String::from("200"), headers: String::new(), delay: 0 };

        let mock_config = MockResponseConfiguration::from(&mock_row);

        assert_eq!(mock_config.response, None);
        assert_eq!(mock_config.status, String::from("200"));
        assert_eq!(mock_config.headers.len(), 0);
        assert_eq!(mock_config.delay, 0);
    }
}
