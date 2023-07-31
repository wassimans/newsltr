use std::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let address = spawn_app();

    // Use `reqwest` to perform HTTP requests against our app
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&format!("http://{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

// No .await call, therefore no need for `spawn_app` to be async now.
// We are also running tests, so it is not worth it to propagate errors:
// if we fail to perform the required setup we can just panic and crash
// all the things.
fn spawn_app() -> String {
    let listner = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");

    // Retrieve the port assigned to us by the OS
    let port = listner.local_addr().unwrap().port();

    let server = newsltr::run(listner).expect("Failed to bind address");
    // Launch the server as a background task
    // tokio::spawn returns a handle to the spawned future,
    // but we have no use for it here, hence the non-binding let
    let _ = tokio::spawn(server);
    format!("127.0.0.1:{}", port)
}
