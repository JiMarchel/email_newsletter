use std::sync::Arc;

use crate::{
    domain::{NewSubscriber, SubscriberName},
    startup::ApplicationState,
};
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
    let name = match SubscriberName::parse(form.name) {
        Ok(name) => name,
        Err(_) => return StatusCode::BAD_REQUEST,
    };

    let new_subscriber = NewSubscriber {
        email: form.email,
        name,
    };

    match insert_subscriber(&pool, &new_subscriber).await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(new_subscriber, pool)
)]
pub async fn insert_subscriber(
    pool: &Arc<ApplicationState>,
    new_subscriber: &NewSubscriber,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO subscriptions (id, email, name, subscribed_at) VALUES($1, $2, $3, $4);",
    )
    .bind(Uuid::new_v4())
    .bind(&new_subscriber.email)
    .bind(new_subscriber.name.as_ref())
    .bind(Utc::now())
    .execute(&pool.pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(())
}
