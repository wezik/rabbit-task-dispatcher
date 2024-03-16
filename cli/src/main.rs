use std::{io, process::exit};

use dotenv::dotenv;
use log::{debug, info};

mod rabbit_service;
mod utils;

#[tokio::main]
async fn main() {
    init();

    info!("================ Rabbit Task Dispatcher ================");

    let stdin = io::stdin();
    let mut input = String::new();

    let queue_name = utils::get_env_var("RABBITMQ_QUEUE", "task-dispatcher", true);

    loop {
        println!("\n=======| 1. Send tasks to RabbitMQ | 2. Exit |=======");
        input.clear();
        let _ = stdin.read_line(&mut input);

        match input.trim() {
            "1" => {
                let messages = vec!["Hello world RabbitmQ!"];
                for message in messages {
                    rabbit_service::publish(&queue_name, message).await;
                }
            }
            "2" => {
                exit(0);
            }
            _ => {
                println!("Invalid input. Please enter 1 or 2");
            }
        }
    }
}

fn init() {
    // Init env variables
    dotenv().ok();

    // Init logger
    env_logger::init();
    let log_level = utils::get_env_var("RUST_LOG", "INFO", true);
    debug!("Log level set to '{}'", log_level);
}
