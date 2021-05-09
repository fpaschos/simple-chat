use thiserror::Error;

/// General result type for chat server defaults to [`Error`].
pub type Result<T = (), E = Error> = std::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Channel does not exist")]
    ChannelNotFound,
    #[error("Actor unexpected termination")]
    ActorUnexpectedTermination,
    #[error("IO error")]
    Io(#[from] std::io::Error),
    #[error("Bincode error")]
    Bincode(#[from] bincode::Error),
    #[error("JSON error")]
    Json(#[from] serde_json::Error),
    #[error("{0}")]
    Generic(String),
}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Self::Generic(s)
    }
}
