use std::collections::HashMap;
use tokio::sync::{
    mpsc::{self, UnboundedSender},
    oneshot,
};
use uuid::Uuid;
use warp::{ws::Message, Error};

const MAX_MAILBOX_SIZE: usize = 1024;
/// Server implementation as an actor like resource
/// inspired by https://ryhl.io/blog/actors-with-tokio/
enum ServerCommand {
    Connect {
        sender: UnboundedSender<Result<Message, Error>>,
        reply_to: oneshot::Sender<u128>,
    },
    Disconnect {
        connection_id: u128,
        reply_to: oneshot::Sender<()>,
    },
}

struct Server {
    receiver: mpsc::Receiver<ServerCommand>,
    // Internal state
    connections: HashMap<u128, mpsc::UnboundedSender<Result<Message, Error>>>,
}

impl Server {
    fn new(receiver: mpsc::Receiver<ServerCommand>) -> Self {
        Server {
            receiver,
            connections: HashMap::default(),
        }
    }

    fn handle_message(&mut self, msg: ServerCommand) {
        match msg {
            ServerCommand::Connect { reply_to, sender } => {
                let connection_id = Uuid::new_v4().as_u128();
                self.connections.insert(connection_id, sender);

                if let Some(c) = self.connections.get(&connection_id) {
                    let _ = c.send(Ok(Message::text("connected <server>")));
                }

                let _ = reply_to.send(connection_id);
            }

            ServerCommand::Disconnect {
                connection_id,
                reply_to,
            } => {
                let _ = connection_id;
                self.connections.remove(&connection_id);

                let _ = reply_to.send(());
            }
        }
    }
}

async fn run(mut server: Server) {
    while let Some(msg) = server.receiver.recv().await {
        server.handle_message(msg);
    }
}
#[derive(Clone)]
pub struct ServerHandle {
    sender: mpsc::Sender<ServerCommand>,
}
impl ServerHandle {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel(MAX_MAILBOX_SIZE);
        let server = Server::new(receiver);
        tokio::spawn(run(server));
        Self { sender }
    }

    pub async fn connect(&self, sender: mpsc::UnboundedSender<Result<Message, Error>>) -> u128 {
        let (reply_to, rx) = oneshot::channel();
        let msg = ServerCommand::Connect { sender, reply_to };

        let _ = self.sender.send(msg).await;
        rx.await.expect("Server has been terminated")
    }

    pub async fn disconnect(&self, connection_id: u128) -> () {
        let (reply_to, rx) = oneshot::channel();
        let msg = ServerCommand::Disconnect {
            connection_id,
            reply_to,
        };

        let _ = self.sender.send(msg).await;
        rx.await.expect("Server has been terminated")
    }
}
