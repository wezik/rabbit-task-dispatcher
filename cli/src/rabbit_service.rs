use crate::utils;
use lapin::{options::*, types::FieldTable, BasicProperties, Connection, ConnectionProperties};
use log::info;

pub async fn publish(queue_name: &str, message: &str) {
    info!("Publishing '{}' to '{}' queue", message, queue_name);

    let connection = establish_rabbitmq_connection().await;
    let channel = connection
        .create_channel()
        .await
        .expect("Failed to create channel");

    channel
        .queue_declare(
            &queue_name,
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await
        .expect("Failed to declare queue");

    channel
        .basic_publish(
            "",
            &queue_name,
            BasicPublishOptions::default(),
            message.as_bytes(),
            BasicProperties::default(),
        )
        .await
        .expect("Failed to publish");
}

async fn establish_rabbitmq_connection() -> Connection {
    let host = utils::get_env_var("RABBITMQ_HOST", "localhost", true);
    let port = utils::get_env_var("RABBITMQ_PORT", "5672", true);
    let username = utils::get_env_var("RABBITMQ_USERNAME", "guest", true);
    let password = utils::get_env_var("RABBITMQ_PASSWORD", "guest", true);
    let vhost = utils::get_env_var("RABBITMQ_VHOST", "test", true);

    let addr = format!(
        "amqp://{}:{}@{}:{}/{}",
        username, password, host, port, vhost
    );
    let conn = Connection::connect(&addr, ConnectionProperties::default())
        .await
        .expect("Failed to connect to RabbitMQ");

    conn
}
