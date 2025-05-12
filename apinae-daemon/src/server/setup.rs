use std::rc::Rc;

use apinae_lib::{config::SetupConfiguration, error::ApplicationError};
use tokio::sync::RwLock;

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
        ServerSetup { servers: Rc::new(RwLock::new(vec![])) }
    }

    /**
     * Setup with the specified configuration. This also initalizes the app servers.
     * 
     * # Arguments
     * * `setup_configuration` - The setup configuration to use.
     * * `args` - The command line arguments.
     * 
     * # Returns
     * Ok if the setup was successful.
     * 
     * # Errors
     * An error if the setup was not successful.
     */
    pub async fn setup(&mut self, setup_configuration: &SetupConfiguration, params: Vec<(String, String)>) -> Result<(), ApplicationError> {
        log::info!("Setting up setup with id {}", &setup_configuration.id);
        let servers: Vec<Box<dyn StartableServer>> =
        setup_configuration.servers.iter().map(|server_configuration| Box::new(AppServer::new(server_configuration.clone(), params.clone())) as Box<dyn StartableServer>).collect();
        let listeners: Vec<Box<dyn StartableServer>> = setup_configuration.listeners.iter().map(|tcp_listener_data| Box::new(AppListener::new(tcp_listener_data)) as Box<dyn StartableServer>).collect();
        self.servers.write().await.extend(servers);
        self.servers.write().await.extend(listeners);
        log::info!("Test setup complete");
        Ok(())
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
    async fn test_setup() {
        let mut server_setup = ServerSetup::new();
        let setup_configuration = SetupConfiguration {
            id: "test".to_string(),
            name: "Test".to_string(),
            description: "Test description".to_string(),
            servers: vec![ServerConfiguration { id: "test".to_string(), name: "Test server".to_string(), http_port: Some(8080), https_config: None, endpoints: vec![] }],
            listeners: vec![],
            params: None,
            predefined_params: None,
        };
        server_setup.setup(&setup_configuration, Vec::new()).await.unwrap();
        let servers = server_setup.start_servers().await;
        assert!(servers.is_ok());
    }
}
