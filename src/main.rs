use std::net::TcpListener;
use zero2prod::startup::run;
use zero2prod::configuration::get_configuration;
use sqlx::PgPool;
use zero2prod::telemetry::{get_subscriber, init_subscriber};
use secrecy::{ExposeSecret, Secret};


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
    let connection_pool = 
        PgPool::connect(&configuration.database.connection_string().expose_secret())
            .await
            .expect("Failed to connect to postgres.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;

    run(listener, connection_pool)?.await
}

