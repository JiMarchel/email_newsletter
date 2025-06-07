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

#[tracing::instrument(
    name= "Adding a new subscriber",
    skip(form, pool),
    fields(
        subscriber_email = %form.email,
        subscriber_name = %form.name
    )
)]
pub async fn subscribe(
    State(pool): State<Arc<ApplicationState>>,
    Form(form): Form<FormData>,
) -> impl IntoResponse {
    match insert_subscriber(&pool, &form).await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(form, pool)
)]
pub async fn insert_subscriber(
    pool: &Arc<ApplicationState>,
    form: &FormData,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO subscriptions (id, email, name, subscribed_at) VALUES($1, $2, $3, $4);",
    )
    .bind(Uuid::new_v4())
    .bind(&form.email)
    .bind(&form.name)
    .bind(Utc::now())
    .execute(&pool.pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(())
}
