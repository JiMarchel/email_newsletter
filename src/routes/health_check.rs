use axum::response::IntoResponse;
use reqwest::StatusCode;

pub async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}
