pub mod channel;
pub mod channel_actor;
pub mod errors;
pub mod registry_actor;
pub mod server_actor;
pub mod websocket;

const MAX_MAILBOX_SIZE: usize = 1024;

/// Relative path of the folder in which Channel information files (`${ID}`) files are stored.
pub const CHANNEL_INFO_FOLDER: &str = "data/channels/info/";
/// Relative path of the folder in which Channel data files are stored (channel directories and messages).
pub const CHANNEL_DATA_FOLDER: &str = "data/channels/data/";

type UserId = String;

type ID = uuid::Uuid;

pub fn new_id() -> uuid::Uuid {
    uuid::Uuid::new_v4()
}
