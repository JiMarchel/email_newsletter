use email_newsletter::run;
use tokio::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    let addr = spawn_app().await;

    // perform http request with reqwest
    let client = reqwest::Client::new();
    //Act
    let response = client
        .get(format!("http://{addr}/health_check"))
        .send()
        .await
        .expect("Failed to execute request");

    //Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

async fn spawn_app() -> std::net::SocketAddr {
    // when you bind using port 0 it's will tell the os to find available port
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    tokio::spawn(async move {
        run(listener).await;
    });

    addr
}

