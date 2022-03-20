//! main.rs

mod configuration;
mod routes;
mod startup;
mod telemetry;

use configuration::get_configuration;
use secrecy::ExposeSecret;
use sqlx::PgPool;
use startup::run;
use std::net::TcpListener;
use telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("newsletter".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool =
        PgPool::connect(configuration.database.connection_string().expose_secret())
            .await
            .expect("Failed to connect to Postgres.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address).expect("Failed to bind to random port");
    run(listener, connection_pool)?.await
}
