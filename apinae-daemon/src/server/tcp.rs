use std::{
    fs::File,
    io::{BufReader, Read},
    time::Duration,
};

use apinae_lib::{
    config::{CloseConnectionWhen, TcpListenerData},
    error::ApplicationError,
};
use log::{error, info};
use tokio::{io::Interest, task::JoinHandle};

use super::common::StartableServer;

/**
 * The `AppListener` struct is used to configure and start the listener.
 */
pub struct AppListener {
    // Server configuration
    tcp_listener: TcpListenerData,
}

impl AppListener {
    /**
     * Create a new `AppListener`.
     *
     * # Arguments
     * `tcp_listener`: The TCP listener configuration.
     *
     * # Returns
     * The created `AppListener`.
     */
    pub fn new(tcp_listener: &TcpListenerData) -> Self {
        AppListener { tcp_listener: tcp_listener.clone() }
    }

    /**
     * Start the listener.
     *
     * # Returns
     * Ok if the listener was started.
     *
     * # Errors
     * An error if the listener could not be started.
     *
     */
    pub async fn start_listener(&self) -> Result<(), ApplicationError> {
        let server = self.bind_listener().await?;
        let tcp_listener_data = self.tcp_listener.clone();

        tokio::spawn(async move {
            loop {
                let Some(stream) = Self::wait_for_accept(&server, &tcp_listener_data).await else {
                    continue;
                };
                let tcp_listener_data = tcp_listener_data.clone();
                let _ = tokio::spawn(async move {
                    let _ = Self::handle_tcp_stream(stream, tcp_listener_data).await.map_err(|err| {
                        error!("Error handling tcp connection: {err}");
                    });
                    info!("Connection closed");
                })
                .await;
            }
        });
        Ok(())
    }

    /**
     * Bind the listener.
     *
     * # Returns
     * The bound listener.
     *
     * # Errors
     *  An error if the listener could not be bound.
     */
    async fn bind_listener(&self) -> Result<tokio::net::TcpListener, ApplicationError> {
        let server = tokio::net::TcpListener::bind(("127.0.0.1", self.tcp_listener.port)).await.map_err(|err| ApplicationError::ServerStartUpError(format!("Failed to create tcp listener: {err}")))?;
        log::info!("Listening on: {}", self.tcp_listener.port);
        Ok(server)
    }

    /**
     * Write the data to the output stream.
     *
     * # Arguments
     * `stream`: The output stream.
     * `data`: The data to write.
     *
     */
    fn output_string_data(stream: &mut tokio::net::TcpStream, data: &String) {
        info!("Sending: {data}");
        let _ = stream.try_write(data.as_bytes()).map_err(|err| {
            error!("Failed to write data: {err}");
        });
    }

    /**
     * Write the file data to the output stream.
     *
     * # Arguments
     * `stream`: The output stream.
     * `file`: The file to write.
     *
     */
    fn output_file_data(stream: &mut tokio::net::TcpStream, file: &String) {
        match File::open(file) {
            Ok(file) => {
                let mut output = BufReader::new(file);
                let mut buffer = Vec::new();
                info!("Sending: {}", String::from_utf8_lossy(&buffer));
                let _ = output.read_to_end(&mut buffer).map_err(|err| {
                    error!("Failed to read file: {err}");
                });
                let _ = stream.try_write(buffer.as_mut()).map_err(|err| {
                    error!("Failed to write file: {err}");
                });
            }
            Err(err) => {
                error!("Failed to open file: {err}");
            }
        }
    }

    /**
     * Wait for the accept.
     *
     * # Arguments
     * `server`: The server.
     *
     * # Returns
     * The accepted stream.
     *
     */
    async fn wait_for_accept(tcp_listener: &tokio::net::TcpListener, tcp_listener_data: &TcpListenerData) -> Option<tokio::net::TcpStream> {
        if !tcp_listener_data.accept {
            tokio::time::sleep(Duration::from_secs(1)).await;
            return None;
        }
        let (stream, _) = match tcp_listener.accept().await {
            Ok(stream) => stream,
            Err(err) => {
                error!("Failed to accept connection: {err}");
                return None;
            }
        };
        log::debug!("Accepted connection");
        Some(stream)
    }

    /**
     * Handle the TCP stream.
     *
     * # Arguments
     * `stream`: The TCP stream.
     * `tcp_listener_data`: The TCP listener data.
     *
     * # Returns
     * Ok if the stream was handled.
     *
     * # Errors
     * An error if the stream could not be handled.
     *
     */
    async fn handle_tcp_stream(mut stream: tokio::net::TcpStream, tcp_listener_data: TcpListenerData) -> Result<(), ApplicationError> {
        let mut written = true;

        loop {
            tokio::time::sleep(Duration::from_micros(10)).await;

            let ready = stream.ready(Interest::READABLE | Interest::WRITABLE).await.map_err(|err| ApplicationError::ServerStartUpError(format!("Failed to get ready: {err}")))?;
            log::debug!("Ready: {ready:?}");
            if ready.is_read_closed() || ready.is_write_closed() {
                return Ok(());
            }

            if tcp_listener_data.close_connection == CloseConnectionWhen::BeforeRead {
                return Ok(());
            }

            if ready.is_readable() {
                loop {
                    let mut buffer = vec![];
                    let _ = stream.try_read_buf(&mut buffer);
                    if !buffer.is_empty() {
                        info!("Received: {:?}", String::from_utf8_lossy(&buffer));
                    }
                    if buffer.is_empty() {
                        written = false;
                        break;
                    }
                }
            }

            if tcp_listener_data.close_connection == CloseConnectionWhen::AfterRead {
                return Ok(());
            }

            if ready.is_writable() && !written {
                if let Some(delay_write_ms) = tcp_listener_data.delay_write_ms {
                    tokio::time::sleep(Duration::from_millis(delay_write_ms)).await;
                }
                if let Some(data) = tcp_listener_data.data.as_ref() {
                    Self::output_string_data(&mut stream, data);
                } else if let Some(file) = tcp_listener_data.file.as_ref() {
                    Self::output_file_data(&mut stream, file);
                }

                if tcp_listener_data.close_connection == CloseConnectionWhen::AfterResponse {
                    return Ok(());
                }
                written = true;
            }
        }
    }
}

impl StartableServer for AppListener {
    fn start_server(&mut self) -> Result<Vec<JoinHandle<()>>, ApplicationError> {
        let tcp_listener_data = self.tcp_listener.clone();
        let handle = tokio::spawn(async move {
            let listener = AppListener { tcp_listener: tcp_listener_data };
            let _ = listener.start_listener().await.map_err(|err| {
                error!("Failed to start listener: {err}");
            });
        });
        Ok(vec![handle])
    }
}
