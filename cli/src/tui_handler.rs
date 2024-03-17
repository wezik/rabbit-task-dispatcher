use ratatui::{
    layout::{Constraint, Direction, Layout},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::AppContext;

pub fn ui(frame: &mut Frame, app_context: &AppContext) {
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(frame.size());

    let ui_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(layout[0]);

    let display_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(layout[1]);

    let menu_paragraph = Paragraph::new("menu.instructions")
        .block(Block::default().title("menu.title").borders(Borders::ALL));
    frame.render_widget(menu_paragraph, ui_layout[0]);

    let rabbitmq_info = format!("{}{}", 0, app_context.queued_messages);
    let info_paragraph = Paragraph::new(rabbitmq_info)
        .block(Block::default().title("info.title").borders(Borders::ALL));
    frame.render_widget(info_paragraph, ui_layout[1]);

    frame.render_widget(
        create_logs_widget(
            &app_context.sent_logs,
            display_layout[0].height,
            "logs.sent.title",
        ),
        display_layout[0],
    );

    frame.render_widget(
        create_logs_widget(
            &app_context.received_logs,
            display_layout[1].height,
            "logs.received.title",
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
