use std::{io, process::exit};

use dotenv::dotenv;
use log::{info, debug};

use crate::rabbit_service::RabbitService;

mod rabbit_service;
mod utils;

#[tokio::main]
async fn main() {
    init();
    info!("================ Rabbit Task Dispatcher ================");

    let rabbitmq = RabbitService::new().await;
    let stdin = io::stdin();
    let mut input = String::new();
    
    loop {
        println!("\n=======| 1. Send a task to RabbitMQ | 2. Exit |=======");
        input.clear();
        let _ = stdin.read_line(&mut input);

        match input.trim() {
            "1" => {
                rabbitmq.publish("Hello world RabbitmQ!").await;
            },
            "2" => {
                exit(0);
            },
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
