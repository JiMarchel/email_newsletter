use axum::{extract::Path, http::StatusCode, response::IntoResponse, routing::get, serve, Router};
use tokio::net::TcpListener;

pub async fn run() {
    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/hello", get(greet))
        .route("/hello/{name}", get(greet));
    let listener = TcpListener::bind("0.0.0.0:8000").await.unwrap();
    serve(listener, app).await.unwrap();
}

async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}

async fn greet(name: Option<Path<String>>) -> String {
    let name = match name {
        Some(Path(name)) => name,
        None => "World".to_string(),
    };

    format!("Hello {}!", name)
}