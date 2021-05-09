use std::collections::HashMap;

use tokio::sync::{mpsc, oneshot};

use crate::{
    channel_actor::ChannelHandle,
    errors::{Error, Result},
};
use crate::{ID, MAX_MAILBOX_SIZE};

/// Registry of channels implementation as an actor resource
/// inspired by https://ryhl.io/blog/actors-with-tokio/
enum RegistryCommand {
    GetChannel {
        channel_id: ID,
        reply_to: oneshot::Sender<Result<ChannelHandle>>,
    },
}

struct RegistryActor {
    receiver: mpsc::Receiver<RegistryCommand>,
    // Internal state
    channels: HashMap<ID, ChannelHandle>,
}

impl RegistryActor {
    fn new(receiver: mpsc::Receiver<RegistryCommand>) -> Self {
        Self {
            receiver,
            channels: HashMap::new(),
        }
    }

    async fn on_start(&mut self) -> Result {
        Ok(())
    }

    fn handle_message(&mut self, msg: RegistryCommand) {
        match msg {
            RegistryCommand::GetChannel {
                channel_id,
                reply_to,
            } => {
                if let Some(c) = self.channels.get(&channel_id) {
                    let _ = reply_to.send(Ok(c.clone()));
                } else {
                    let c = ChannelHandle::new(channel_id);
                    self.channels.insert(channel_id, c.clone());
                    let _ = reply_to.send(Ok(c));
                }
            }
        }
    }
}

async fn run(mut actor: RegistryActor) {
    if let Err(_err) = actor.on_start().await {
        return; // Note here the actor terminates
    }
    while let Some(msg) = actor.receiver.recv().await {
        actor.handle_message(msg);
    }
}
/// Handle of the [`RegistryActor`]
/// Provides the public interface of the actor
#[derive(Clone)]
pub struct RegistryHandle {
    sender: mpsc::Sender<RegistryCommand>,
}

impl RegistryHandle {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel(MAX_MAILBOX_SIZE);
        let actor = RegistryActor::new(receiver);
        tokio::spawn(run(actor));
        Self { sender }
    }

    pub async fn get_channel(&self, channel_id: ID) -> Result<ChannelHandle> {
        let (reply_to, rx) = oneshot::channel();
        let msg = RegistryCommand::GetChannel {
            channel_id,
            reply_to,
        };

        let _ = self.sender.send(msg).await;
        rx.await.map_err(|_| Error::ActorUnexpectedTermination)?
    }
}
