use std::process::{Child, Command};

pub async fn start_server(config_file: &str, id: &str) -> Result<Child, Box<dyn std::error::Error>> {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let binary_path = format!("{manifest_dir}/../target/debug/apinae-daemon");
    let config_path = format!("{manifest_dir}/{config_file}");
    // Start the daemon with the specified id.
    let server_command = Command::new(binary_path).args(["--file", &config_path, "--id", id]).spawn()?;
    // Wait until server is running.
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    Ok(server_command)
}
