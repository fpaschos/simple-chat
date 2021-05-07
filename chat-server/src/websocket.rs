use crate::ID;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ClientMessage {
    Join { user: ID },
    SendMessage { user: ID, msg: String },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum ServerMessage {
    InvalidCommand,
    ChatMessage { user: ID, msg: String },
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_join_serialization() {
        let id: ID = uuid::Uuid::parse_str("13cdc63e-55e2-403b-9ac6-4aa7c2155bf4").unwrap();
        let json = serde_json::to_string(&ClientMessage::Join { user: id }).unwrap();
        println!("{}", json);
        assert_eq!(
            json,
            "{\"type\":\"Join\",\"user\":\"13cdc63e-55e2-403b-9ac6-4aa7c2155bf4\"}"
        );
    }

    #[test]
    fn test_send_message_serialization() {
        let id: ID = uuid::Uuid::parse_str("13cdc63e-55e2-403b-9ac6-4aa7c2155bf4").unwrap();
        let json = serde_json::to_string(&ClientMessage::SendMessage {
            user: id,
            msg: "test message".into(),
        })
        .unwrap();
        assert_eq!(
            json,
            "{\"type\":\"SendMessage\",\"user\":\"13cdc63e-55e2-403b-9ac6-4aa7c2155bf4\",\"msg\":\"test message\"}"
        );
    }
}
