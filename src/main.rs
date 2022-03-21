//! main.rs

mod configuration;
mod routes;
mod startup;
mod telemetry;

use configuration::get_configuration;
use secrecy::ExposeSecret;
use sqlx::{PgPool, postgres::PgPoolOptions};
use startup::run;
use std::net::TcpListener;
use telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("newsletter".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect_lazy(&configuration.database.connection_string().expose_secret())
            .expect("Failed to create Postgres connection pool.");
    let address = format!("127.0.0.1:{}", configuration.application.port);
    let listener = TcpListener::bind(address).expect("Failed to bind to random port");
    run(listener, connection_pool)?.await
}
