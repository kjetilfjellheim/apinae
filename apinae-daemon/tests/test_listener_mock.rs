use std::process::{Child, Command, Stdio};

mod common;

/**
 * Initalizes the server for http with a mocked response.
 * Requests the server with curl and verifies the response.
 */
#[tokio::test(flavor = "multi_thread", worker_threads = 10)]
async fn test_tcp_listener() {
    // Start the server.
    let mut server_command = common::start_server("./tests/resources/test_tcp_listener.json", "1")
        .await
        .expect("Failed to start server");

    // Assert the server response.
    assert_command(&mut server_command, "8080", "Test").await;
    assert_command(&mut server_command, "8081", "Testing This File").await;
    assert_command(&mut server_command, "8082", "").await;
    assert_command(&mut server_command, "8083", "").await;

    // Stop the server.
    server_command.kill().expect("Failed to kill process");    
}

async fn assert_command(server_command: &mut Child, port : &str, expected: &str) {        
    let nc_command = match Command::new("nc")
        .arg("-w")
        .arg("3")
        .arg("localhost")
        .arg(port)
        .output()                
    {        
        Ok(nc_command) => nc_command,
        Err(error) => {
            println!("Failed to execute nc command: {}", error);
            server_command
                .kill()
                .expect("Failed to kill server process");
            panic!("Failed to execute nc command: {}", error);
        }
    };
    // Read the output from the nc command.
    let output_string = String::from_utf8_lossy(&nc_command.stdout).to_string();   
    // Verify the output.
    assert_eq!(output_string, expected);
}
