use std::{fs::File, io::BufReader, str::FromStr, sync::Arc, time::Duration};

use actix_web::{http::StatusCode, middleware::Logger, web, App, HttpRequest, HttpResponse, HttpServer};
use apinae_lib::{
    config::{EndpointConfiguration, EndpointType, HttpsConfiguration, MockResponseConfiguration, RouteConfiguration, ServerConfiguration, TlsVersion},
    error::ApplicationError,
};
use log::{error, info};
use regex::Regex;
use reqwest::Method;
use rustls::{
    pki_types::PrivateKeyDer,
    server::{danger::ClientCertVerifier, WebPkiClientVerifier},
    version::{TLS12, TLS13},
    RootCertStore, ServerConfig, SupportedProtocolVersion,
};
use rustls_pemfile::{certs, pkcs8_private_keys};
use tokio::task::JoinHandle;

use super::common::StartableServer;

/**
 * The character used to separate query parameters in a URL.
 */
const QUERYPARAMSEPARATOR: char = '&';
/**
 * The character used to separate key and value in a query parameter.
 */
const KEYVALUESEPARATOR: char = '=';

/**
 * The `AppServer` struct is used to configure and start the server.
 */
pub struct AppServer {
    // Server configuration
    server_configuration: ServerConfiguration,
    // Parameters
    params: Vec<(String, String)>,
}

impl AppServer {
    /**
     * Create a new `AppServer`.
     *
     * # Arguments
     * `server_configuration`: The server configuration.
     * `params`: The parameters to use for the server.
     *
     * # Returns
     * The created `AppServer`.
     */
    pub fn new(server_configuration: ServerConfiguration, params: Vec<(String, String)>) -> Self {
        AppServer { server_configuration, params }
    }

    /**
     * Start the server with HTTP.
     *
     * # Returns
     * Ok if the server was started.
     *
     * # Errors
     * An error if the server could not be started.
     */
    pub fn start_server_http(&mut self) -> Result<(), ApplicationError> {
        if let Some(http_port) = self.server_configuration.http_port {
            log::info!("Starting http server on port: {http_port}");
            let appstate = web::Data::new(AppState::new(self.server_configuration.clone(), self.params.clone()));
            let server = HttpServer::new(move || App::new().wrap(Logger::default()).app_data(appstate.clone()).default_service(web::to(request_handler)))
                .bind(("127.0.0.1", http_port))
                .map_err(|err| ApplicationError::ServerStartUpError(format!("Failed to create http server: {err}")))?;
            let server = server.workers(2).run();
            tokio::spawn(async move {
                match server.await {
                    Ok(()) => {
                        log::info!("Server started");
                    }
                    Err(err) => error!("{err}"),
                }
            });
        }
        Ok(())
    }

    /**
     * Start the server with HTTPS. It uses rust tls to create a secure server.
     *
     * # Returns
     * Ok if the server was started.
     *
     * # Errors
     * An error if the server could not be started.
     */
    pub fn start_server_https(&self) -> Result<(), ApplicationError> {
        let config = self.server_configuration.clone();
        if let Some(https_config) = config.https_config {
            log::info!("Starting https server on port: {}", https_config.https_port);
            let ssl_builder = ssl_builder(&https_config)?;
            let appstate = web::Data::new(AppState::new(self.server_configuration.clone(), self.params.clone()));
            let server = HttpServer::new(move || App::new().wrap(Logger::default()).app_data(appstate.clone()).default_service(web::to(request_handler)))
                .bind_rustls_0_23("127.0.0.1:".to_owned() + https_config.https_port.to_string().as_str(), ssl_builder)
                .map_err(|err| ApplicationError::ServerStartUpError(format!("Failed to create https server: {err}")))?;
            let server = server.workers(2).run();
            tokio::spawn(async move {
                match server.await {
                    Ok(()) => {}
                    Err(err) => error!("Server error: {err}"),
                }
            });
        }
        Ok(())
    }
}

/**
 * Handle the request. All request will be handled by this function.
 *
 * # Arguments
 * `server_configuration`: The server configuration.
 * `req`: The request.
 *
 * # Returns
 * The response.
 */
async fn request_handler(app_state: web::Data<AppState>, req: HttpRequest, payload: Option<web::Payload>) -> HttpResponse {
    let payload_string: Option<String> = get_body_as_string(payload).await;
    let path = get_path(&req);
    for endpoint in &app_state.server_configuration.endpoints {         
        match is_valid_endpoint(path.as_str(), req.method().as_str(), endpoint, &payload_string) {
            Ok(true) => match handle_endpoint(endpoint, &req, &payload_string, app_state.params.clone()).await {
                Ok(response) => return response,
                Err(err) => {
                    error!("Error handling request: {err}. Returning not implemented");
                    return get_non_implemented_response();
                }
            },
            Ok(false) => {}
            Err(err) => {
                error!("Error checking endpoint: {err}. Returning service unavailable");
                return HttpResponse::ServiceUnavailable().body(err.to_string());
            }
        }
    }
    info!("No endpoints found: Returning not implemented");
    get_non_implemented_response()
}

/**
 * Get the not implemented response.
 * The response will have a status code of 501 Not Implemented and a body with the message "Not implemented".
 *
 * # Returns
 * The not implemented response.
 */
fn get_non_implemented_response() -> HttpResponse {
    HttpResponse::NotImplemented().body("Not implemented")
}
/**
 * Get the request path including query from the request object.
 * If the query string is not empty and contains more than just an = sign, it will be appended to the path.
 *
 * # Arguments
 * `req`: The request.
 *
 * # Returns
 * The request path.
 */
fn get_path(req: &HttpRequest) -> String {
    let mut path = req.path().to_string();
    println!("Request path: {path} query: {}", req.query_string());
    if !req.query_string().is_empty() && req.query_string() != "=" {
        path.push('?');
        path.push_str(req.query_string());
    }
    path.trim().to_string()
}

/**
 * This function will read the payload from the request and convert it to a string.
 * If the payload is None, it will return None. If the payload is Some, it will read 
 * the bytes from the payload and convert it to a string.
 *
 * # Arguments
 * `payload`: The payload.
 *
 * # Returns
 * The payload as a string.
 */
async fn get_body_as_string(payload: Option<web::Payload>) -> Option<String> {
    let payload_string: Option<String> = if let Some(payload) = payload {
        payload
            .to_bytes()
            .await
            .map_err(|err| {
                error!("Failed to read payload: {err}");
            })
            .and_then(|bytes| {
                String::from_utf8(bytes.to_vec()).map_err(|err| {
                    error!("Failed to convert payload to string: {err}");
                })
            })
            .ok()
    } else {
        None
    };
    payload_string
}

/**
 * Check if the request is a valid endpoint. This function will check the request path, method, and payload against the endpoint configuration.
 * If all are true it will return true, otherwise it will return false.
 *
 * # Arguments
 * `request_path`: The request path.
 * `request_method`: The request method.
 * `endpoint`: The endpoint configuration.
 * `payload_string`: The request payload as a string.
 *
 * # Returns
 * True if the request is a valid endpoint.
 *
 * # Errors
 * An error if the endpoint is invalid.
 */
fn is_valid_endpoint(request_path: &str, request_method: &str, endpoint: &EndpointConfiguration, payload_string: &Option<String>) -> Result<bool, ApplicationError> {
    let path_result = check_regexp(endpoint.path_expression.clone(), &Some(request_path.to_owned()))?;
    let payload_result = check_regexp(endpoint.body_expression.clone(), payload_string)?;
    let method_result = endpoint.method.clone().map_or_else(|| true, |f| f == request_method);
    Ok(path_result && payload_result && method_result)
}

/**
 * Check regular expression against the specified data.
 *
 * If the regular expression is not provided, it will return true.
 * If the data is not provided, it will return false if a regular expression is defined.
 * If both are provided, it will check if the data matches the regular expression.
 *
 * # Arguments
 * `regexp`: The regular expression.
 * `data`: The data to check.
 *
 * # Returns
 * Regular expression result.
 *
 * # Errors
 * An error if the regular expression is invalid
 */
fn check_regexp(regexp: Option<String>, data: &Option<String>) -> Result<bool, ApplicationError> {
    let regexp = if let Some(regexp) = regexp {
        Regex::new(regexp.as_str()).map_err(|err| ApplicationError::ConfigurationError(format!("Error in regular expression {regexp}: {err}")))?
    } else {
        // If no regexp is provided, we return true
        return Ok(true);
    };
    let Some(data) = data else {
        // If no data is provided, but a regexp is defined, we return false
        return Ok(false);
    };
    Ok(regexp.is_match(data))
}

/**
 * Handle the endpoint. This function will check the endpoint type and call the appropriate handler either
 * a mock response or a route request. If it fails to match any endpoint type, it will return a not 
 * implemented response.
 *
 * # Arguments
 * `endpoint`: The endpoint configuration.
 *
 * # Returns
 * The response.
 *
 * # Errors
 * An error if the status code is invalid.
 */
async fn handle_endpoint(endpoint: &EndpointConfiguration, req: &HttpRequest, payload: &Option<String>, params: Vec<(String, String)>) -> Result<HttpResponse, ApplicationError> {
    if let Some(endpoint_type) = &endpoint.endpoint_type {
        match endpoint_type {
            EndpointType::Mock { configuration } => {
                return generate_mock_response(configuration, params).await;
            }
            EndpointType::Route { configuration } => {
                return route_request(configuration, req, payload.clone()).await;
            }
        }
    }
    Ok(HttpResponse::NotImplemented().body("Not implemented"))
}

/**
 * Route the request to the configured route configuration.
 *
 * # Arguments
 * `route_configuration`: The route configuration.
 * `req`: The request.
 *
 * # Returns
 * The response.
 */
async fn route_request(route_configuration: &RouteConfiguration, req: &HttpRequest, payload: Option<String>) -> Result<HttpResponse, ApplicationError> {
    let mut url = route_configuration.url.clone();
    url.push_str(req.path());

    let request = get_request(req, payload.clone(), url)?;

    let client = get_client(route_configuration)?;

    if let Some(delay_before) = route_configuration.delay_before {
        log::debug!("Waiting {delay_before}ms before request");
        tokio::time::sleep(Duration::from_millis(delay_before)).await;
    }

    let response = client.execute(request).await.map_err(|err| ApplicationError::RoutingError(format!("Error executing client request: {err}")))?;

    let response = get_response(response).await?;

    if let Some(delay_after) = route_configuration.delay_after {
        log::debug!("Waiting {delay_after}ms after request");
        tokio::time::sleep(Duration::from_millis(delay_after)).await;
    }

    Ok(response)
}

/**
 * Converts the request response to this applications response.
 *
 * # Arguments
 * `response`: The response.
 *
 * # Returns
 * The response.
 *
 * # Errors
 * An error if the status code is invalid.
 */
async fn get_response(response: reqwest::Response) -> Result<HttpResponse, ApplicationError> {
    log::debug!("Creating response");
    let mut response_builder = HttpResponse::build(
        StatusCode::from_u16(response.status().as_u16()).map_err(|err| ApplicationError::RoutingError(format!("Invalid status code for response {}: {err}", response.status().as_str())))?,
    );
    for (key, value) in response.headers() {
        response_builder.append_header((key.as_str(), value.to_str().map_err(|err| ApplicationError::RoutingError(format!("Invalid header value for response {value:?}: {err}")))?));
    }
    let body = response.text().await.map_err(|err| ApplicationError::RoutingError(format!("Invalid body for response: {err}")))?;

    let response = response_builder.body(body);

    Ok(response)
}

/**
 * Get the function name. This is required due to the fact that the client
 * allows proxy while sending from the request object does not.
 *
 * # Arguments
 * `route_configuration`: The route configuration.
 *
 * # Returns
 * Client object to make the requests.
 *
 * # Errors
 * An error if the client could not be created.
 *
 */
fn get_client(route_configuration: &RouteConfiguration) -> Result<reqwest::Client, ApplicationError> {
    log::debug!("Creating client");
    let mut client_builder = reqwest::Client::builder();
    if let Some(connect_timeout) = route_configuration.connect_timeout {
        client_builder = client_builder.connect_timeout(Duration::from_millis(connect_timeout));
    }
    if let Some(read_timeout) = route_configuration.read_timeout {
        client_builder = client_builder.read_timeout(Duration::from_millis(read_timeout));
    }
    if route_configuration.http1_only {
        client_builder = client_builder.http1_only();
    }
    client_builder = client_builder.danger_accept_invalid_certs(route_configuration.accept_invalid_certs).danger_accept_invalid_hostnames(route_configuration.accept_invalid_hostnames);

    if let Some(min_tls_version) = &route_configuration.min_tls_version {
        match min_tls_version {
            TlsVersion::TLSv1_0 => {
                client_builder = client_builder.min_tls_version(reqwest::tls::Version::TLS_1_0);
            }
            TlsVersion::TLSv1_1 => {
                client_builder = client_builder.min_tls_version(reqwest::tls::Version::TLS_1_1);
            }
            TlsVersion::TLSv1_2 => {
                client_builder = client_builder.min_tls_version(reqwest::tls::Version::TLS_1_2);
            }
            TlsVersion::TLSv1_3 => {
                client_builder = client_builder.min_tls_version(reqwest::tls::Version::TLS_1_3);
            }
        }
    }

    let client = match &route_configuration.proxy_url {
        Some(proxy) => {
            log::debug!("Creating client with proxy: {proxy}");
            let reqwest_proxy = reqwest::Proxy::all(proxy.clone()).map_err(|err| ApplicationError::RoutingError(format!("Could not create proxy settings: {err}")))?;
            client_builder.proxy(reqwest_proxy).build().map_err(|err| ApplicationError::RoutingError(format!("Failed to create client with proxy: {err}")))?
        }
        None => client_builder.build().map_err(|err| ApplicationError::RoutingError(format!("Failed to create client without proxy: {err}")))?,
    };
    Ok(client)
}

/**
 * Get request object for client. 
 *
 * # Arguments
 * `req`: The original request.
 * `payload`: The payload.
 * `url`: The URL.
 *
 * # Returns
 * The request object.
 *
 * # Errors
 * An error if the method is invalid.
 * An error if the version is invalid.
 * An error if the headers are invalid.
 * An error if the payload is invalid.
 * An error if the query parameters are invalid.
 * An error if the request could not be built.
 *
 * # Example
 * ```
 * let request = get_request(req, payload, url).await?;
 * ```
 *
 */
fn get_request(req: &HttpRequest, payload: Option<String>, url: String) -> Result<reqwest::Request, ApplicationError> {
    log::debug!("Creating request");
    let mut request_builder = reqwest::Client::new()
        .request(Method::from_bytes(req.method().as_str().as_bytes()).map_err(|err| ApplicationError::RoutingError(format!("Failed to map method {}: {err}", req.method().as_str())))?, url);
    request_builder = match req.version() {
        actix_web::http::Version::HTTP_09 => request_builder.version(reqwest::Version::HTTP_09),
        actix_web::http::Version::HTTP_10 => request_builder.version(reqwest::Version::HTTP_10),
        actix_web::http::Version::HTTP_11 => request_builder.version(reqwest::Version::HTTP_11),
        actix_web::http::Version::HTTP_2 => request_builder.version(reqwest::Version::HTTP_2),
        actix_web::http::Version::HTTP_3 => request_builder.version(reqwest::Version::HTTP_3),
        _ => return Err(ApplicationError::RoutingError("Invalid version".to_string())),
    };
    for (key, value) in req.headers() {
        let value = value.to_str().map_err(|err| ApplicationError::RoutingError(format!("Failed to map request header: {err}")))?;
        request_builder = request_builder.header(key.as_str(), value);
    }
    if let Some(payload) = payload {
        let bytes = payload.clone().into_bytes();
        request_builder = request_builder.body(bytes);
    }
    request_builder = request_builder.query(
        &req.query_string()
            .split(QUERYPARAMSEPARATOR)
            .map(|x| {
                let mut parts = x.split(KEYVALUESEPARATOR);
                let key = parts.next().map_or_else(|| "", |x| x);
                let value = parts.next().map_or_else(|| "", |x| x);
                (key.to_owned(), value.to_owned())
            })
            .collect::<Vec<(String, String)>>(),
    );
    let request = request_builder.build().map_err(|err| ApplicationError::RoutingError(format!("Failed to create request: {err}")))?;
    Ok(request)
}

/**
 * Generate a mock response. This function will wait for the specified delay, then build the response based on the 
 * mock response configuration. It will replace any parameters in the status code, headers, and response body with 
 * the values from the provided parameters vector.
 *
 * # Arguments
 * `mock_response`: The mock response configuration.
 *
 * # Returns
 * The generated response.
 *
 * # Errors
 * An error if the status code is invalid.
 */
async fn generate_mock_response(mock_response: &MockResponseConfiguration, params: Vec<(String, String)>) -> Result<HttpResponse, ApplicationError> {
    if mock_response.delay > 0 {
        log::debug!("Waiting {}ms for mock response", mock_response.delay);
        tokio::time::sleep(Duration::from_millis(mock_response.delay)).await;
    }
    log::debug!("Generating mock response");
    let mut response_builder: actix_web::HttpResponseBuilder =
        HttpResponse::build(StatusCode::from_str(convert_params(mock_response.status.as_str(), &params).as_str()).map_err(|err| ApplicationError::ConfigurationError(err.to_string()))?);
    for (key, value) in &mock_response.headers {
        response_builder.append_header((convert_params(key.as_str(), &params), convert_params(value.as_str(), &params)));
    }
    if let Some(response) = &mock_response.response {
        return Ok(response_builder.body(convert_params(response.clone().as_str(), &params)));
    }
    Ok(response_builder.finish())
}

/**
 * Convert parameters to string. The parameters are in the format ${key} and will be replaced with the value from 
 * the parameters vector.
 *
 * # Arguments
 * `value`: The value to convert.
 * `params`: The parameters.
 *
 * # Returns
 * The converted value.
 */
fn convert_params(value: &str, params: &Vec<(String, String)>) -> String {
    let mut result = value.to_string();
    for (key, value) in params {
        let key = format!("${{{key}}}");
        if result.contains(&key) {
            result = result.replace(&key, value);
        }
    }
    result
}
/**
 * Create a new SSL builder.
 *
 * # Arguments
 * `https_config`: The HTTPS configuration.
 *
 * # Returns
 * The SSL builder.
 *
 * # Errors
 * An error if the acceptor could not be created.
 * An error if the private key file could not be set.
 * An error if the certificate chain file could not be set.
 *
 */
fn ssl_builder(https_config: &HttpsConfiguration) -> Result<ServerConfig, ApplicationError> {
    log::info!("Creating ssl builder");
    let config_builder = ServerConfig::builder_with_protocol_versions(&get_protocol_versions(&https_config.supported_tls_versions));
    log::debug!("Supported TLS versions: {:?}", &https_config.supported_tls_versions);

    let config_builder = match https_config.clone().client_certificate {
        Some(client_certificate) => {
            log::debug!("Require client certificate: {client_certificate}");
            let client_auth = get_client_verifier(client_certificate)?;
            config_builder.with_client_cert_verifier(client_auth)
        }
        None => config_builder.with_no_client_auth(),
    };

    let cert_file = &mut BufReader::new(File::open(https_config.clone().server_certificate).map_err(|err| ApplicationError::ConfigurationError(format!("Failed to read certificate file: {err}")))?);
    let key_file = &mut BufReader::new(File::open(https_config.clone().private_key).map_err(|err| ApplicationError::ConfigurationError(format!("Failed to read private key file: {err}")))?);

    let cert_chain = certs(cert_file).collect::<Result<Vec<_>, _>>().map_err(|err| ApplicationError::ConfigurationError(format!("Failed to convert certificate to der: {err}")))?;
    let mut keys = pkcs8_private_keys(key_file)
        .map(|key| key.map(PrivateKeyDer::Pkcs8))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|err| ApplicationError::ConfigurationError(format!("Failed to convert private key to der: {err}")))?;
    let config = config_builder.with_single_cert(cert_chain, keys.remove(0)).map_err(|err| ApplicationError::ConfigurationError(format!("Failed to create server config: {err}")))?;

    Ok(config)
}

/**
 * Get the protocol versions.
 *
 * # Arguments
 * `supported_tls_versions`: The supported TLS versions.
 *
 * # Returns
 * The protocol versions.
 *
 */
fn get_protocol_versions(supported_tls_versions: &[TlsVersion]) -> Vec<&'static SupportedProtocolVersion> {
    supported_tls_versions
        .iter()
        .map(|version| match version {
            TlsVersion::TLSv1_0 | TlsVersion::TLSv1_1 | TlsVersion::TLSv1_2 => &TLS12,
            TlsVersion::TLSv1_3 => &TLS13,
        })
        .collect()
}

/**
 * Get the client verifier.
 *
 * # Arguments
 * `client_certificate`: The client certificate.
 *
 * # Returns
 * The client verifier.
 *
 * # Errors
 * An error if the client verifier could not be created.
 *
 */
fn get_client_verifier(client_certificate: String) -> Result<Arc<dyn ClientCertVerifier>, ApplicationError> {
    log::info!("Creating client verifier");
    let cert_file = &mut BufReader::new(File::open(client_certificate).map_err(|err| ApplicationError::ConfigurationError(format!("Failed to read client certificate: {err}")))?);
    let cert_chain = certs(cert_file).collect::<Result<Vec<_>, _>>().map_err(|err| ApplicationError::ConfigurationError(format!("Failed to convert client certificate to der: {err}")))?;

    let mut cert_store = RootCertStore::empty();

    for cert in cert_chain {
        cert_store.add(cert).map_err(|err| ApplicationError::ConfigurationError(format!("Failed to add certificate to store: {err}")))?;
    }

    let client_auth = WebPkiClientVerifier::builder(Arc::new(cert_store)).build().map_err(|err| ApplicationError::ConfigurationError(format!("Failed to create client verifier: {err}")))?;

    Ok(client_auth)
}

impl StartableServer for AppServer {
    /**
     * Start the server.
     *
     * # Returns
     * Ok if the server was started.
     *
     * # Errors
     * An error if the server could not be started.
     */
    fn start_server(&mut self) -> Result<Vec<JoinHandle<()>>, ApplicationError> {
        let handles = vec![];
        self.start_server_http()?;
        self.start_server_https()?;
        Ok(handles)
    }
}

struct AppState {
    server_configuration: ServerConfiguration,
    params: Vec<(String, String)>,
}

impl AppState {
    fn new(server_configuration: ServerConfiguration, params: Vec<(String, String)>) -> Self {
        AppState { server_configuration, params }
    }
}

#[cfg(test)]
mod test {

    use std::collections::HashMap;

    use super::*;

    /**
     * Verifying that the endpoints are found.
     */
    #[tokio::test(flavor = "multi_thread", worker_threads = 10)]
    async fn test_valid_endpoint() {
        let endpoint = EndpointConfiguration::new(Some("^\\/test$".to_string()), Some("GET".to_string()), Some("".to_string()), None).unwrap();
        assert!(is_valid_endpoint("/test", "GET", &endpoint, &Some("body".to_string())).unwrap());
    }

    /**
     * Create client verifier for mtls.
     */
    #[tokio::test(flavor = "multi_thread", worker_threads = 10)]
    async fn test_client_verifier() {
        let client_certificate = "tests/resources/client_cert.pem".to_owned();
        let client_auth = get_client_verifier(client_certificate);
        assert!(client_auth.is_ok());
    }

    /**
     * Create client verifier for mtls.
     */
    #[tokio::test(flavor = "multi_thread", worker_threads = 10)]
    async fn test_client_verifier_no_file() {
        let client_certificate = "tests/resources/no_file.pem".to_owned();
        let client_auth = get_client_verifier(client_certificate);
        assert!(client_auth.is_err());
    }

    /**
     * Verify get_supported_tls_versions method.
     */
    #[tokio::test(flavor = "multi_thread", worker_threads = 10)]
    async fn test_get_supported_tls_versions() {
        let supported_tls_versions = vec![TlsVersion::TLSv1_0, TlsVersion::TLSv1_1, TlsVersion::TLSv1_2];
        let protocol_versions = get_protocol_versions(&supported_tls_versions);
        assert_eq!(protocol_versions.len(), 3);
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 10)]
    async fn test_https_ssl_builder() {
        let https_config = HttpsConfiguration::new(
            "tests/resources/server_cert.pem".to_owned(),
            "tests/resources/server_key.pem".to_owned(),
            8080,
            None,
            vec![TlsVersion::TLSv1_0, TlsVersion::TLSv1_1, TlsVersion::TLSv1_2],
        );
        let ssl_builder = ssl_builder(&https_config);
        assert!(ssl_builder.is_ok());
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 10)]
    async fn test_https_ssl_builder_with_client_auth() {
        let https_config = HttpsConfiguration::new(
            "tests/resources/server_cert.pem".to_owned(),
            "tests/resources/server_key.pem".to_owned(),
            8080,
            Some("tests/resources/client_cert.pem".to_owned()),
            vec![TlsVersion::TLSv1_0, TlsVersion::TLSv1_1, TlsVersion::TLSv1_2],
        );
        let ssl_builder = ssl_builder(&https_config);
        assert!(ssl_builder.is_ok());
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 10)]
    async fn test_https_ssl_builder_no_file() {
        let https_config = HttpsConfiguration::new(
            "tests/resources/no_file.pem".to_owned(),
            "tests/resources/no_file.pem".to_owned(),
            8080,
            None,
            vec![TlsVersion::TLSv1_0, TlsVersion::TLSv1_1, TlsVersion::TLSv1_2],
        );
        let ssl_builder = ssl_builder(&https_config);
        assert!(ssl_builder.is_err());
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 10)]
    async fn test_generate_mock_response() {
        let mock_response = MockResponseConfiguration::new(Some("Test".to_owned()), String::from("200"), HashMap::new(), 0);
        let response = generate_mock_response(&mock_response, Vec::new()).await;
        assert!(response.is_ok());
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 10)]
    async fn test_get_client_no_proxy() {
        let route_configuration = RouteConfiguration::new("http://localhost:8080".to_owned(), None, None, false, false, false, None, None, None, None, None, Some(10));
        let client = get_client(&route_configuration);
        assert!(client.is_ok());
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 10)]
    async fn test_get_client_with_proxy() {
        let route_configuration = RouteConfiguration::new("http://localhost:8080".to_owned(), Some("http_//proxy.com:9999".to_owned()), None, false, false, false, None, None, None, None, None, Some(100));
        let client = get_client(&route_configuration);
        assert!(client.is_ok());
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 10)]
    async fn convert_params_test() {
        let params = vec![("param1".to_string(), "value1".to_string()), ("param2".to_string(), "value2".to_string())];
        let value = "This is a test with ${param1} and ${param2}";
        let result = convert_params(value, &params);
        assert_eq!(result, "This is a test with value1 and value2");
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 10)]
    async fn check_regexp_test() {
        assert!(check_regexp(None, &None).unwrap());
        assert!(check_regexp(None, &Some("123-45-6789".to_string())).unwrap());
        assert!(!check_regexp(Some(r"^\d{3}-\d{2}-\d{4}$".to_string()), &None).unwrap());
        assert!(check_regexp(Some(r"^\d{3}-\d{2}-\d{4}$".to_string()), &Some("123-45-6789".to_string())).unwrap());
        assert!(check_regexp(Some("^\\d{}-\\d{2}-\\d{4}".to_string()), &Some("123-45-6789".to_string())).is_err());
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 10)]
    async fn test_convert_params() {
        let params = vec![("param1".to_string(), "value1".to_string()), ("param2".to_string(), "value2".to_string())];
        let value = "This is a test with ${param1} and ${param2}";
        let result = convert_params(value, &params);
        assert_eq!(result, "This is a test with value1 and value2");
    }

}
