use dotenv::dotenv;
use log::debug;
use std::error::Error;

use crate::rabbit_service::RabbitService;

mod rabbit_service;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    env_logger::init();
    let log_level = utils::get_env_var("RUST_LOG", "", false);
    debug!("Log level set to '{}'", log_level);

    let rabbitmq = RabbitService::new().await;
    rabbitmq.publish("Hello world RabbitMQ!").await;

    Ok(())
}
