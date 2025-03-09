use apinae_lib::error::ApplicationError;

pub trait StartableServer {
    fn start_server(&mut self) -> Result<Vec<tokio::task::JoinHandle<()>>, ApplicationError>;
}
