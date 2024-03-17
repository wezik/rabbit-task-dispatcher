use core::time;

use amqprs::{
    callbacks::{DefaultChannelCallback, DefaultConnectionCallback},
    channel::{self, Channel, QueueDeclareArguments},
    connection::{Connection, OpenConnectionArguments},
};

pub struct RabbitConnect {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub vhost: String,
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
        .connection_name("rust-client"),
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
            .connection_name("rust-client"),
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
    let mut args = QueueDeclareArguments::new(queue_name);
    args.durable(true); //golang amq lib sets it by default
    channel.queue_declare(args).await.unwrap().unwrap();
}
