use std::{fs::File, io::BufReader, sync::Arc};

use actix_web::{http::StatusCode, web, App, HttpRequest, HttpResponse, HttpServer};
use regex::Regex;
use reqwest::Method;
use rustls::{
    pki_types::PrivateKeyDer,
    server::{danger::ClientCertVerifier, WebPkiClientVerifier},
    RootCertStore, ServerConfig,
};
use rustls_pemfile::{certs, pkcs8_private_keys};
use testit_lib::{
    config::{
        EndpointConfiguration, HttpsConfiguration, MockResponseConfiguration, RouteConfiguration,
        ServerConfiguration, TestConfiguration,
    },
    error::ApplicationError,
};
use tokio::sync::RwLock;

const QUERYPARAMSEPARATOR: char = '&';
const KEYVALUESEPARATOR: char = '=';

/**
 * The `ServerSetup` struct is used to start and stop servers.
 */
pub struct ServerSetup {
    servers: Arc<RwLock<Vec<AppServer>>>,
}

impl ServerSetup {
    /**
     * Create a new `ServerSetup`.
     *
     * # Returns
     * The created `ServerSetup`.
     */
    pub fn new() -> Self {
        ServerSetup {
            servers: Arc::new(RwLock::new(vec![])),
        }
    }

    /**
     * Setup the test with the specified configuration. This also initalizes the app servers.
     */
    pub async fn setup_test(&mut self, test_configuration: &TestConfiguration) {
        let servers: Vec<AppServer> = test_configuration
            .servers
            .iter()
            .map(|server_configuration| AppServer::new(server_configuration.clone()))
            .collect();
        self.servers.write().await.extend(servers);
    }

    /**
     * Start the servers.
     *
     * # Returns
     * Ok if the servers were started.
     *
     * # Errors
     * An error if the servers could not be started.
     */
    pub async fn start_servers(&mut self) -> Result<(), ApplicationError> {
        let mut handles = vec![];
        for server in self.servers.write().await.iter_mut() {
            handles.push(server.start_server_http()?);
            handles.push(server.start_server_https()?);
        }
        Ok(())
    }
}

/**
 * The `AppServer` struct is used to configure and start the server.
 */
struct AppServer {
    // Server configuration
    server_configuration: ServerConfiguration,
}

impl AppServer {
    /**
     * Create a new `AppServer`.
     *
     * # Arguments
     * `server_configuration`: The server configuration.
     *
     * # Returns
     * The created `AppServer`.
     */
    fn new(server_configuration: ServerConfiguration) -> Self {
        AppServer {
            server_configuration,
        }
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
    fn start_server_http(&mut self) -> Result<(), ApplicationError> {
        if let Some(http_port) = self.server_configuration.http_port {
            let appstate = web::Data::new(self.server_configuration.clone());
            let server = HttpServer::new(move || {
                App::new()
                    .app_data(appstate.clone())
                    .default_service(web::to(request_handler))
            })
            .bind(("127.0.0.1", http_port))
            .map_err(|err| ApplicationError::ServerStartUpError(err.to_string()))?;
            let server = server.workers(2).run();
            tokio::spawn(async move {
                match server.await {
                    Ok(()) => {}
                    Err(err) => eprintln!("{err}"),
                }
            });
        }
        Ok(())
    }

    /**
     * Start the server with HTTPS.
     *
     * # Returns
     * Ok if the server was started.
     *
     * # Errors
     * An error if the server could not be started.
     */
    fn start_server_https(&self) -> Result<(), ApplicationError> {
        let config = self.server_configuration.clone();
        if let Some(https_config) = config.https_config {
            let ssl_builder = ssl_builder(&https_config)?;
            let appstate = web::Data::new(self.server_configuration.clone());
            let server = HttpServer::new(move || {
                App::new()
                    .app_data(appstate.clone())
                    .default_service(web::to(request_handler))
            })
            .bind_rustls_0_23(
                "127.0.0.1:".to_owned() + https_config.https_port.to_string().as_str(),
                ssl_builder,
            )
            .map_err(|err| ApplicationError::ServerStartUpError(err.to_string()))?;
            let server = server.workers(2).run();
            tokio::spawn(async move {
                match server.await {
                    Ok(()) => {}
                    Err(err) => eprintln!("{err}"),
                }
            });
        }
        Ok(())
    }
}

/**
 * Handle the request.
 *
 * # Arguments
 * `server_configuration`: The server configuration.
 * `req`: The request.
 *
 * # Returns
 * The response.
 */
async fn request_handler(
    server_configuration: web::Data<ServerConfiguration>,
    req: HttpRequest,
    payload: Option<web::Payload>,
) -> HttpResponse {
    for endpoint in &server_configuration.endpoints {
        match is_valid_endpoint(&req, endpoint) {
            Ok(true) => match handle_endpoint(endpoint, req, payload).await {
                Ok(response) => return response,
                Err(err) => {
                    eprintln!("{err}");
                    return HttpResponse::NotImplemented().body("Not implemented");
                }
            },
            Ok(false) => {}
            Err(err) => return HttpResponse::ServiceUnavailable().body(err.to_string()),
        }
    }
    HttpResponse::NotImplemented().body("Not implemented")
}

/**
 * Check if the request is a valid endpoint.
 *
 * # Arguments
 * `request`: The request.
 * `endpoint`: The endpoint configuration.
 *
 * # Returns
 * True if the request is a valid endpoint.
 *
 * # Errors
 * An error if the endpoint is invalid.
 */
fn is_valid_endpoint(
    request: &HttpRequest,
    endpoint: &EndpointConfiguration,
) -> Result<bool, ApplicationError> {
    let regexp = Regex::new(&endpoint.endpoint)
        .map_err(|err| ApplicationError::ConfigurationError(err.to_string()))?;
    Ok(regexp.is_match(request.uri().path())
        && request.method().as_str() == endpoint.method.as_str())
}

/**
 * Handle the endpoint.
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
async fn handle_endpoint(
    endpoint: &EndpointConfiguration,
    req: HttpRequest,
    payload: Option<web::Payload>,
) -> Result<HttpResponse, ApplicationError> {
    if let Some(mock_response) = &endpoint.mock_response {
        std::thread::sleep(std::time::Duration::from_millis(mock_response.delay));
        return generate_mock_response(mock_response);
    }
    if let Some(route_configuration) = &endpoint.route {
        return route_request(route_configuration, req, payload).await;
    }
    Ok(HttpResponse::NotImplemented().body("Not implemented"))
}

/**
 * Route the request.
 *
 * # Arguments
 * `route_configuration`: The route configuration.
 * `req`: The request.
 *
 * # Returns
 * The response.
 */
async fn route_request(
    route_configuration: &RouteConfiguration,
    req: HttpRequest,
    payload: Option<web::Payload>,
) -> Result<HttpResponse, ApplicationError> {
    let mut url = route_configuration.endpoint.clone();
    url.push_str(req.path());

    let request = get_request(req, payload, url).await?;

    let client = get_client(route_configuration)?;

    let response = client
        .execute(request)
        .await
        .map_err(|err| ApplicationError::RoutingError(err.to_string()))?;

    let response = get_response(response).await?;

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
    let mut response_builder = HttpResponse::build(
        StatusCode::from_u16(response.status().as_u16())
            .map_err(|err| ApplicationError::RoutingError(err.to_string()))?,
    );
    for (key, value) in response.headers() {
        response_builder.append_header((
            key.as_str(),
            value
                .to_str()
                .map_err(|err| ApplicationError::RoutingError(err.to_string()))?,
        ));
    }
    let body = response
        .text()
        .await
        .map_err(|err| ApplicationError::RoutingError(err.to_string()))?;
    
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
    let client = match &route_configuration.proxy_url {
        Some(proxy) => {
            let reqwest_proxy = reqwest::Proxy::all(proxy.clone())
                .map_err(|err| ApplicationError::RoutingError(err.to_string()))?;
            reqwest::Client::builder()
                .proxy(reqwest_proxy)
                .build()
                .map_err(|err| ApplicationError::RoutingError(err.to_string()))?
        }
        None => reqwest::Client::builder()
            .build()
            .map_err(|err| ApplicationError::RoutingError(err.to_string()))?,
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
async fn get_request(req: HttpRequest, payload: Option<web::Payload>, url: String) -> Result<reqwest::Request, ApplicationError> {
    let mut request_builder = reqwest::Client::new().request(
        Method::from_bytes(req.method().as_str().as_bytes())
            .map_err(|err| ApplicationError::RoutingError(err.to_string()))?,
        url.as_str(),
    );
    request_builder = match req.version() {
        actix_web::http::Version::HTTP_09 => request_builder.version(reqwest::Version::HTTP_09),
        actix_web::http::Version::HTTP_10 => request_builder.version(reqwest::Version::HTTP_10),
        actix_web::http::Version::HTTP_11 => request_builder.version(reqwest::Version::HTTP_11),
        actix_web::http::Version::HTTP_2 => request_builder.version(reqwest::Version::HTTP_2),
        actix_web::http::Version::HTTP_3 => request_builder.version(reqwest::Version::HTTP_3),
        _ => {
            return Err(ApplicationError::RoutingError(
                "Invalid version".to_string(),
            ))
        }
    };
    for (key, value) in req.headers() {
        let value = value
            .to_str()
            .map_err(|err| ApplicationError::RoutingError(err.to_string()))?;
        request_builder = request_builder.header(key.as_str(), value);
    }
    if let Some(payload) = payload {
        let bytes = payload
            .to_bytes()
            .await
            .map_err(|err| ApplicationError::RoutingError(err.to_string()))?;
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
    let request = request_builder
        .build()
        .map_err(|err| ApplicationError::RoutingError(err.to_string()))?;
    Ok(request)
}

/**
 * Generate a mock response.
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
fn generate_mock_response(
    mock_response: &MockResponseConfiguration,
) -> Result<HttpResponse, ApplicationError> {
    let mut response_builder: actix_web::HttpResponseBuilder = HttpResponse::build(
        StatusCode::from_u16(mock_response.status)
            .map_err(|err| ApplicationError::ConfigurationError(err.to_string()))?,
    );
    for (key, value) in &mock_response.headers {
        response_builder.append_header((key.as_str(), value.as_str()));
    }
    if let Some(response) = &mock_response.response {
        return Ok(response_builder.body(response.clone()));
    }
    Ok(response_builder.finish())
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
    let config_builder = match https_config.clone().client_certificate {
        Some(client_certificate) => {
            let client_auth = get_client_verifier(client_certificate)?;
            ServerConfig::builder().with_client_cert_verifier(client_auth)
        }
        None => ServerConfig::builder().with_no_client_auth(),
    };

    let cert_file = &mut BufReader::new(
        File::open(https_config.clone().server_certificate)
            .map_err(|err| ApplicationError::ConfigurationError(err.to_string()))?,
    );
    let key_file = &mut BufReader::new(
        File::open(https_config.clone().private_key)
            .map_err(|err| ApplicationError::ConfigurationError(err.to_string()))?,
    );

    let cert_chain = certs(cert_file)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|err| ApplicationError::ConfigurationError(err.to_string()))?;
    let mut keys = pkcs8_private_keys(key_file)
        .map(|key| key.map(PrivateKeyDer::Pkcs8))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|err| ApplicationError::ConfigurationError(err.to_string()))?;
    let config = config_builder
        .with_single_cert(cert_chain, keys.remove(0))
        .map_err(|err| ApplicationError::ConfigurationError(err.to_string()))?;
    Ok(config)
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
fn get_client_verifier(
    client_certificate: String,
) -> Result<Arc<dyn ClientCertVerifier>, ApplicationError> {
    let cert_file = &mut BufReader::new(
        File::open(client_certificate)
            .map_err(|err| ApplicationError::ConfigurationError(err.to_string()))?,
    );
    let cert_chain = certs(cert_file)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|err| ApplicationError::ConfigurationError(err.to_string()))?;

    let mut cert_store = RootCertStore::empty();

    for cert in cert_chain {
        cert_store
            .add(cert)
            .map_err(|err| ApplicationError::ConfigurationError(err.to_string()))?;
    }

    let client_auth = WebPkiClientVerifier::builder(Arc::new(cert_store))
        .build()
        .map_err(|err| ApplicationError::ConfigurationError(err.to_string()))?;

    Ok(client_auth)
}

#[cfg(test)]
mod test {
    use std::{collections::HashMap, fs::File, io::Read, thread, time::Duration};

    use super::*;

    /**
     * Verifying that the server can be started.
     * TODO:Move this to integration tests
     */
    #[tokio::test(flavor = "multi_thread", worker_threads = 10)]
    async fn test_server_start() {
        let test_configuration = TestConfiguration {
            servers: vec![
                ServerConfiguration {
                    name: "test".to_string(),
                    http_port: Some(8080),
                    id: "test".to_string(),
                    endpoints: vec![],
                    https_config: None,
                },
                ServerConfiguration {
                    name: "test".to_string(),
                    http_port: Some(8081),
                    id: "test".to_string(),
                    endpoints: vec![],
                    https_config: None,
                },
            ],
            name: "test".to_string(),
            description: "test".to_string(),
            id: "test".to_string(),
        };
        let mut server_setup = ServerSetup::new();
        server_setup.setup_test(&test_configuration).await;
        let result = server_setup.start_servers().await;
        assert!(result.is_ok());
        thread::sleep(Duration::from_secs(1));
        let res = reqwest::get("http://localhost:8080").await.unwrap();
        assert_eq!(res.status(), 501);
        let res = reqwest::get("http://localhost:8081").await.unwrap();
        assert_eq!(res.status(), 501);
    }

    /**
     * Verifying that the endpoints are found.
     */
    #[tokio::test(flavor = "multi_thread", worker_threads = 10)]
    async fn test_endpoint_ok() {
        let test_configuration = TestConfiguration::new(
            "test".to_string(),
            "test".to_string(),
            vec![ServerConfiguration::new(
                "test".to_string(),
                Some(8082),
                vec![
                    EndpointConfiguration::new(
                        "/test2".to_string(),
                        "GET".to_string(),
                        None,
                        Some(MockResponseConfiguration::new(
                            Some("{}".to_string()),
                            400,
                            HashMap::new(),
                            1000,
                        )),
                        None,
                    ),
                    EndpointConfiguration::new(
                        "/test".to_string(),
                        "GET".to_string(),
                        None,
                        Some(MockResponseConfiguration::new(
                            Some("{}".to_string()),
                            200,
                            HashMap::new(),
                            1000,
                        )),
                        None,
                    ),
                ],
                None,
            )],
        );
        let mut server_setup = ServerSetup::new();
        server_setup.setup_test(&test_configuration).await;
        let result = server_setup.start_servers().await;
        assert!(result.is_ok());
        thread::sleep(Duration::from_secs(1));
        let res = reqwest::get("http://localhost:8082/test").await.unwrap();
        assert_eq!(res.status(), 200);
        assert_eq!(res.text().await.unwrap(), "{}".to_string());
        let res = reqwest::get("http://localhost:8082").await.unwrap();
        assert_eq!(res.status(), 501);
    }

    /**
     * Verifying https server.
     */
    #[tokio::test(flavor = "multi_thread", worker_threads = 10)]
    async fn test_https() {
        let server_cert_path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/..",
            "/testit-daemon/test/resources/https_test/server_cert.pem"
        )
        .to_owned();
        let server_key_path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/..",
            "/testit-daemon/test/resources/https_test/server_key.pem"
        )
        .to_owned();
        let https_config =
            HttpsConfiguration::new(server_cert_path.clone(), server_key_path, 8084, None);
        let test_configuration = TestConfiguration::new(
            "test".to_string(),
            "test".to_string(),
            vec![ServerConfiguration::new(
                "test".to_string(),
                None,
                vec![EndpointConfiguration::new(
                    "/".to_string(),
                    "GET".to_string(),
                    None,
                    Some(MockResponseConfiguration::new(
                        Some("{}".to_string()),
                        200,
                        HashMap::new(),
                        1000,
                    )),
                    None,
                )],
                Some(https_config),
            )],
        );
        let mut server_setup = ServerSetup::new();
        server_setup.setup_test(&test_configuration).await;
        let result = server_setup.start_servers().await;
        thread::sleep(Duration::from_secs(1));
        assert!(result.is_ok());
        let mut buf = Vec::new();
        File::open(server_cert_path)
            .unwrap()
            .read_to_end(&mut buf)
            .unwrap();
        let cert = reqwest::Certificate::from_pem(&buf).unwrap();
        let client = reqwest::Client::builder()
            .add_root_certificate(cert)
            .danger_accept_invalid_hostnames(true)
            .build()
            .unwrap();
        let res = client.get("https://localhost:8084").send().await.unwrap();
        assert_eq!(res.status(), 200);
    }
}
