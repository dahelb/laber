use std::{
    io,
    net::TcpStream,
    sync::{Arc, RwLock, mpsc::Receiver},
};

use common::{message::{Message, MessageParseError}, tcp_worker};

// If the client needs custom message formatting, we can keep this specialized version
pub fn send_worker(stream: &mut TcpStream, r: Receiver<Message>) -> io::Result<()> {
    tcp_worker::send_worker(stream, r)
}

struct ClientMessageHandler {
    messages: Arc<RwLock<Vec<Message>>>,
}

impl tcp_worker::MessageHandler for ClientMessageHandler {
    fn handle_message(&self, message_result: Result<Message, MessageParseError>) {
        if let Ok(m) = message_result {
            let mut messages_lock = self.messages.write().expect("locking failed.");
            messages_lock.push(m);
        }
    }
}

pub fn read_worker(stream: TcpStream, messages: Arc<RwLock<Vec<Message>>>) {
    let handler = ClientMessageHandler { messages };
    let _ = tcp_worker::read_worker(stream, handler);
}
