use std::sync::Arc;

use crate::startup::ApplicationState;
use axum::{
    extract::{Form, State},
    http::StatusCode,
    response::IntoResponse,
};
use chrono::Utc;
use serde::Deserialize;
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
    match sqlx::query(
        "INSERT INTO subscriptions (id, email, name, subscribed_at) VALUES($1, $2, $3, $4);",
    )
    .bind(Uuid::new_v4())
    .bind(form.email)
    .bind(form.name)
    .bind(Utc::now())
    .execute(&db.pool)
    .await
    {
        Ok(_) => StatusCode::OK,
        Err(e) => {
            println!("Failed to execute query {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
