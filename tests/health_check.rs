use email_newsletter::run;
use reqwest::header::CONTENT_TYPE;
use tokio::net::TcpListener;

async fn spawn_app() -> String {
    // when you bind using port 0 it's will tell the os to find available port
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    tokio::spawn(async move {
        run(listener).await;
    });

    format!("http://{addr}")
}

#[tokio::test]
async fn health_check_works() {
    let addr = spawn_app().await;

    // perform http request with reqwest
    let client = reqwest::Client::new();
    //Act
    let response = client
        .get(&format!("{}/health_check", &addr))
        .send()
        .await
        .expect("Failed to execute request");

    //Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribing_return_a_200_for_valid_data_form() {
    let addr = spawn_app().await;
    let client = reqwest::Client::new();

    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", &addr))
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("failed to execute request");

    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn subscrice_return_a_422_when_data_is_missing() {
    let addr = spawn_app().await;
    let cliet = reqwest::Client::new();
    let test_case = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_case {
        let response = cliet
            .post(&format!("{}/subscriptions", &addr))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request");

        assert_eq!(
            422,
            response.status().as_u16(), // Axum defaults to 422 instead of 400
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}
