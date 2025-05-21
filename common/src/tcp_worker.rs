use std::{
    io::{self, BufRead, BufReader, Write},
    net::TcpStream,
    str::FromStr,
    sync::mpsc::Receiver,
};

use crate::message::{Message, MessageParseError};

/// Handles sending messages over a TCP stream
pub fn send_worker(stream: &mut TcpStream, receiver: Receiver<Message>) -> io::Result<()> {
    for message in receiver.iter() {
        write!(stream, "{}\r\n", message)?;
    }
    Ok(())
}

/// A trait for handling received messages
pub trait MessageHandler {
    fn handle_message(&self, message: Result<Message, MessageParseError>);
}

/// Handles reading messages from a TCP stream
pub fn read_worker<H: MessageHandler>(stream: TcpStream, handler: H) -> io::Result<()> {
    let mut buf = String::new();
    let mut reader = BufReader::new(stream);

    while let Ok(nbytes) = reader.read_line(&mut buf) {
        if nbytes == 0 {
            break;
        }

        if buf.ends_with("\r\n") {
            let buf_trimmed = buf.trim_end();
            handler.handle_message(Message::from_str(buf_trimmed));
        }

        buf.clear();
    }

    Ok(())
}