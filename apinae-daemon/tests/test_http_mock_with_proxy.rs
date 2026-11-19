use tokio::process::Command;

mod common;

/**
 * Initalizes the server for http with a mocked response.
 * Requests the server with curl and verifies the response.
 */
#[tokio::test(flavor = "multi_thread", worker_threads = 10)]
async fn test_http_server_with_proxy() {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let tinyproxy_config = format!("{manifest_dir}/tests/resources/http_tinyproxy.conf");
    // Start the proxy. Allow zombie process as it's a daemon running.
    let mut tinyproxy_command = Command::new("tinyproxy")
        .arg("-c")
        .arg(tinyproxy_config)
        .arg("-d")
        .spawn()
        .expect("Failed to start tinyproxy");
    tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
    if tinyproxy_command.try_wait().expect("Failed to read tinyproxy status").is_some() {
        eprintln!("Skipping test_http_server_with_proxy: tinyproxy exited before startup");
        return;
    }
    // Start the server. Allow zombie process as it's a daemon running.
    let mut server_command = common::start_server("tests/resources/test_http_mock_with_proxy.json", "1-1")
        .await
        .expect("Failed to start server");
    // Run curl and verify the response.
    let curl_command: std::process::Output = match Command::new("curl").arg("http://localhost:8080/test").output().await {
        Ok(command) => command,
        Err(error) => {
            let _ = server_command.kill();
            let _ = tinyproxy_command.kill().await;
            panic!("Failed to execute curl command: {error}");
        }
    };
    // Read the output from the curl command.
    let output_string = String::from_utf8_lossy(&curl_command.stdout).to_string();
    // Stop the server.
    let _ = server_command.kill();
    // Stop the proxy.
    let _ = tinyproxy_command.kill().await;
    // Verify the output.
    assert_eq!(output_string, "{ \"test\": \"Success http\" }");
}
