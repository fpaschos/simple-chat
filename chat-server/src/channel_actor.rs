use tokio::sync::{mpsc, oneshot};

use crate::{channel::Channel, errors::Error, ID, MAX_MAILBOX_SIZE};
use crate::{channel::Message, errors::Result, UserId};

/// Channel implementation as an actor resource
/// inspired by https://ryhl.io/blog/actors-with-tokio/
enum ChannelCommand {
    AddMessage {
        user: UserId,
        content: String,
        reply_to: oneshot::Sender<Result<Message>>,
    },
    GetChannelUsers(
        oneshot::Sender<Result<Vec<UserId>>>, // TODO good case for small vec usage
    ),
}

struct ChannelActor {
    receiver: mpsc::Receiver<ChannelCommand>,
    // Internal state
    channel_id: ID,
    channel: Option<Channel>,
}

impl ChannelActor {
    fn new(channel_id: ID, receiver: mpsc::Receiver<ChannelCommand>) -> Self {
        ChannelActor {
            receiver,
            channel_id,
            channel: None,
        }
    }

    async fn on_start(&mut self) -> Result {
        let c = Channel::load_or_create(self.channel_id).await?;
        self.channel.replace(c);
        Ok(())
    }

    async fn handle_message(&mut self, msg: ChannelCommand) {
        match msg {
            ChannelCommand::AddMessage {
                user,
                content,
                reply_to,
            } => {
                if let Some(c) = self.channel.as_mut() {
                    let res = c.add_message(user, content).await;
                    let _ = reply_to.send(res);
                } else {
                    let _ = reply_to.send(Err(Error::ChannelNotFound));
                }
            }
            ChannelCommand::GetChannelUsers(reply_to) => {
                if let Some(c) = self.channel.as_ref() {
                    // FIXME!!! the `.cloned()` here is realy realy BAD.
                    // We need to send an bunch of UserIds over a channel but the type currently is String
                    // That means that each time we cloning the WHOLE string id in order to send it over the Channel which degrades performance with
                    // uneccessary heap allocations.
                    // One possible solution is to use Rc<UserId> but this means that the Channel implementation should change.
                    // A better solution is to enforce the user id type to be Copy
                    // For now we just ignore these issues as we are struggling to make the code functional :)
                    let users: Vec<_> = c.users.iter().cloned().collect();
                    let _ = reply_to.send(Ok(users));
                } else {
                    let _ = reply_to.send(Err(Error::ChannelNotFound));
                }
            }
        }
    }
}

async fn run(mut actor: ChannelActor) {
    if let Err(err) = actor.on_start().await {
        eprintln!(
            "Channel actor {} initialization error {}",
            &actor.channel_id, &err
        );
        return; // Note here the actor terminates
    }
    while let Some(msg) = actor.receiver.recv().await {
        actor.handle_message(msg).await;
    }
}

/// Handle of the [`ChannelActor`]
/// Provides the public interface of the actor
#[derive(Clone)]
pub struct ChannelHandle {
    sender: mpsc::Sender<ChannelCommand>,
}

impl ChannelHandle {
    pub fn new(channel_id: ID) -> Self {
        let (sender, receiver) = mpsc::channel(MAX_MAILBOX_SIZE);
        let server = ChannelActor::new(channel_id, receiver);
        tokio::spawn(run(server));
        Self { sender }
    }

    pub async fn add_message(&self, user: UserId, content: String) -> Result<Message> {
        let (reply_to, rx) = oneshot::channel();
        let msg = ChannelCommand::AddMessage {
            user,
            content,
            reply_to,
        };

        let _ = self.sender.send(msg).await;
        rx.await.map_err(|_| Error::ActorUnexpectedTermination)?
    }

    pub async fn get_channel_users(&self) -> Result<Vec<UserId>> {
        let (reply_to, rx) = oneshot::channel();
        let msg = ChannelCommand::GetChannelUsers(reply_to);

        let _ = self.sender.send(msg).await;
        rx.await.map_err(|_| Error::ActorUnexpectedTermination)?
    }
}
