use std::{
    collections::HashMap,
    io::stdout,
};

use crossterm::{
    terminal::{enable_raw_mode, EnterAlternateScreen},
    ExecutableCommand,
};
use dotenv::dotenv;
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

impl<'a> AppContext<'a> {
    pub fn new() -> Self {
        AppContext {
            sent_logs: vec![],
            received_logs: vec![],
            workers_online: 0,
            queued_messages: 0,
            translations: translations_handler::get_translations(Translations::English),
            current_translation: Translations::English,
        }
    }
}

#[tokio::main]
async fn main() {
    init();
    let mut should_quit = false;

    let mut app_context = AppContext::new();
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout())).unwrap();

    while !should_quit {
        let _ = terminal.draw(|frame| tui_handler::ui(frame, &app_context));
        should_quit = tui_handler::handle_events(&mut app_context).await.unwrap();
    }
}

fn init() {
    dotenv().ok();
    let _ = enable_raw_mode();
    stdout().execute(EnterAlternateScreen).unwrap();
}
