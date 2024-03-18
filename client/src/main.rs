use dotenv::dotenv;
use rabbit_service::{declare_queue, establish_channel, establish_connection, RabbitConnect};
use utils::read_env;

mod context_handler;
mod log_handler;
mod rabbit_service;
mod utils;

struct AppContext {
    logs: Vec<String>,
    tasks_sent: Vec<String>,
    tasks_received: Vec<String>,
    workers_connected: usize,
    total_tasks: usize,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let context = AppContext {
        logs: vec![],
        tasks_sent: vec![],
        tasks_received: vec![],
        workers_connected: 0,
        total_tasks: 0,
    };

    let connection_details = RabbitConnect {
        host: read_env("RABBITMQ_HOST", "localhost", true),
        port: read_env("RABBITMQ_PORT", "5672", true)
            .parse()
            .expect("RABBITMQ_PORT is not a number"),
        username: read_env("RABBITMQ_USERNAME", "guest", true),
        password: read_env("RABBITMQ_PASSWORD", "guest", true),
        vhost: read_env("RABBITMQ_VHOST", "/", true),
        connection_name: read_env("RABBITMQ_CONNECTION_NAME", "rust-client", true),
    };

    let pub_queue = read_env("RABBITMQ_PUBLISH_QUEUE", "task-dispatcher", true);
    let cons_queue = read_env("RABBITMQ_CONSUMER_QUEUE", "task-response", true);
    let connection = establish_connection(&connection_details).await;
    let pub_channel = establish_channel(&connection).await;
    let cons_channel = establish_channel(&connection).await;
    declare_queue(&pub_channel, &pub_queue).await;
    declare_queue(&cons_channel, &cons_queue).await;

    let (tx, pub_handle) =
        rabbit_service::create_publisher(pub_channel, &pub_queue).await;
    let cons_handle =
        rabbit_service::create_consumer(&cons_channel, &cons_queue).await;

    for i in 0..25 {
        let message = format!("Task id '{}'", i);
        let _ = tx.send(message).await;
    }

    let _ = tokio::join!(pub_handle, cons_handle);
}

// fn init() {
//     dotenv().ok();
//     let _ = enable_raw_mode();
//     stdout().execute(EnterAlternateScreen).unwrap();
// }
