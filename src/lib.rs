use axum::{Router, extract::Path, http::StatusCode, response::IntoResponse, routing::get, serve};
use tokio::net::TcpListener;

pub fn app() -> Router {
    Router::new()
        .route("/health_check", get(health_check))
        .route("/hello", get(greet))
        .route("/hello/{name}", get(greet))
}

pub async fn run(listener: TcpListener) {
    serve(listener, app()).await.unwrap();
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
