use chrono::Local;
use ratatui::text::Span;

use crate::AppContext;

pub enum LOG {
    LogSent(String),
    LogReceived(String),
}

pub fn log(log: LOG, app_context: &mut AppContext) {
    fn send_log(destination: &mut Vec<Span>, value: String) {
        let date = Local::now();
        let timestamp = date.format("[%Y-%m-%d][%H:%M:%S]");
        destination.push(Span::from(format!("{} - {}", timestamp, value)));
    }

    match log {
        LOG::LogSent(val) => {
            let destination = &mut app_context.sent_logs;
            send_log(destination, val);
        }
        LOG::LogReceived(val) => {
            send_log(&mut app_context.sent_logs, "test".to_string());
            let destination = &mut app_context.received_logs;
            send_log(destination, val);
        }
    }
}
