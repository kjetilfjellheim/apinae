mod args;
mod server;

use clap::Parser;

use apinae_lib::{
    config::{AppConfiguration, TestConfiguration},
    error::ApplicationError,
};
use args::Args;
use server::ServerSetup;

/**
 * The main function for the apinae-daemon application.
 *
 * This application is used to start a daemon.
 */
#[actix_web::main]
async fn main() -> Result<(), ApplicationError> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let args = Args::parse();
    let config = read_input_file(&args)?;
    init(args, config).await?;
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        wait_for_terminate().await?;
    }
}

/**
 * Read the input file with the specified arguments.
 *
 * # Arguments
 * `args`: The arguments to read the input file with.
 *
 * # Returns
 * The configuration read from the input file.
 *
 * # Errors
 * An error if the input file could not be read.
 */
fn read_input_file(args: &Args) -> Result<AppConfiguration, ApplicationError> {
    let config = AppConfiguration::load(&args.file)?;
    Ok(config)
}

/**
 * Initialize the application with the specified arguments and configuration.
 *
 * # Arguments
 * `args`: The arguments to initialize the application with.
 * `config`: The configuration to initialize the application with.
 *
 * # Returns
 * Ok if the application was initialized successfully.
 *
 * # Errors
 * An error if the daemon could not be started.
 * An error if the tests could not be listed.
 * An error if the id is missing.
 * An error if the test is not found.
 */
async fn init(args: Args, config: AppConfiguration) -> Result<(), ApplicationError> {
    if args.list {
        list_tests(&config);
    } else {
        start_daemon(args.id.as_ref(), &config).await?;
    }
    Ok(())
}

/**
 * List the available tests in the specified configuration.
 *
 * # Arguments
 * `config`: The configuration to list the tests from.
 *
 */
fn list_tests(config: &AppConfiguration) {
    println!("Available tests for configuration: {}", config.name);
    println!("ID\tName\tDescription");
    for test in &config.tests {
        println!("{}\t{}\t{}", test.id, test.name, test.description);
    }
}

/**
 * Start the daemon with the specified id.
 *
 * # Arguments
 * `id`: The id of the test to start.
 * `config`: The configuration to search for the test.
 *
 * # Returns
 * Ok if the daemon was started successfully.
 *
 * # Errors
 * An error if the test is not found.
 * An error if the id is missing.
 */
async fn start_daemon(
    id: Option<&String>,
    config: &AppConfiguration,
) -> Result<(), ApplicationError> {
    let Some(id) = id else {
        return Err(ApplicationError::MissingId("Missing id".to_string()));
    };
    let test = get_test(id, config)?;
    let mut server_setup = ServerSetup::new();
    server_setup.setup_test(test).await;
    server_setup.start_servers().await.map_err(|err| {
        ApplicationError::ServerStartUpError(format!("Server startup failed: {err}"))
    })?;
    server_setup.start_listeners().await.map_err(|err| {
        ApplicationError::ServerStartUpError(format!("Listener startup failed: {err}"))
    })?;    
    Ok(())
}

/**
 * Get the test with the specified id.
 *
 * # Arguments
 * `id`: The id of the test.
 * `config`: The configuration to search for the test.
 *
 * # Returns
 * The test with the specified id.
 *
 * # Errors
 * An error if the test is not found.
 */
fn get_test<'a>(
    id: &str,
    config: &'a AppConfiguration,
) -> Result<&'a TestConfiguration, ApplicationError> {
    let test = config.tests.iter().find(|test| test.id == id);
    match test {
        Some(test) => Ok(test),
        None => Err(ApplicationError::CouldNotFindTest(format!(
            "No test with id: {id}"
        ))),
    }
}

/**
 * Wait for the terminate signal.
 *
 * This function is only available on Linux.
 *
 * # Returns
 * Ok if the signal was received.
 *
 * # Errors
 * An error if the signals could not be initialized.
 */
#[cfg(unix)]
async fn wait_for_terminate() -> Result<(), ApplicationError> {
    use std::process::exit;

    use tokio::signal::unix::{signal, SignalKind};

    // Infos here:
    // https://www.gnu.org/software/libc/manual/html_node/Termination-Signals.html
    let mut signal_terminate = signal(SignalKind::terminate()).map_err(|err| {
        ApplicationError::ServerStartUpError(format!("Failed to terminate: {err}"))
    })?;
    let mut signal_interrupt = signal(SignalKind::interrupt()).map_err(|err| {
        ApplicationError::ServerStartUpError(format!("Failed to terminate: {err}"))
    })?;

    tokio::select! {
        _ = signal_terminate.recv() => exit(0),
        _ = signal_interrupt.recv() => exit(0),
    };
}

/**
 * Wait for the terminate signal.
 *
 * This function is only available on Windows.
 *
 * # Returns
 * Ok if the signal was received.
 *
 * # Errors
 * An error if the signals could not be initialized.
 */
#[cfg(windows)]
async fn wait_for_terminate() -> Result<(), ApplicationError> {
    use std::process::exit;

    use tokio::signal::windows;

    // Infos here:
    // https://learn.microsoft.com/en-us/windows/console/handlerroutine
    let mut signal_c =
        windows::ctrl_c().map_err(|err| ApplicationError::ServerStartUpError(err.to_string()))?;
    let mut signal_break = windows::ctrl_break()
        .map_err(|err| ApplicationError::ServerStartUpError("Failed to terminate: {err}"))?;
    let mut signal_close = windows::ctrl_close()
        .map_err(|err| ApplicationError::ServerStartUpError("Failed to terminate: {err}"))?;
    let mut signal_shutdown = windows::ctrl_shutdown()
        .map_err(|err| ApplicationError::ServerStartUpError("Failed to terminate: {err}"))?;

    tokio::select! {
        _ = signal_c.recv() => exit(0),
        _ = signal_break.recv() => exit(0),
        _ = signal_close.recv() => exit(0),
        _ = signal_shutdown.recv() => exit(0),
    };
}

#[cfg(test)]
mod test {

    use super::*;

    use apinae_lib::config::AppConfiguration;

    #[tokio::test(flavor = "multi_thread", worker_threads = 10)]
    async fn test_get_test() {
        let config = AppConfiguration::load("./tests/resources/test_http_mock.json").unwrap();
        assert!(get_test("1", &config).is_ok());
        assert!(get_test("2", &config).is_err());
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 10)]
    async fn test_start_daemon() {
        let config: AppConfiguration =
            AppConfiguration::load("./tests/resources/test_http_mock.json").unwrap();
        let _ = start_daemon(Some(&"1".to_string()), &config).await.is_ok();
        assert!(start_daemon(Some(&"2".to_string()), &config).await.is_err());
        assert!(start_daemon(None, &config).await.is_err());
    }
}
