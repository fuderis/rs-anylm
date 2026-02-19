use macron::{Display, Error, From};

/// The result alias
pub type Result<T> = macron::Result<T>;
pub type StdResult<T, E> = std::result::Result<T, E>;

// The error
#[derive(Debug, Display, Error, From)]
pub enum Error {
    #[from]
    Io(std::io::Error),

    #[from]
    Json(serde_json::Error),

    #[from]
    Request(reqwest::Error),

    #[display = "Incorrect context - missing a new user request"]
    IncorrectContext,

    #[display = "Encoded base64 string is invalid"]
    InvalidBase64Url,

    #[display = "AI-generation error: {}"]
    ResponseError(String),
}
