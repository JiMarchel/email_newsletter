use std::sync::Arc;

use crate::routes::{health_check::health_check, subscriptions::subscribe};
use axum::{
    Router,
    routing::{get, post},
    serve,
};
use sqlx::{Pool, Postgres};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
pub struct ApplicationState {
    pub pool: Pool<Postgres>,
}

pub async fn run(listener: TcpListener, pool: Pool<Postgres>) {
    let app_state = Arc::new(ApplicationState { pool });
    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
        .with_state(app_state)
        .layer(TraceLayer::new_for_http());

    serve(listener, app).await.unwrap();
}
