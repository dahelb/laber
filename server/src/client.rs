use std::{collections::HashMap, net::SocketAddr, sync::mpsc::SendError};

use crate::message::Message;

#[derive(Debug, thiserror::Error)]
pub enum ClientError {
    #[error("The client with peer address {0} could not be found!")]
    NotFound(SocketAddr),
    #[error("Sending a message failed.")]
    SendFail(#[from] SendError<Message>),
}

pub struct Client {
    sender: std::sync::mpsc::Sender<Message>,
}

impl Client {
    pub fn new(sender: std::sync::mpsc::Sender<Message>) -> Self {
        Client { sender }
    }
    pub fn send_message(&self, message: Message) -> Result<(), SendError<Message>> {
        self.sender.send(message)?;
        Ok(())
    }
}

pub struct Clients {
    clients: HashMap<SocketAddr, Client>,
}

impl Clients {
    pub fn new() -> Self {
        Clients {
            clients: HashMap::new(),
        }
    }

    pub fn add_client(&mut self, peer_addr: SocketAddr, client: Client) -> std::io::Result<()> {
        self.clients.insert(peer_addr, client);

        Ok(())
    }

    pub fn remove_client(&mut self, peer_addr: &SocketAddr) -> Option<Client> {
        self.clients.remove(peer_addr)
    }

    pub fn broadcast(&self, message: Message) -> Result<(), SendError<Message>> {
        for client in self.clients.values() {
            client.send_message(message.clone())?;
        }
        Ok(())
    }

    pub fn send_to_client(
        &self,
        peer_addr: &SocketAddr,
        message: Message,
    ) -> Result<(), ClientError> {
        let client = self
            .clients
            .get(peer_addr)
            .ok_or(ClientError::NotFound(*peer_addr))?;

        client.send_message(message)?;

        Ok(())
    }
}
