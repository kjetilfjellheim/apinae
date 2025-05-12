mod args;
mod server;

use clap::Parser;

use apinae_lib::{
    config::{AppConfiguration, SetupConfiguration},
    error::ApplicationError,
};
use args::Args;
use server::setup::ServerSetup;

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
    Ok(())
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
 * An error if the setups could not be listed.
 * An error if the id is missing.
 * An error if the setup is not found.
 */
async fn init(args: Args, config: AppConfiguration) -> Result<(), ApplicationError> {
    if args.list {
        list_setups(&config);
    } else if args.list_predefined_sets {
        list_predefined_sets(&config, args.id)?;
    } else if args.list_params {
        list_params(&config, args.id)?;
    } else {
        start_daemon(args, &config).await?;
    }
    Ok(())
}

/**
 * List the available predefined sets for the specified setup.
 *
 * # Arguments
 * `config`: The configuration to list the predefined sets from.
 * `setup_id`: The id of the setup to list the predefined sets for.
 *
 * # Returns
 * Ok if the predefined sets were listed successfully.
 *
 * # Errors
 * An error if the setup is not found.
 */
fn list_predefined_sets(config: &AppConfiguration, setup_id: Option<String>)  -> Result<(), ApplicationError> {
    if let Some(setup_id) = setup_id {
        let setup = get_setup(setup_id.as_str(), config)?;
        println!("Available predefined sets for configuration: {}", config.name);
        println!("Name");
        if let Some(predefined_sets) = &setup.predefined_params {
            for predefined_set in predefined_sets {
                println!("{}", predefined_set.name);
            }
        } else {
            println!("No predefined sets available for setup: {}", setup.name);
        }    
    } else {
        println!("No setup id specified.");
    }
    Ok(())
}

/**
 * List the available parameters for the specified setup.
 *
 * # Arguments
 * `config`: The configuration to list the parameters from.
 * `setup_id`: The id of the setup to list the parameters for.
 * 
 * # Returns
 * Ok if the parameters were listed successfully.
 * 
 * # Errors
 * An error if the setup is not found.
 *
 */
fn list_params(config: &AppConfiguration, setup_id: Option<String>) -> Result<(), ApplicationError> {
    if let Some(setup_id) = setup_id {
        let setup = get_setup(setup_id.as_str(), config)?;
        if let Some(params) = &setup.params {
            println!("Available parameters for setup: {}", setup.name);
            println!("Name");
            for param in params {
                println!("{param}");
            }
        } else {
            println!("No parameters available for setup: {}", setup.name);
        }
    } else {
        println!("No setup id specified.");
    }
    Ok(())
}

/**
 * List the available setups in the specified configuration.
 *
 * # Arguments
 * `config`: The configuration to list the setups from.
 *
 */
fn list_setups(config: &AppConfiguration) {
    println!("Available setups for configuration: {}", config.name);
    println!("ID\tName\tDescription");
    for setup in &config.setups {
        println!("{}\t{}\t{}", setup.id, setup.name, setup.description);
    }
}

/**
 * Start the daemon with the specified id.
 *
 * # Arguments
 * `args`: Arguments to start the daemon with.
 * `config`: The configuration to search for the setup.
 *
 * # Returns
 * Ok if the daemon was started successfully.
 *
 * # Errors
 * An error if the setup is not found.
 * An error if the id is missing.
 */
async fn start_daemon(args: Args, config: &AppConfiguration) -> Result<(), ApplicationError> {
    let setup_id = args.clone().id.ok_or(ApplicationError::CouldNotFind("Missing id".to_string()))?;
    log::info!("Starting daemon with id: {setup_id} and args {args:?}");
    let setup = get_setup(setup_id.as_str(), config)?;
    let params = validate_parameters(setup, &args)?;
    let mut server_setup = ServerSetup::new();
    server_setup.setup(setup, params).await?;
    server_setup.start_servers().await.map_err(|err| ApplicationError::ServerStartUpError(format!("Server startup failed: {err}")))?;
    if args.verify {
        return Ok(());
    }
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;        
        wait_for_terminate().await?;                
    }
}

/**
 * Validate the parameters for the test.
 * All test parameters must be specified in the arguments.
 *
 * # Arguments
 * `setup`: The setup to validate the parameters for.
 * `args`: The application arguments to validate the parameters with.
 *
 * # Returns
 * Ok if the parameters are valid.
 *
 * # Errors
 * An error if the parameters are invalid.
 */
fn validate_parameters(setup: &SetupConfiguration, args: &Args) -> Result<Vec<(String, String)>, ApplicationError> {
    let setup_params_required = &setup.params.clone().unwrap_or_default();
    let mut setup_params = Vec::new();
    if let Some(predefined_set_name) = &args.predefined_set {
        log::info!("Using predefined set {predefined_set_name}");
        let predefined_set = get_predefined_set(setup, predefined_set_name)?;
        setup_params.extend(predefined_set.values.iter().map(|(k, v)| (k.clone(), v.clone())));
    }    
    for (key, value) in &args.param {
        if setup_params_required.iter().any(|param| param.eq(key)) {
            setup_params.push((key.clone(), value.clone()));
        } else {
            return Err(ApplicationError::CouldNotFind(format!("Parameter {key} not found in setup {}", setup.id)));
        }
    }
    if setup_params_required.is_empty() {
        return Ok(Vec::new());
    }    
    for required_param in setup_params_required {
        if !setup_params.iter().any(|(key, _)| key.eq(required_param)) {
            return Err(ApplicationError::CouldNotFind(format!("Missing parameter: {required_param}")));
        }
    }
    if setup_params_required.len() != setup_params.len() {
        return Err(ApplicationError::CouldNotFind(format!("Missing parameters: {setup_params_required:?}")));
    }
    Ok(setup_params)
}

/**
 * Get the setup with the specified id.
 *
 * # Arguments
 * `id`: The id of the setup.
 * `config`: The configuration to search for the setup.
 *
 * # Returns
 * The setup with the specified id.
 *
 * # Errors
 * An error if the test is not found.
 */
fn get_setup<'a>(id: &str, config: &'a AppConfiguration) -> Result<&'a SetupConfiguration, ApplicationError> {
    let setup = config.setups.iter().find(|setup| setup.id == id);
    match setup {
        Some(setup) => Ok(setup),
        None => Err(ApplicationError::CouldNotFind(format!("No setup with id: {id}"))),
    }
}

/**
 * Get predefined set for a setup.
 *
 * # Arguments
 * `setup_configuration`: The setup configuration to get the predefined set from.
 * `predefined_set_name`: The name of the predefined set to get.
 *
 * # Returns
 * Ok if the predefined set was found.
 *
 * # Errors
 * An error if the predefined set was not found.
 */
fn get_predefined_set(setup_configuration: &SetupConfiguration, predefined_set_name: &String) -> Result<apinae_lib::config::PredefinedSet, ApplicationError> {
    let predefined_set = setup_configuration.clone().predefined_params
        .and_then(|f| f.iter().find(|p| p.name == *predefined_set_name).cloned())
        .ok_or_else(|| ApplicationError::CouldNotFind("Predefined set not found".to_string()))?;
    Ok(predefined_set)
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

    let mut signal_terminate = signal(SignalKind::terminate()).map_err(|err| ApplicationError::ServerStartUpError(format!("Failed to terminate: {err}")))?;
    let mut signal_interrupt = signal(SignalKind::interrupt()).map_err(|err| ApplicationError::ServerStartUpError(format!("Failed to terminate: {err}")))?;

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
    let mut signal_c = windows::ctrl_c().map_err(|err| ApplicationError::ServerStartUpError(err.to_string()))?;
    let mut signal_break = windows::ctrl_break().map_err(|err| ApplicationError::ServerStartUpError("Failed to terminate: {err}"))?;
    let mut signal_close = windows::ctrl_close().map_err(|err| ApplicationError::ServerStartUpError("Failed to terminate: {err}"))?;
    let mut signal_shutdown = windows::ctrl_shutdown().map_err(|err| ApplicationError::ServerStartUpError("Failed to terminate: {err}"))?;

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
    async fn test_get_setup() {
        let config = AppConfiguration::load("./tests/resources/test_http_mock.json").unwrap();
        assert!(get_setup("1", &config).is_ok());
        assert!(get_setup("2", &config).is_err());
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 10)]
    async fn test_start_daemon() {
        let config: AppConfiguration = AppConfiguration::load("./tests/resources/test_http_mock.json").unwrap();
        let args = Args::parse_from(["apinae-daemon", "--file", "./tests/resources/test_http_mock.json", "--id", "1", "--verify"]);
        let _ = start_daemon(args, &config).await.is_ok();
        let args = Args::parse_from(["apinae-daemon", "--file", "./tests/resources/test_http_mock.json", "--id", "2", "--verify"]);
        assert!(start_daemon(args, &config).await.is_err());
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 10)]
    async fn test_list_setups() {
        let config: AppConfiguration = AppConfiguration::load("./tests/resources/test_http_mock.json").unwrap();
        list_setups(&config);
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 10)]
    async fn test_validate_parameters() {
        let config: AppConfiguration = AppConfiguration::load("./tests/resources/test_http_mock_with_param.json").unwrap();
        let args_missing_param = Args::parse_from(["apinae-daemon", "--file", "./tests/resources/test_http_mock.json", "--id", "1"]);
        assert!(validate_parameters(config.setups.first().unwrap(), &args_missing_param).is_err());
        let args_missing_param1 = Args::parse_from(["apinae-daemon", "--file", "./tests/resources/test_http_mock.json", "--id", "1", "--param", "param2=2"]);
        assert_eq!(validate_parameters(config.setups.first().unwrap(), &args_missing_param1), Err(ApplicationError::CouldNotFind("Missing parameter: param1".to_string())));
        let args_params_ok = Args::parse_from(["apinae-daemon", "--file", "./tests/resources/test_http_mock.json", "--id", "1", "--param", "param2=2", "--param", "param1=1"]);
        assert_eq!(validate_parameters(config.setups.first().unwrap(), &args_params_ok), Ok(vec![("param2".to_string(), "2".to_string()), ("param1".to_string(), "1".to_string())]));
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 10)]
    async fn test_validate_parameter_set() {
        let config: AppConfiguration = AppConfiguration::load("./tests/resources/test_predefined_set.json").unwrap();
        let args_missing_param = Args::parse_from(["apinae-daemon", "--file", "./tests/resources/test_predefined_set.json", "--id", "1"]);
        assert!(validate_parameters(config.setups.first().unwrap(), &args_missing_param).is_err());
        let args_missing_param1 = Args::parse_from(["apinae-daemon", "--file", "./tests/resources/test_predefined_set.json", "--id", "1", "--predefined-set", "not_found"]);
        assert!(validate_parameters(config.setups.first().unwrap(), &args_missing_param1).is_ok());
    }
}
