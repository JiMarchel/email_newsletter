use axum::response::IntoResponse;
use reqwest::StatusCode;
use tracing::info;

pub async fn health_check() -> impl IntoResponse {
    info!("HEALTH_CHECKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKK");
    StatusCode::OK
}
