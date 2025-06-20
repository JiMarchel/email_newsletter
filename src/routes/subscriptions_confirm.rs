use axum::{extract::Query, response::IntoResponse};
use reqwest::StatusCode;

#[derive(serde::Deserialize)]
pub struct Parameters {
    subscription_token: String,
}

#[tracing::instrument(name = "Confirm a pending subscriber", skip(parameters))]
pub async fn confirm(Query(parameters): Query<Parameters>) -> impl IntoResponse {
    StatusCode::OK
}
