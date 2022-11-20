use zero2prod::configuration::get_configuration;
use zero2prod::startup::Application;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

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
    let configuration = get_configuration().expect("Failed to read configuration");
    let application = Application::build(configuration).await?;
    application.run_until_stopped().await?;
    Ok(())
}
