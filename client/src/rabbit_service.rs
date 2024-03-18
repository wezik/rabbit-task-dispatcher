use amqprs::{
    callbacks::{DefaultChannelCallback, DefaultConnectionCallback},
    channel::{
        BasicConsumeArguments, BasicPublishArguments, Channel, ConsumerMessage,
        QueueDeclareArguments,
    },
    connection::{Connection, OpenConnectionArguments},
    BasicProperties,
};
use core::time;
use tokio::{
    sync::mpsc::{self, Sender},
    task::JoinHandle,
};

use crate::{
    context_handler::ContextOP,
    log_handler::{self, LOG},
};

pub struct RabbitConnect {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub vhost: String,
    pub connection_name: String,
}

pub async fn establish_connection(connection_details: &RabbitConnect) -> Connection {
    let mut res = Connection::open(
        &OpenConnectionArguments::new(
            &connection_details.host,
            connection_details.port,
            &connection_details.username,
            &connection_details.password,
        )
        .virtual_host(&connection_details.vhost)
        .connection_name(&connection_details.connection_name),
    )
    .await;

    while res.is_err() {
        std::thread::sleep(time::Duration::from_millis(2000));
        res = Connection::open(
            &OpenConnectionArguments::new(
                &connection_details.host,
                connection_details.port,
                &connection_details.username,
                &connection_details.password,
            )
            .virtual_host(&connection_details.vhost)
            .connection_name(&connection_details.connection_name),
        )
        .await;
    }

    let connection = res.unwrap();
    connection
        .register_callback(DefaultConnectionCallback)
        .await
        .unwrap();

    connection
}

pub async fn establish_channel(connection: &Connection) -> Channel {
    let channel = connection.open_channel(None).await.unwrap();
    channel
        .register_callback(DefaultChannelCallback)
        .await
        .unwrap();
    channel
}

pub async fn declare_queue(channel: &Channel, queue_name: &str) {
    let args = QueueDeclareArguments::new(queue_name);
    channel.queue_declare(args).await.unwrap().unwrap();
}

pub async fn create_publisher(
    channel: Channel,
    queue_name: &str,
) -> (mpsc::Sender<String>, JoinHandle<()>) {
    async fn publish_messages(
        mut rx: mpsc::Receiver<String>,
        channel: Channel,
        queue_name: String,
    ) {
        let args = BasicPublishArguments::default()
            .routing_key(queue_name)
            .finish();
        while let Some(msg) = rx.recv().await {
            log_handler::log(LOG::LogSentTask(msg.clone()));
            channel
                .basic_publish(
                    BasicProperties::default(),
                    msg.as_bytes().to_vec(),
                    args.clone(),
                )
                .await
                .unwrap();
        }
    }

    let (tx, rx) = mpsc::channel::<String>(100);
    let handle = tokio::spawn(publish_messages(
        rx,
        channel,
        queue_name.to_string(),
    ));
    (tx, handle)
}

pub async fn create_consumer(
    channel: &Channel,
    queue_name: &str,
) -> JoinHandle<()> {
    async fn receive_messages(
        mut rx: mpsc::UnboundedReceiver<ConsumerMessage>,
    ) {
        while let Some(msg) = rx.recv().await {
            let a = msg.content.unwrap();
            let s = String::from_utf8_lossy(&a);
            log_handler::log(LOG::LogReceivedTask(s.to_string()));
        }
    }

    let mut args = BasicConsumeArguments::new(queue_name, "");
    args.no_ack = true;
    let (_, rx) = channel.basic_consume_rx(args.clone()).await.unwrap();

    tokio::spawn(receive_messages(rx))
}
