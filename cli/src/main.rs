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
    backend::CrosstermBackend, layout::{Constraint, Direction, Layout}, text::{Line, Span, Text}, widgets::{Block, Borders, Paragraph}, Frame, Terminal
};
use tokio::io::split;

mod rabbit_service;
mod utils;

struct App<'a> {
    sent_log: Vec<Span<'a>>,
    received_log: Vec<Span<'a>>,
    workers_online: u8,
    queued_messages: u8,
}
#[tokio::main]
async fn main() {
    let _ = enable_raw_mode();
    stdout().execute(EnterAlternateScreen).unwrap();
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout())).unwrap();

    let mut should_quit = false;
    let mut app = App {
        sent_log: vec![],
        received_log: vec![],
        workers_online: 0,
        queued_messages: 0,
    };
    while !should_quit {
        let _ = terminal.draw(|frame| ui(frame, &app));
        should_quit = handle_events(&mut app).await.unwrap();
    }
}

async fn handle_events<'a>(app: &mut App<'a>) -> io::Result<bool> {
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
                        app.sent_log.push(span);

                        rabbit_service::publish("task-dispatcher", "Hello world!").await;
                    }
                    _ => {}
                }
            }
        }
    }
    Ok(false)
}

fn ui(frame: &mut Frame, app: &App) {
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(29), Constraint::Percentage(69)])
        .split(frame.size());

    let ui_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(29), Constraint::Percentage(69)])
        .split(layout[0]);

    let display_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(49), Constraint::Percentage(49)])
        .split(layout[1]);

    frame.render_widget(
        Paragraph::new("1. To Send\nq. To Quit")
            .block(Block::default()
                .title("Instructions")
                .borders(Borders::ALL)),
        ui_layout[0],
    );

    let rabbitmq_info = format!("Online workers: {}\nQueued messages: {}", 0, app.queued_messages);

    frame.render_widget(
        Paragraph::new(rabbitmq_info)
            .block(Block::default()
                .title("RabbitMQ info")
                .borders(Borders::ALL)),
        ui_layout[1],
    );
    
    
    let mut lines = vec![];
    
    for span in &app.sent_log {
        lines.push(Line::from(span.clone()))
    }

    let visible_window_height = display_layout[0].height;

    let scroll_offset: u16 = if lines.len() as u16 > visible_window_height - 2 {
        lines.len() as u16 - (visible_window_height - 2)
    } else {
        0 as u16
    };

    frame.render_widget(
        Paragraph::new(lines)
            .scroll((scroll_offset, 0))
            .block(Block::default()
                .title("Sent tasks")
                .borders(Borders::ALL)),
        display_layout[0],
    );


    let mut lines = vec![];
    
    for span in &app.received_log {
        lines.push(Line::from(span.clone()))
    }

    frame.render_widget(
        Paragraph::new(lines)
            .block(Block::default()
                .title("Received results")
                .borders(Borders::ALL)),
        display_layout[1],
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
