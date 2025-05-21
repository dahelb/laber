use std::{
    net::{TcpListener, TcpStream},
    sync::{
        mpsc::{channel, Receiver},
        Arc, RwLock,
    },
    thread,
};

use client::{Client, Clients};
use common::{message::{Message, MessageParseError}, tcp_worker};
use tracing::{info, span, Level};

pub mod client;

struct ServerMessageHandler {
    clients: Arc<RwLock<Clients>>,
    peer_addr: std::net::SocketAddr,
}

impl tcp_worker::MessageHandler for ServerMessageHandler {
    fn handle_message(&self, message_result: Result<Message, MessageParseError>) {
        match message_result {
            Ok(m) => {
                info!("New message received from client");
                let read_lock = self.clients.read().expect("Unlock failed.");
                read_lock.broadcast(m).unwrap();
            }
            Err(e) => {
                let read_lock = self.clients.read().expect("Unlock failed.");
                read_lock
                    .send_to_client(&self.peer_addr, Message::from(e))
                    .unwrap();
            }
        }
    }
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
        let mut stream_for_writing = stream.try_clone().expect("Cloning stream failed");

        thread::spawn(move || tcp_worker::send_worker(&mut stream_for_writing, r));

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

        let clients_for_reading = clients.clone();
        let handler = ServerMessageHandler {
            clients: clients_for_reading,
            peer_addr,
        };

        thread::spawn(move || {
            let result = tcp_worker::read_worker(stream_clone, handler);
            if let Err(e) = result {
                info!("Read worker error: {}", e);
            }
            
            // If the client disconnects, remove them from the clients list
            info!("Removing {} from clients.", peer_addr);
            let mut clients_lock = clients.write().unwrap();
            clients_lock.remove_client(&peer_addr);
        });
    }

    Ok(())
}
