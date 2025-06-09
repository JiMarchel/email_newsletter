use email_newsletter::{
    configuration::get_configuration,
    startup::run,
    telemetry::{get_subscriber, init_subscriber},
};
use secrecy::ExposeSecret;
use sqlx::postgres::PgPoolOptions;

use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let subscriber = get_subscriber("email_newsletter".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration!");
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&configuration.database.connection_string().expose_secret())
        .await
        .expect("Failed to connect postgres");

    let addr = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(addr).await.unwrap();
    run(listener, pool).await;
}
