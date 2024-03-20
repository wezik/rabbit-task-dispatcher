use tokio::sync::mpsc::{self, Sender};

pub enum ContextOP {
    None,
    Log(String),
    LogSent(String),
    LogReceived(String),
    TasksTotal(usize),
    WorkersOnline(usize),
    Translation,
}

pub fn register_event(context_op: ContextOP) {
    todo!();
}

pub fn get_context_sender() -> Sender<ContextOP> {
    todo!();
}
