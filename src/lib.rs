use axum::{
    Form, Router,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    serve,
};
use serde::Deserialize;
use tokio::net::TcpListener;

#[derive(Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub fn app() -> Router {
    Router::new()
        .route("/health_check", get(health_check))
        .route("/subscription", post(subscribe))
}

pub async fn run(listener: TcpListener) {
    serve(listener, app()).await.unwrap();
}

async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}

async fn subscribe(_form: Form<FormData>) -> impl IntoResponse {
    StatusCode::OK
}
