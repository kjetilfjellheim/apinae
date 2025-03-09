use std::rc::Rc;

use apinae_lib::{
    config::
        TestConfiguration
    ,
    error::ApplicationError,
};
use tokio::
    sync::RwLock
;

use super::{common::StartableServer, http::AppServer, tcp::AppListener};

/**
 * The `ServerSetup` struct is used to start and stop servers.
 */
pub struct ServerSetup {
    servers: Rc<RwLock<Vec<Box<dyn StartableServer>>>>,
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
            servers: Rc::new(RwLock::new(vec![])),
        }
    }

    /**
     * Setup the test with the specified configuration. This also initalizes the app servers.
     */
    pub async fn setup_test(&mut self, test_configuration: &TestConfiguration) {
        log::info!("Setting up test with id {}", test_configuration.id);
        let servers: Vec<Box<dyn StartableServer>> = test_configuration
            .servers
            .iter()
            .map(|server_configuration| Box::new(AppServer::new(server_configuration.clone())) as Box<dyn StartableServer>)
            .collect();
        let listeners: Vec<Box<dyn StartableServer>> = test_configuration
            .listeners
            .iter()
            .map(|tcp_listener_data| Box::new(AppListener::new(tcp_listener_data)) as Box<dyn StartableServer>)
            .collect();
        self.servers.write().await.extend(servers);
        self.servers.write().await.extend(listeners);
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
            handles.push(server.start_server()?);
        }
        Ok(())
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use apinae_lib::config::ServerConfiguration;

    #[tokio::test]
    async fn test_setup_test() {
        let mut server_setup = ServerSetup::new();
        let test_configuration = TestConfiguration {
            id: "test".to_string(),
            name: "Test".to_string(),
            description: "Test description".to_string(),
            servers: vec![
                ServerConfiguration {
                    id: "test".to_string(),
                    name: "Test server".to_string(),
                    http_port: Some(8080),
                    https_config: None,
                    endpoints: vec![],
                },
            ],
            listeners: vec![],
        };
        server_setup.setup_test(&test_configuration).await;
        let servers = server_setup.start_servers().await;
        assert!(servers.is_ok());        
    }
}