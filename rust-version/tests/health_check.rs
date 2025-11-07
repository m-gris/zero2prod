//! tests/health_check.rs

use std::net::TcpListener;

use const_format::formatcp; // For compile-time string formatting

// `tokio::test` is the testing equivalent of `tokio::main`.
// It also spares you from having to specify the `#[test]` attribute. //
// You can inspect what code gets generated using
// `cargo expand --test health_check` (<- name of the test file)
#[tokio::test]
async fn health_check_works() {
    // ARRANGE
    let root_address = spawn_app();
    // nota: no http:// in the string... since it already is baked in root_address
    let health_address = &format!("{}/health_check", &root_address);
    // use REQWEST to perform HTTP requests against our app
    let client = reqwest::Client::new();

    // ACT
    let response = client
        .get(health_address)
        .send()
        .await
        .expect("Failed to execute request");

    // ASSERT
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());

    // No .await call, therefore no need for `spawn_app` to be async now.
    // We are also running tests, so it is not worth it to propagate errors:
    // if we fail to perform the required setup we can just panic and crash.
    fn spawn_app() -> String {
        const HOST: &str = "127.0.0.1";
        const RANDOM_PORT: &str = "0"; // (i.e OS scan and takes whatever is available)
        const TMP_ADDRESS: &str = formatcp!("{}:{}", HOST, RANDOM_PORT);
        let listener: TcpListener =
            TcpListener::bind(TMP_ADDRESS).expect("Failed to bind to the address");
        // We retrieve the port assigned to us by the OS
        let port = listener.local_addr().unwrap().port();
        let server = zero2prod::run(listener).expect("Failed to bind address"); // Launch the server as a background task
        // tokio::spawn returns a handle to the spawned future,
        // but we have no use for it here, hence the non-binding let
        let _ = tokio::spawn(server);

        // We return the application address to the caller!
        format!("http://127.0.0.1:{}", port)
    }

    // A NOTE ON CLEAN-UP / TEARDOWN
    // when a tokio runtime is shut down all tasks spawned on it are dropped.
    // tokio::test spins up a new runtime at the beginning of each test case and they shut down at the end of each test case.
}
