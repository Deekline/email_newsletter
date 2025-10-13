use email_newsletter::configuration::get_configuration;
use email_newsletter::startup::run;
use email_newsletter::telemetry::{get_subsciber, init_subscriber};
use sqlx::PgPool;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subsciber("email_newsletter".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");

    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgress.");

    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address).expect("Failed to get port");

    run(listener, connection_pool)?.await
}
