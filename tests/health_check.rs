//! tests/health_check.rs

// cargo expand --test health_check

#[tokio::test]
async fn health_check_works() {
    // Arrange
    spawn_app();

    // Use `reqwest` to perform HTTP requests against our app
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

// Launch our application in the background
// No await-ing, if tests fail panic and crash all the things
fn spawn_app() {
    let server = zero2prod::run().expect("Failed to bind address.");
    // Launch server as background task
    let _ = tokio::spawn(server);
}
