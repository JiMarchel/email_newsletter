use std::{pin::Pin, sync::Arc};

use crate::{
    configuration::{DatabaseSettings, Settings},
    email_client::EmailClient,
    routes::{
        health_check::health_check, subscriptions::subscribe, subscriptions_confirm::confirm,
    },
};
use axum::{
    Router,
    body::Body,
    extract::Request,
    routing::{get, post},
    serve,
};
use sqlx::{PgPool, Pool, Postgres, postgres::PgPoolOptions};
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tower_request_id::{RequestId, RequestIdLayer};
use tracing::info_span;

pub struct ApplicationBaseUrl(pub String);
pub struct ApplicationState {
    pub pool: Pool<Postgres>,
    pub email_client: EmailClient,
    pub base_url: ApplicationBaseUrl,
}

pub struct Application {
    port: u16,
    server: Pin<Box<dyn Future<Output = ()> + Send + 'static>>,
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, std::io::Error> {
        let pool = get_connection_pool(&configuration.database);

        let sender_email = configuration
            .email_client
            .sender()
            .expect("Invalid sender email address");
        let timeout = configuration.email_client.timeout();
        let email_client = EmailClient::new(
            configuration.email_client.base_url.clone(),
            sender_email,
            configuration.email_client.authorization_token.clone(),
            timeout,
        );

        let addr = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );
        let listener = TcpListener::bind(addr).await.unwrap();
        let port = listener.local_addr().unwrap().port();
        let server = run(
            listener,
            pool,
            email_client,
            configuration.application.base_url,
        );

        Ok(Self {
            port,
            server: Box::pin(server),
        })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) {
        self.server.await
    }
}

pub fn get_connection_pool(configuration: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new()
        .max_connections(10)
        .connect_lazy_with(configuration.with_db())
}

pub async fn run(
    listener: TcpListener,
    pool: Pool<Postgres>,
    email_client: EmailClient,
    base_url: String,
) {
    let app_state = Arc::new(ApplicationState {
        pool,
        email_client,
        base_url: ApplicationBaseUrl(base_url),
    });
    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
        .route("/subscriptions/confirm", get(confirm))
        .with_state(app_state)
        .layer(ServiceBuilder::new().layer(RequestIdLayer).layer(
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
        ));

    serve(listener, app).await.unwrap();
}
