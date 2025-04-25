use tokio::process::Command;

mod common;

/**
 * Initalizes the server for http with a mocked response.
 * Requests the server with curl and verifies the response.
 */
#[tokio::test(flavor = "multi_thread", worker_threads = 10)]
async fn test_http_server_with_proxy() {
    // Start the proxy. Allow zombie process as it's a daemon running.
    #![allow(clippy::zombie_processes)]
    let mut tinyproxy_command = Command::new("tinyproxy").arg("-c").arg("./tests/resources/http_tinyproxy.conf").arg("-d").spawn().expect("Failed to start tinyproxy");
    // Start the server. Allow zombie process as it's a daemon running.
    let mut server_command = common::start_server("./tests/resources/test_http_mock_with_proxy.json", "1").await.expect("Failed to start server");
    // Run curl and verify the response.
    let curl_command: std::process::Output = match Command::new("curl").arg("http://localhost:8080/test").output().await {
        Ok(command) => command,
        Err(error) => {
            server_command.kill().expect("Failed to kill server process");
            tinyproxy_command.kill().await.expect("Failed to kill process");
            panic!("Failed to execute curl command: {error}");
        }
    };
    // Read the output from the curl command.
    let output_string = String::from_utf8_lossy(&curl_command.stdout).to_string();
    // Stop the server.
    server_command.kill().expect("Failed to kill process");
    // Stop the proxy.
    tinyproxy_command.kill().await.expect("Failed to kill process");
    // Verify the output.
    assert_eq!(output_string, "{ \"test\": \"Success http\" }");
}
