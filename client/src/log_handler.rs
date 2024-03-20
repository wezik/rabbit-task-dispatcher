use tokio::sync::mpsc;

pub enum Log {
    Info(String),
    Debug(String),
    Error(String),
    SentTask(String),
    ReceivedTask(String),
}

static mut LOGGER_TX: Option<mpsc::Sender<Log>> = None;

pub fn get_logger_tx<'a>() -> mpsc::Sender<Log> {
    unsafe {
        if LOGGER_TX.is_none() {
            let (tx, mut rx) = mpsc::channel(100);
            tokio::spawn(async move {
                while let Some(log) = rx.recv().await {
                    match log {
                        Log::Info(log) => handle_info_log(log),
                        Log::Debug(log) => handle_debug_log(log),
                        Log::Error(log) => handle_error_log(log),
                        Log::SentTask(log) => handle_sent_task(log),
                        Log::ReceivedTask(log) => handle_received_task(log),
                    }
                }
            });
            LOGGER_TX = Some(tx);
        }
        LOGGER_TX.clone().unwrap()
    }
}

fn handle_info_log(msg: String) {
    println!("Log: {}", msg);
}

fn handle_debug_log(msg: String) {
    println!("Debug: {}", msg);
}

fn handle_error_log(msg: String) {
    println!("Error: {}", msg);
}

fn handle_sent_task(msg: String) {
    println!("Sent task: {}", msg);
}

fn handle_received_task(msg: String) {
    println!("Received task: {}", msg);
}
