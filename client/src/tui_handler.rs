use std::io;

use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::{
    log_handler::{self, LOG},
    rabbit_service,
    translations_handler::{self, load_translation, Translations},
    AppContext,
};

pub fn ui(frame: &mut Frame, app_context: &AppContext) {
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(frame.size());

    let ui_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(layout[0]);

    let display_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(layout[1]);

    let keys = vec![
        "menu.instructions.1",
        "menu.instructions.2",
        "menu.instructions.3",
        "menu.instructions.q",
    ];
    let translations = keys
        .iter()
        .map(|key| load_translation(key, app_context))
        .map(Span::from)
        .collect::<Vec<Span>>();

    let lines = translations
        .iter()
        .map(|span| Line::from(span.clone()))
        .collect::<Vec<_>>();

    let menu_paragraph = Paragraph::new(lines).block(
        Block::default()
            .title(load_translation("menu.title", app_context))
            .borders(Borders::ALL),
    );
    frame.render_widget(menu_paragraph, ui_layout[0]);

    let workers = load_translation("rabbitmq.data.workers", app_context);
    let tasks = load_translation("rabbitmq.data.tasks", app_context);

    let workers_span = Span::from(format!("{}{}", workers, 0));
    let tasks_span = Span::from(format!("{}{}", tasks, app_context.queued_messages));

    let spans = vec![workers_span, tasks_span];

    let lines = spans
        .iter()
        .map(|span| Line::from(span.clone()))
        .collect::<Vec<_>>();

    let info_paragraph = Paragraph::new(lines).block(
        Block::default()
            .title(load_translation("rabbitmq.title", app_context))
            .borders(Borders::ALL),
    );
    frame.render_widget(info_paragraph, ui_layout[1]);

    frame.render_widget(
        create_logs_widget(
            &app_context.sent_logs,
            display_layout[0].height,
            &load_translation("logs.sent.title", app_context),
        ),
        display_layout[0],
    );

    frame.render_widget(
        create_logs_widget(
            &app_context.received_logs,
            display_layout[1].height,
            &load_translation("logs.received.title", app_context),
        ),
        display_layout[1],
    );
}

fn create_logs_widget<'a>(logs: &'a Vec<Span>, content_height: u16, title: &str) -> Paragraph<'a> {
    let scroll_offset: u16 = if logs.len() as u16 > content_height - 2 {
        logs.len() as u16 - (content_height - 2)
    } else {
        0
    };

    let lines = logs
        .iter()
        .skip(scroll_offset as usize)
        .map(|span| Line::from(span.clone()))
        .collect::<Vec<_>>();

    Paragraph::new(lines).block(
        Block::default()
            .title(title.to_owned())
            .borders(Borders::ALL),
    )
}

pub async fn handle_events<'a>(app: &mut AppContext<'a>) -> io::Result<bool> {
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
