use std::process::{Child, Command};

mod common;

/**
 * Initalizes the server for http with a mocked response.
 * Requests the server with curl and verifies the response.
 */
#[tokio::test(flavor = "multi_thread", worker_threads = 10)]
async fn test_tcp_listener() {
    // Start the server.
    let mut server_command = common::start_server("./tests/resources/test_tcp_listener.json", "1").await.expect("Failed to start server");

    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    // Assert the server response.
    assert_command(&mut server_command, "8180", "Test").await;
    assert_command(&mut server_command, "8181", "Testing This File").await;
    assert_command(&mut server_command, "8182", "").await;
    assert_command(&mut server_command, "8183", "").await;

    // Stop the server.
    server_command.kill().expect("Failed to kill process");
}

/**
 * Asserts the server response.
 */
async fn assert_command(server_command: &mut Child, port: &str, expected: &str) {
    let connect = format!("http://localhost:{port}");
    let nc_command = match Command::new("curl").arg("--http0.9").arg("-X").arg("GET").arg("--max-time").arg("3").arg(connect).output() {
        Ok(nc_command) => nc_command,
        Err(error) => {
            server_command.kill().expect("Failed to kill server process");
            panic!("Failed to execute nc command: {}", error);
        }
    };
    // Read the output from the nc command.
    let output_string = String::from_utf8_lossy(&nc_command.stdout).to_string();
    // Verify the output.
    if output_string.trim() != expected {
        server_command.kill().expect("Failed to kill server process");
        panic!("Expected: {expected}, Got: {output_string}");
    }
}
