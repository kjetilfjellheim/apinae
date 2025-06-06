/**
 * This module contains the error type for the application.
 */
#[derive(Debug, PartialEq)]
pub enum ApplicationError {
    FileError(String),
    MissingId(String),
    CouldNotFind(String),
    ConfigurationError(String),
    ServerStartUpError(String),
    RoutingError(String),
}

/**
 * Convert the error to a string.
 */
impl std::fmt::Display for ApplicationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ApplicationError::FileError(err) => write!(f, "File error: {err}"),
            ApplicationError::MissingId(err) => write!(f, "Missing id: {err}"),
            ApplicationError::CouldNotFind(err) => write!(f, "Could not find: {err}"),
            ApplicationError::ConfigurationError(err) => write!(f, "Configuration error: {err}"),
            ApplicationError::ServerStartUpError(err) => {
                write!(f, "Server start up error: {err}")
            }
            ApplicationError::RoutingError(err) => write!(f, "Routing error: {err}"),
        }
    }
}
