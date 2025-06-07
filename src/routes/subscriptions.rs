use std::sync::Arc;

use crate::startup::ApplicationState;
use axum::{
    extract::{Form, State},
    http::StatusCode,
    response::IntoResponse,
};
use chrono::Utc;
use serde::Deserialize;
use tracing::Instrument;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct FormData {
    pub email: String,
    pub name: String,
}
pub async fn subscribe(
    State(db): State<Arc<ApplicationState>>,
    Form(form): Form<FormData>,
) -> impl IntoResponse {
    let request_id = Uuid::new_v4();
    let request_span = tracing::info_span!(
        "Adding a new subscriber",
            %request_id,
            subscriber_email = %form.email,
            subscriber_name = %form.name
    );
    let _request_span_guard = request_span.enter();

    let query_span = tracing::info_span!("Saving new subscriber details in the database");

    match sqlx::query(
        "INSERT INTO subscriptions (id, email, name, subscribed_at) VALUES($1, $2, $3, $4);",
    )
    .bind(Uuid::new_v4())
    .bind(form.email)
    .bind(form.name)
    .bind(Utc::now())
    .execute(&db.pool)
    .instrument(query_span)
    .await
    {
        Ok(_) => StatusCode::OK,
        Err(e) => {
            tracing::error!("Failed to execute query {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
