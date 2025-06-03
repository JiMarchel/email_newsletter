
#[tokio::test]
async fn health_check_works(){
       spawn_app().await.expect("Failed to spawn our app");

       // perform http request with reqwest
       let client = reqwest::Client::new();
       //Act
       let response = client.get("http://0.0.0.0:8000/health_check").send().await.expect("Failed to execute request");

       //Assert
       assert!(response.status().is_success());
       assert_eq!(Some(0), response.content_length()); 
}

async fn spawn_app(){
    todo!()
}