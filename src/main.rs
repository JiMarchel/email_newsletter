use email_newsletter::{
    configuration::get_configuration,
    startup::Application,
    telemetry::{get_subscriber, init_subscriber},
};
#[tokio::main]
async fn main() {
    let subscriber = get_subscriber("email_newsletter".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration!");
    let application = Application::build(configuration)
        .await
        .expect("Failed to build application");
    application.run_until_stopped().await;
}
