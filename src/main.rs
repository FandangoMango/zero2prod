use std::net::TcpListener;
use zero2prod::startup::run;
use zero2prod::configuration::get_configuration;
use zero2prod::telemetry::{get_subscriber, init_subscriber};
use secrecy::{ExposeSecret};
use sqlx::postgres::PgPoolOptions;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let name = "zero2prod".to_string();
    let env_filter = "info".to_string();
    let sink = std::io::stdout;
    // prints all spans at "env_filter"-level by default. 
    // Can be changed with the RUST_LOG environment variable.
    // sink determines where logs are printed
    let subscriber = get_subscriber(name, env_filter, sink);
    init_subscriber(subscriber);

    let configuration = 
        get_configuration().expect("Failed to read configuration");
    let connection_pool = PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.database.with_db());
    let address = format!(
        "{}:{}", 
        configuration.application.host,
        configuration.application.port, 
    );
    let listener = TcpListener::bind(address)?;

    run(listener, connection_pool)?.await
}

