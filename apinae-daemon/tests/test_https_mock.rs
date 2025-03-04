use tokio::process::Command;

mod common;

/**
 * Initalizes the server for http with a mocked response.
 * Requests the server with curl and verifies the response.
 */
#[tokio::test(flavor = "multi_thread", worker_threads = 10)]
async fn test_https_server() {
    // Start the server. Allow zombie process as it's a daemon running.
    #![allow(clippy::zombie_processes)]
    let mut server_command = common::start_server("./tests/resources/test_https_mock.json", "1")
        .await
        .expect("Failed to start server");
    // Run curl and verify the response.
    let curl_command = match Command::new("curl")
        .arg("-v")
        .arg("--insecure")
        .arg("https://localhost:8080/test")
        .output()
        .await
    {
        Ok(command) => command,
        Err(error) => {
            server_command
                .kill()
                .expect("Failed to kill server process");
            panic!("Failed to execute curl command: {}", error);
        }
    };
    // Read the output from the curl command.
    let output_string = String::from_utf8_lossy(&curl_command.stdout).to_string();
    // Stop the server.
    server_command.kill().expect("Failed to kill process");
    // Verify the output.
    assert_eq!(output_string, "{ \"test\": \"Success https\" }");
}
