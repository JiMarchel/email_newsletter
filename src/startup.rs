use std::sync::Arc;

use crate::routes::{health_check::health_check, subscriptions::subscribe};
use axum::{
    Router,
    body::Body,
    extract::Request,
    routing::{get, post},
    serve,
};
use sqlx::{Pool, Postgres};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tower_request_id::{RequestId, RequestIdLayer};
use tracing::info_span;

pub struct ApplicationState {
    pub pool: Pool<Postgres>,
}

pub async fn run(listener: TcpListener, pool: Pool<Postgres>) {
    let app_state = Arc::new(ApplicationState { pool });
    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
        .with_state(app_state)
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &Request<Body>| {
                // We get the request id from the extensions
                let request_id = request
                    .extensions()
                    .get::<RequestId>()
                    .map(ToString::to_string)
                    .unwrap_or_else(|| "unknown".into());
                // And then we put it along with other information into the `request` span
                info_span!(
                    "request",
                    id = %request_id,
                    method = %request.method(),
                    uri = %request.uri(),
                )
            }),
        )
        .layer(RequestIdLayer);

    serve(listener, app).await.unwrap();
}
