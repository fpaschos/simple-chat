use std::sync::Arc;

use crate::{
    channel::Message, errors::Result, registry_actor::RegistryHandle, server_actor::ServerHandle,
    UserId, ID,
};
use futures::{FutureExt, StreamExt};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::ws::WebSocket;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ClientMessage {
    // Join { user: ID },
    SendMessage { user: UserId, content: String },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum ServerMessage {
    InvalidCommand,
    ChatMessage(Message),
}

pub async fn handle_connection(
    ws: WebSocket,
    server: Arc<ServerHandle>,
    registry: Arc<RegistryHandle>,
) -> Result {
    let (outgoing, mut ws_incoming) = ws.split();
    let (connection_tx, connection_rx) = mpsc::unbounded_channel();

    let connection_id = server.connect(connection_tx).await?;
    println!("Conn #<{}>: Opened", &connection_id);

    let connection_rx = UnboundedReceiverStream::new(connection_rx);
    tokio::spawn(connection_rx.forward(outgoing).map(move |result| {
        if let Err(e) = result {
            eprintln!("Conn #<{}>: message error {}", connection_id, e);
        }
    }));

    while let Some(msg) = ws_incoming.next().await {
        let msg = match msg {
            Ok(msg) => msg,
            Err(e) => {
                eprintln!("Conn #<{}>: message error {}", &connection_id, e);
                break;
            }
        };

        println!("Conn #<{}>: received RAW msg {:?}", &connection_id, &msg);
        if let Ok(msg) = msg.to_str() {
            if let Ok(client_message) = serde_json::from_str(msg) {
                match client_message {
                    ClientMessage::SendMessage { user, content } => {
                        // TODO handle this error
                        if let Err(err) =
                            handle_send_message(&server, &registry, connection_id, user, content)
                                .await
                        {
                            eprintln!("Conn #<{}>: Send message error {}", &connection_id, &err);
                        }
                    }
                }
            } else {
                eprintln!("Conn #<{}>: Invalid client message", &connection_id);
            }
        } else {
            eprintln!(
                "Conn #<{}>: Unsupported (binary?) formatted RAW msg {:?}",
                &connection_id, &msg
            )
        }
    }

    println!("Conn #<{}>: Closed", &connection_id);
    server.disconnect(connection_id).await?;
    Ok(())
}

async fn handle_send_message(
    server: &ServerHandle,
    registry: &RegistryHandle,
    connection_id: u128,
    user: UserId,
    msg: String,
) -> Result {
    // TODO remove this (currently we handle only one hardcoded channel)
    let channel_id: ID = uuid::Uuid::parse_str("13cdc63e-55e2-403b-9ac6-4aa7c2155bf4").unwrap();
    let channel = registry.get_channel(channel_id).await?;
    let msg = channel.add_message(user.clone(), msg).await?;
    let server_message = ServerMessage::ChatMessage(msg);
    // TODO remove this (currently we subscribe users to the channel implicitly move to REST API)
    server.register_user(connection_id, user).await?;
    server.publish_to_channel(&channel, server_message).await?;
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    // #[test]
    // fn test_join_serialization() {
    //     let id: ID = uuid::Uuid::parse_str("13cdc63e-55e2-403b-9ac6-4aa7c2155bf4").unwrap();
    //     let json = serde_json::to_string(&ClientMessage::Join { user: id }).unwrap();
    //     println!("{}", json);
    //     assert_eq!(
    //         json,
    //         "{\"type\":\"Join\",\"user\":\"13cdc63e-55e2-403b-9ac6-4aa7c2155bf4\"}"
    //     );
    // }

    #[test]
    fn test_send_message_serialization() {
        let id: ID = uuid::Uuid::parse_str("13cdc63e-55e2-403b-9ac6-4aa7c2155bf4").unwrap();
        let json = serde_json::to_string(&ClientMessage::SendMessage {
            user: id.to_string(),
            content: "test message".into(),
        })
        .unwrap();
        assert_eq!(
            json,
            "{\"type\":\"SendMessage\",\"user\":\"13cdc63e-55e2-403b-9ac6-4aa7c2155bf4\",\"content\":\"test message\"}"
        );
    }
}
