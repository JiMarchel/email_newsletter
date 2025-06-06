use std::sync::Arc;

use crate::routes::{health_check::health_check, subscriptions::subscribe};
use axum::{
    Router,
    routing::{get, post},
    serve,
};
use sqlx::{Pool, Postgres};
use tokio::net::TcpListener;
pub struct ApplicationState {
    pub pool: Pool<Postgres>,
}

pub async fn run(listener: TcpListener, pool: Pool<Postgres>) {
    let app_state = Arc::new(ApplicationState { pool });
    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
        .with_state(app_state);

    serve(listener, app).await.unwrap();
}
