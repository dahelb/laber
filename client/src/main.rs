use std::{
    net::TcpStream,
    sync::{Arc, RwLock, mpsc},
    thread,
};

use client::{
    app::App,
    workers::{read_worker, send_worker},
};

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:9999")?;
    let read_stream = stream.try_clone().expect("Cloning stream failed");

    let messages = Arc::new(RwLock::new(Vec::new()));

    let (s, r) = mpsc::channel();

    let messages_clone = messages.clone();
    thread::spawn(move || read_worker(read_stream, messages_clone));
    thread::spawn(move || send_worker(&mut stream, r));

    let app = App::new(messages, s);
    let terminal = ratatui::init();

    let res = app.run(terminal);

    ratatui::restore();

    Ok(())
}
