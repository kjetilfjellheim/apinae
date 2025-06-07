use apinae_lib::error::ApplicationError;

/**
 * Trait implemented by server instances. Implementations used
 * are used by the http server and the tcp server.
 */
pub trait StartableServer {
    /**
     * Starts the server and returns a vector of JoinHandles
     * for the server tasks. 
     * 
     * # Returns
     * A vector of `JoinHandle<()>` representing the server tasks.
     * 
     * # Errors
     * If the server fails to start, an `ApplicationError` is returned.
     */
    fn start_server(&mut self) -> Result<Vec<tokio::task::JoinHandle<()>>, ApplicationError>;
}
