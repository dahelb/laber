use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    str::FromStr,
    sync::{
        mpsc::{channel, Receiver},
        Arc, RwLock,
    },
    thread,
};

use client::{Client, Clients};
use message::Message;
use tracing::{info, span, Level};

pub mod client;
pub mod message;

fn write_worker(mut stream: TcpStream, r: Receiver<Message>) -> std::io::Result<()> {
    for m in r.iter() {
        write!(stream, "{}\r\n", m)?;
    }
    Ok(())
}

fn read_worker(stream: TcpStream, clients: Arc<RwLock<Clients>>) -> std::io::Result<()> {
    let peer_addr = stream.peer_addr().expect("Failed to extract peer addr");

    let _client_read_span = span!(Level::INFO, "read-worker", addr = ?peer_addr).entered();
    let mut buf = String::new();

    let mut reader = BufReader::new(&stream);
    while let Ok(nbytes) = reader.read_line(&mut buf) {
        if nbytes == 0 {
            info!("Socket closed.");
            break;
        }

        if buf.ends_with("\r\n") {
            let buf_trimmed = buf.trim_end();
            info!("New message received from client");
            match Message::from_str(&buf_trimmed) {
                Ok(m) => {
                    let read_lock = clients.read().expect("Unlock failed.");
                    read_lock.broadcast(m).unwrap();
                }
                Err(e) => {
                    let read_lock = clients.read().expect("Unlock failed.");
                    read_lock
                        .send_to_client(&peer_addr, Message::from(e))
                        .unwrap();
                }
            }
        }

        buf.clear();
    }

    // If the client disconnects, remove them from the clients list
    info!("Removing {} from clients.", stream.peer_addr().unwrap());

    let mut clients_lock = clients.write().unwrap();
    clients_lock.remove_client(&peer_addr);
    drop(clients_lock);

    Ok(())
}

pub fn start_server(listen_addr: &str) -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(listen_addr)?;

    info!("Started new listener at {}", listen_addr);

    let clients = Arc::new(RwLock::new(Clients::new()));

    for stream in listener.incoming() {
        let stream = match stream {
            Ok(s) => s,
            Err(_) => {
                continue;
            }
        };

        let (s, r) = channel();

        let peer_addr = stream.peer_addr().expect("Could not extract peer address.");

        let stream_clone = stream.try_clone().expect("Cloning stream failed");

        thread::spawn(move || write_worker(stream, r));

        info!("New connection");

        let clients = clients.clone();
        let mut clients_lock = clients.write().unwrap();
        clients_lock
            .add_client(peer_addr, Client::new(s))
            .expect("Adding client failed");

        clients_lock.send_to_client(
            &peer_addr,
            Message::System("Welcome to the chat server!".to_string()),
        )?;

        drop(clients_lock);

        thread::spawn(move || read_worker(stream_clone, clients));
    }

    Ok(())
}
