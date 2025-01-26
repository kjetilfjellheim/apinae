use std::process::{Child, Command};

pub async fn start_server(config_file: &str, id: &str) -> Result<Child, Box<dyn std::error::Error>> {
    // Start the daemon with the specified id.
    let server_command = Command::new("../target/debug/testit-daemon")
        .args(["--file", config_file, "--id", id])    
        .spawn()?;
    // Wait until server is running.
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;   
    Ok(server_command)
}