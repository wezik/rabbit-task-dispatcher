use std::{
    io::{self, stdout},
    process::exit,
};

use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{enable_raw_mode, EnterAlternateScreen},
    ExecutableCommand,
};
use dotenv::dotenv;
use log::{debug, info};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};
use tokio::io::split;

mod rabbit_service;
mod tui_handler;
mod utils;

struct AppContext<'a> {
    sent_logs: Vec<Span<'a>>,
    received_logs: Vec<Span<'a>>,
    workers_online: usize,
    queued_messages: usize,
}
#[tokio::main]
async fn main() {
    let _ = enable_raw_mode();
    stdout().execute(EnterAlternateScreen).unwrap();
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout())).unwrap();

    let mut should_quit = false;
    let mut app = AppContext {
        sent_logs: vec![],
        received_logs: vec![],
        workers_online: 0,
        queued_messages: 0,
    };
    while !should_quit {
        let _ = terminal.draw(|frame| tui_handler::ui(frame, &app));
        should_quit = handle_events(&mut app).await.unwrap();
    }
}

async fn handle_events<'a>(app: &mut AppContext<'a>) -> io::Result<bool> {
    if event::poll(std::time::Duration::from_millis(50)).unwrap() {
        if let Event::Key(key) = event::read().unwrap() {
            if key.kind == event::KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') => {
                        return Ok(true);
                    }

                    KeyCode::Char('1') => {
                        let span = Span::raw(format!(
                            "Publish task id: {} to 'task-dispatcher' queue\n",
                            app.queued_messages
                        ));
                        app.queued_messages += 1;
                        app.sent_logs.push(span);

                        rabbit_service::publish("task-dispatcher", "Hello world!").await;
                    }

                    KeyCode::Char('2') => {
                        for _ in 0..250 {
                            app.queued_messages += 1;
                            rabbit_service::publish("task-dispatcher", "Hello world!").await;
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    Ok(false)
}

fn init() {
    // Init env variables
    dotenv().ok();

    // Init logger
    env_logger::init();
    let log_level = utils::get_env_var("RUST_LOG", "INFO", true);
    debug!("Log level set to '{}'", log_level);
}
