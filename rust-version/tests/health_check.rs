//! tests/health_check.rs

use const_format::formatcp; // For compile-time string formatting

// `tokio::test` is the testing equivalent of `tokio::main`.
// It also spares you from having to specify the `#[test]` attribute. //
// You can inspect what code gets generated using
// `cargo expand --test health_check` (<- name of the test file)
#[tokio::test]
async fn health_check_works() {
    // ARRANGE
    spawn_app();
    // use REQWEST to perform HTTP requests against our app
    let client = reqwest::Client::new();
    const HOST: &str = "127.0.0.1";
    const PORT: &str = "8765";
    const ROOT_ADDRESS: &str = formatcp!("{}:{}", HOST, PORT);
    const HEALTH_ADDRESS: &str = formatcp!("http://{}/health_check", ROOT_ADDRESS);

    // ACT
    let response = client
        .get(HEALTH_ADDRESS)
        .send()
        .await
        .expect("Failed to execute request");

    // ASSERT
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());

    // No .await call, therefore no need for `spawn_app` to be async now.
    // We are also running tests, so it is not worth it to propagate errors:
    // if we fail to perform the required setup we can just panic and crash.
    fn spawn_app() {
        let server = zero2prod::run(ROOT_ADDRESS).expect("Failed to bind address"); // Launch the server as a background task
        // tokio::spawn returns a handle to the spawned future,
        // but we have no use for it here, hence the non-binding let
        let _ = tokio::spawn(server);
    }

    // A NOTE ON CLEAN-UP / TEARDOWN
    // when a tokio runtime is shut down all tasks spawned on it are dropped.
    // tokio::test spins up a new runtime at the beginning of each test case and they shut down at the end of each test case.
}
