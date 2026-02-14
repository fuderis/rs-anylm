use macron::{Display, Error, From};

/// Std Result alias
pub type StdResult<T, E> = std::result::Result<T, E>;
/// Result alias
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;

// The error
#[derive(Debug, Display, Error, From)]
pub enum Error {
    #[from]
    Io(std::io::Error),

    #[from]
    ParseJson(serde_json::Error),

    #[display = "Incorrect context - missing a new user request"]
    IncorrectContext,
}
