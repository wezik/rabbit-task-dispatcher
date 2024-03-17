use core::time;
use std::{collections::HashMap, io::stdout};

use amqprs::{
    callbacks::{DefaultChannelCallback, DefaultConnectionCallback},
    channel::{BasicConsumeArguments, Channel, QueueDeclareArguments},
    connection::{self, Connection, OpenConnectionArguments},
};
// use crossterm::{
//     terminal::{enable_raw_mode, EnterAlternateScreen},
//     ExecutableCommand,
// };
// use dotenv::dotenv;
use rabbit_service::{declare_queue, establish_channel, establish_connection, RabbitConnect};
// use ratatui::{backend::CrosstermBackend, text::Span, Terminal};
// use translations_handler::Translations;

// mod log_handler;
mod rabbit_service;
// mod translations_handler;
// mod tui_handler;
// mod utils;
//
// struct AppContext<'a> {
//     sent_logs: Vec<Span<'a>>,
//     received_logs: Vec<Span<'a>>,
//     workers_online: usize,
//     queued_messages: usize,
//     translations: HashMap<String, String>,
//     current_translation: Translations,
// }
//
// impl<'a> AppContext<'a> {
//     pub fn new() -> Self {
//         AppContext {
//             sent_logs: vec![],
//             received_logs: vec![],
//             workers_online: 0,
//             queued_messages: 0,
//             translations: translations_handler::get_translations(Translations::English),
//             current_translation: Translations::English,
//         }
//     }
// }

#[tokio::main]
async fn main() {
    let connection_details = RabbitConnect {
        host: "localhost".to_string(),
        port: 5672,
        username: "guest".to_string(),
        password: "guest".to_string(),
        vhost: "test".to_string(),
    };

    loop {
        let mut connection = establish_connection(&connection_details).await;
        println!("Started connection:{}", connection);
        let mut channel = establish_channel(&connection).await;
        println!("Started channel:{}", channel);
        declare_queue(&channel, "task-response").await;
        let mut args = BasicConsumeArguments::new("task-response", "");
        args.no_ack = true;
        let (_, mut messages_rx) = channel.basic_consume_rx(args.clone()).await.unwrap();
        println!("Launch succesfull, listening for messages!");
        while let Some(msg) = messages_rx.recv().await {
            let a = msg.content.unwrap();
            let s = String::from_utf8_lossy(&a);

            println!("received message: {}", s);
        }
    }
    // init();
    // let mut should_quit = false;
    //
    // let mut app_context = AppContext::new();
    // let mut terminal = Terminal::new(CrosstermBackend::new(stdout())).unwrap();
    //
    // while !should_quit {
    //     let _ = terminal.draw(|frame| tui_handler::ui(frame, &app_context));
    //     should_quit = tui_handler::handle_events(&mut app_context).await.unwrap();
    //
    //     let mut connection = get_connection().await;
    //     let channel = connection.open_channel(None).await.unwrap();
    //     channel.register_callback(DefaultChannelCallback).await.unwrap();
    //     let qparams = QueueDeclareArguments::default()
    //         .queue("task-respones".to_string())
    //         .auto_delete(true)
    //         .durable(false)
    //         .arguments(Default::default())
    //         .finish();
    //     channel.queue_declare(qparams).await.unwrap().unwrap();
    //
    //     let args = BasicConsumeArguments::new("task-response", "task-response");
    //     let (ctag, mut messages_rx) = channel.basic_consume_rx(args.clone()).await.unwrap();
    //
    //     while let Some(msg) = messages_rx.recv().await {
    //         let a = msg.content.unwrap();
    //         let body = String::from_utf8_lossy(&a).to_string();
    //         app_context.received_logs.push(Span::from(body));
    //     }
    // }
}

// fn init() {
//     dotenv().ok();
//     let _ = enable_raw_mode();
//     stdout().execute(EnterAlternateScreen).unwrap();
// }
