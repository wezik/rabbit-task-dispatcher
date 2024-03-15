use dotenv::dotenv;
use lapin::{Connection, ConnectionProperties};
use log::{debug, info};

fn main() {
    dotenv().ok();

    env_logger::init();
    let log_level = fetch_env_variable("RUST_LOG", "", false);
    debug!("Log level set to '{}'", log_level);

    establish_rabbitmq_connection();
}

fn establish_rabbitmq_connection() {
    info!("Establishing RabbitMQ connection");
    let host = fetch_env_variable("RABBITMQ_HOST", "localhost", true);
    let port = fetch_env_variable("RABBITMQ_PORT", "5672", true);
    let username = fetch_env_variable("RABBITMQ_USERNAME", "guest", true);
    let password = fetch_env_variable("RABBITMQ_PASSWORD", "guest", true);

    let addr = format!("amqp://{}:{}@{}:{}/", username, password, host, port);

    let _conn = Connection::connect(&addr, ConnectionProperties::default());

    info!("Connected to RabbitMQ at '{}:{}'", host, port);
}

fn fetch_env_variable(token: &str, default: &str, should_default: bool) -> String {
    match std::env::var(token) {
        Ok(value) => value,
        Err(_) if should_default => {
            debug!(
                "Environment variable '{}' not found, defaulted to '{}'",
                token, default
            );
            default.to_string()
        }
        _ => panic!(
            "Environment variable '{}' not found and no default provided",
            token
        ),
    }
}
