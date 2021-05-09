use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;

use chrono::{serde::ts_milliseconds, DateTime, Utc};

use crate::errors::{Error, Result};
use crate::{new_id, UserId, CHANNEL_DATA_FOLDER, CHANNEL_INFO_FOLDER, ID};

#[derive(Debug, Serialize, Deserialize)]
pub struct Channel {
    pub id: ID,
    pub name: String,
    pub description: String,
    pub users: HashSet<UserId>,
}

impl Channel {
    pub fn new(id: ID, name: String) -> Self {
        Self {
            id,
            name,
            description: String::new(),
            users: HashSet::default(),
        }
    }

    pub fn get_data_path(&self) -> String {
        format!("{}/{:x}", CHANNEL_DATA_FOLDER, self.id.as_u128())
    }

    /// Gets the file path to be used for storing the channel info.
    pub fn get_info_path(&self) -> String {
        format!("{}{:x}", CHANNEL_INFO_FOLDER, self.id.as_u128())
    }

    /// Loads an existings channel (info) from disk by `channel_id`
    pub async fn load(channel_id: ID) -> Result<Self> {
        let filename = format!("{}/{:x}", CHANNEL_INFO_FOLDER, channel_id.as_u128());
        let path = std::path::Path::new(&filename);
        if !path.exists() {
            return Err(Error::ChannelNotFound);
        }
        let mut file = tokio::fs::OpenOptions::new().read(true).open(path).await?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf).await?;
        Ok(bincode::deserialize(&buf)?)
    }

    /// Saves a channel (info) to disk
    pub async fn save(&self) -> Result {
        tokio::fs::create_dir_all(CHANNEL_INFO_FOLDER).await?;
        let mut file = tokio::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open(self.get_info_path())
            .await?;
        let bytes = bincode::serialize(self)?;
        let mut buf: &[u8] = bytes.as_slice();
        file.write_buf(&mut buf).await?;
        file.flush().await?;
        Ok(())
    }

    /// Attempts to load an existings channel by id.
    /// On [`Error::ChannelNotFound`] failure creates and saves a new channel.
    pub async fn load_or_create(channel_id: ID) -> Result<Self> {
        let channel = match Channel::load(channel_id).await {
            Ok(c) => c,
            Err(Error::ChannelNotFound) => {
                let c = Channel::new(channel_id, format!("Channel #{:x}", channel_id.as_u128()));
                c.save().await?;
                c
            }
            Err(err) => Err(err)?,
        };

        Ok(channel)
    }

    /// Attempts to add a new message to the channel disk file.
    /// The disk file is treated as append only immutable log.
    pub async fn add_message(&mut self, user: UserId, content: String) -> Result<Message> {
        let m = Message::new(self.id, user.clone(), content);
        // TODO remove this for now we append a new user on every channel message
        self.users.insert(user);

        // TODO implement file access
        Ok(m)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: ID,
    pub channel_id: ID,
    pub sender: UserId,
    #[serde(with = "ts_milliseconds")]
    pub created: DateTime<Utc>,
    pub content: String,
}

impl Message {
    pub fn new(channel_id: ID, sender: UserId, content: String) -> Self {
        Self {
            id: new_id(),
            channel_id,
            sender,
            created: Utc::now(),
            content,
        }
    }
}
