use std::{
    collections::{HashMap, HashSet},
    io::{self, stdout},
};

use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{enable_raw_mode, EnterAlternateScreen},
    ExecutableCommand,
};
use dotenv::dotenv;
use log::debug;
use log_handler::LOG;
use ratatui::{backend::CrosstermBackend, text::Span, Terminal};
use translations_handler::Translations;

mod log_handler;
mod rabbit_service;
mod translations_handler;
mod tui_handler;
mod utils;

struct AppContext<'a> {
    sent_logs: Vec<Span<'a>>,
    received_logs: Vec<Span<'a>>,
    workers_online: usize,
    queued_messages: usize,
    translations: HashMap<String, String>,
    current_translation: Translations,
}
#[tokio::main]
async fn main() {
    init();
    let _ = enable_raw_mode();
    stdout().execute(EnterAlternateScreen).unwrap();
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout())).unwrap();

    let mut should_quit = false;

    let mut app = AppContext {
        sent_logs: vec![],
        received_logs: vec![],
        workers_online: 0,
        queued_messages: 0,
        translations: translations_handler::get_translations(Translations::English),
        current_translation: Translations::English,
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
                        log_handler::log(
                            LOG::LogSent(format!(
                                "Publish task id: {} to 'task-dispatcher' queue\n",
                                app.queued_messages
                            )),
                            app,
                        );
                        app.queued_messages += 1;

                        rabbit_service::publish("task-dispatcher", "Hello world!").await;
                    }

                    KeyCode::Char('2') => {
                        for _ in 0..250 {
                            app.queued_messages += 1;
                            rabbit_service::publish("task-dispatcher", "Hello world!").await;
                        }
                    }
                    KeyCode::Char('3') => match app.current_translation {
                        Translations::English => {
                            app.translations =
                                translations_handler::get_translations(Translations::Polish);
                            app.current_translation = Translations::Polish;
                        }
                        _ => {
                            app.translations =
                                translations_handler::get_translations(Translations::English);
                            app.current_translation = Translations::English;
                        }
                    },
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
}
