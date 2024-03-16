use std::{io::{self, stdout}, process::exit};

use crossterm::{event::{self, Event, KeyCode}, terminal::{enable_raw_mode, EnterAlternateScreen}, ExecutableCommand};
use dotenv::dotenv;
use log::{debug, info};
use ratatui::{backend::CrosstermBackend, layout::{Constraint, Direction, Layout}, widgets::{Block, Borders, Paragraph}, Frame, Terminal};
use tokio::io::split;

mod rabbit_service;
mod utils;

struct Body {
    text: String
}
#[tokio::main]
async fn main() {
    let _ = enable_raw_mode();
    stdout().execute(EnterAlternateScreen).unwrap();
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout())).unwrap();

    let mut should_quit = false;
    let mut body = Body{ text: "".to_string() };
    while !should_quit {
        let _ = terminal.draw(|frame| ui(frame, &body));
        should_quit = handle_events(&mut body).await.unwrap();
    }

/*     init();

    info!("================ Rabbit Task Dispatcher ================");

    let stdin = io::stdin();
    let mut input = String::new();

    let publish_queue = utils::get_env_var("RABBITMQ_PUBLLISH_QUEUE", "task-dispatcher", true);
    // let consumer_queue = utils::get_env_var("RABBITMQ_CONSUMER_QUEUE", "task-return", true);

    loop {
        println!("\n=======| 1. Send tasks to RabbitMQ | 2. Exit |=======");
        input.clear();
        let _ = stdin.read_line(&mut input);

        match input.trim() {
            "1" => {
                let messages = vec![
                    "Hello world RabbitMQ!",
                    "Message 1",
                    "Message 2",
                    "Message 3",
                    "Message 4",
                    "Message 5",
                    "Message 6",
                ];
                for message in messages {
                    rabbit_service::publish(&publish_queue, message).await;
                }
            }
            "2" => {
                exit(0);
            }
            _ => {
                println!("Invalid input. Please enter 1 or 2");
            }
        }
    } */
}

async fn handle_events(body: &mut Body) -> io::Result<bool> {
    if event::poll(std::time::Duration::from_millis(50)).unwrap() {
        if let Event::Key(key) = event::read().unwrap() {
            if key.kind == event::KeyEventKind::Press {

                match key.code {
                    KeyCode::Char('q') => {
                        return Ok(true);
                    }

                    KeyCode::Char('1') => {

                        let line = format!("Publish task id: {} to 'task-dispatcher' queue\n",body.text.len());
                        body.text += &line; 
                        rabbit_service::publish("task-dispatcher", "Hello world!").await;
                    }
                    _ => {

                    }
                }
            }
        }
    }
    Ok(false)
}

fn ui(frame: &mut Frame, body: &Body) {

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(frame.size());


    frame.render_widget(
        Paragraph::new("Hello world :)").block(Block::default().title("Rabbit task dispatcher").borders(Borders::ALL)),
        layout[0],
    );

    frame.render_widget(
        Paragraph::new(body.text.to_string()).block(Block::default().title("Recieved messages | 1. 'Send Hello World' message | q. Quit").borders(Borders::ALL)),
        layout[1],
    );
}

fn init() {
    // Init env variables
    dotenv().ok();

    // Init logger
    env_logger::init();
    let log_level = utils::get_env_var("RUST_LOG", "INFO", true);
    debug!("Log level set to '{}'", log_level);
}
