use std::collections::{HashMap, HashSet};
use tokio::sync::{
    mpsc::{self, UnboundedSender},
    oneshot,
};
use uuid::Uuid;
use warp::{ws::Message as WsMessage, Error as WsError};

use crate::{
    channel_actor::ChannelHandle,
    errors::{Error, Result},
    websocket::ServerMessage,
    UserId, MAX_MAILBOX_SIZE,
};

/// Server implementation as an actor like resource
/// inspired by https://ryhl.io/blog/actors-with-tokio/
enum ServerCommand {
    Connect {
        sender: UnboundedSender<Result<WsMessage, WsError>>,
        reply_to: oneshot::Sender<Result<u128>>,
    },
    Disconnect {
        connection_id: u128,
        reply_to: oneshot::Sender<Result>,
    },
    RegisterUser {
        connection_id: u128,
        user: UserId,
        reply_to: oneshot::Sender<Result>,
    },
    PublishToChannel {
        users: Vec<UserId>,
        message: ServerMessage,
        reply_to: oneshot::Sender<Result>,
    },
}

struct ServerActor {
    receiver: mpsc::Receiver<ServerCommand>,
    // Internal state

    // Maps connection_id -> websocket sender
    connections: HashMap<u128, mpsc::UnboundedSender<Result<WsMessage, WsError>>>,

    // Maps connection -> user
    users_inverse: HashMap<u128, UserId>,

    // Maps user  -> many  connection_id
    users: HashMap<UserId, HashSet<u128>>,
}

impl ServerActor {
    fn new(receiver: mpsc::Receiver<ServerCommand>) -> Self {
        ServerActor {
            receiver,
            connections: HashMap::default(),
            users_inverse: HashMap::default(),
            users: HashMap::default(),
        }
    }

    fn handle_message(&mut self, msg: ServerCommand) {
        match msg {
            ServerCommand::Connect { reply_to, sender } => {
                let connection_id = Uuid::new_v4().as_u128();
                self.connections.insert(connection_id, sender);

                let _ = reply_to.send(Ok(connection_id));
            }

            ServerCommand::Disconnect {
                connection_id,
                reply_to,
            } => {
                self.connections.remove(&connection_id);
                if let Some(user_id) = self.users_inverse.remove(&connection_id) {
                    self.users.remove(&user_id);
                }

                let _ = reply_to.send(Ok(()));
            }
            ServerCommand::RegisterUser {
                connection_id,
                user,
                reply_to,
            } => {
                self.users_inverse.insert(connection_id, user.clone());
                if let Some(connections) = self.users.get_mut(&user) {
                    connections.insert(connection_id);
                } else {
                    let mut connections = HashSet::new();
                    connections.insert(connection_id);
                    self.users.insert(user.clone(), connections);
                }

                let _ = reply_to.send(Ok(()));
            }
            ServerCommand::PublishToChannel {
                users,
                message,
                reply_to,
            } => {
                let message = match serde_json::to_string(&message) {
                    Err(err) => {
                        let _ = reply_to.send(Err(Error::Json(err)));
                        return;
                    }
                    Ok(m) => m,
                };

                let message = WsMessage::text(message);

                for u in users {
                    if let Some(connections) = self.users.get(&u) {
                        for connection_id in connections {
                            if let Some(c) = self.connections.get(connection_id) {
                                // TODO handle send error this happens if the receiver of the websocket connection is closed
                                // That is the connection is closed (we should at list log this error)
                                let _ = c.send(Ok(message.clone()));
                            }
                        }
                    }
                }
                let _ = reply_to.send(Ok(()));
            }
        }
    }
}

async fn run(mut actor: ServerActor) {
    while let Some(msg) = actor.receiver.recv().await {
        actor.handle_message(msg);
    }
}
#[derive(Clone)]
pub struct ServerHandle {
    sender: mpsc::Sender<ServerCommand>,
}
impl ServerHandle {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel(MAX_MAILBOX_SIZE);
        let actor = ServerActor::new(receiver);
        tokio::spawn(run(actor));
        Self { sender }
    }

    pub async fn connect(
        &self,
        sender: mpsc::UnboundedSender<Result<WsMessage, WsError>>,
    ) -> Result<u128> {
        let (reply_to, rx) = oneshot::channel();
        let msg = ServerCommand::Connect { sender, reply_to };

        let _ = self.sender.send(msg).await;
        rx.await.map_err(|_| Error::ActorUnexpectedTermination)?
    }

    pub async fn disconnect(&self, connection_id: u128) -> Result<()> {
        let (reply_to, rx) = oneshot::channel();
        let msg = ServerCommand::Disconnect {
            connection_id,
            reply_to,
        };

        let _ = self.sender.send(msg).await;
        rx.await.map_err(|_| Error::ActorUnexpectedTermination)?
    }

    // TODO remove this for now we registrer UserId manually in the future pass the UseId to the connection
    pub async fn register_user(&self, connection_id: u128, user: UserId) -> Result {
        let (reply_to, rx) = oneshot::channel();
        let msg = ServerCommand::RegisterUser {
            connection_id,
            user,
            reply_to,
        };

        let _ = self.sender.send(msg).await;
        rx.await.map_err(|_| Error::ActorUnexpectedTermination)?
    }

    pub async fn publish_to_channel(
        &self,
        channel: &ChannelHandle,
        message: ServerMessage,
    ) -> Result {
        let (reply_to, rx) = oneshot::channel();
        let users: Vec<UserId> = channel.get_channel_users().await?;
        let msg = ServerCommand::PublishToChannel {
            users,
            message,
            reply_to,
        };

        let _ = self.sender.send(msg).await;
        rx.await.map_err(|_| Error::ActorUnexpectedTermination)?
    }
}
