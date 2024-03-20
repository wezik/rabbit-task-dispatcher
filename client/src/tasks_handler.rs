use crate::rabbit_service;

pub async fn handle_response(msg: String) {
    println!("Received '{}' message from 'task-response'", msg);
}

pub async fn handle_sending(msg: String) {
    println!("Sending '{}' message", msg);
}
