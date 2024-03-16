use crate::utils;
use lapin::{options::*, types::FieldTable, BasicProperties, Connection, ConnectionProperties};
use log::info;

pub struct RabbitService {
    connection: Connection,
}

impl RabbitService {
    pub async fn new() -> Self {
        let connection = establish_rabbitmq_connection().await;
        RabbitService { connection }
    }

    pub async fn publish(&self, message: &str) {
        let channel = self
            .connection
            .create_channel()
            .await
            .expect("Failed to create channel");
        info!("Created channel");

        let queue_name = utils::get_env_var("RABBITMQ_QUEUE", "task-dispatcher", true);

        channel
            .queue_declare(
                &queue_name,
                QueueDeclareOptions::default(),
                FieldTable::default(),
            )
            .await
            .expect("Failed to declare queue");
        info!("Declared queue");

        info!("Publishing message");
        channel
            .basic_publish(
                message,
                &queue_name,
                BasicPublishOptions::default(),
                message.as_bytes(),
                BasicProperties::default(),
            )
            .await
            .expect("Failed to publish");
        info!("Succesfully published a message");
    }
}

async fn establish_rabbitmq_connection() -> Connection {
    info!("Establishing RabbitMQ connection");
    let host = utils::get_env_var("RABBITMQ_HOST", "localhost", true);
    let port = utils::get_env_var("RABBITMQ_PORT", "5672", true);
    let username = utils::get_env_var("RABBITMQ_USERNAME", "guest", true);
    let password = utils::get_env_var("RABBITMQ_PASSWORD", "guest", true);
    let vhost = utils::get_env_var("RABBITMQ_VHOST", "", true);

    let addr = format!(
        "amqp://{}:{}@{}:{}/{}",
        username, password, host, port, vhost
    );
    let conn = Connection::connect(&addr, ConnectionProperties::default())
        .await
        .expect("Failed to connect to RabbitMQ");

    info!("Connected to RabbitMQ at '{}:{}'", host, port);

    conn
}
