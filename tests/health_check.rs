use actix_web::dev::Server;

#[tokio::test]
async fn dummy_test() {
    spawn_app();

    let client = reqwest::Client::new();
    let response = client
        .get("http://127.0.0.1:8000/health-check")
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() {
    let server: Server = zero2prod::run().expect("Failed to bind address");
    let _ = tokio::spawn(server);
}
