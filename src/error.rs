use crate::chunk::ResponseError;
use macron::{Display, Error, From};

/// The error
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
    ResponseError(ResponseError),
}
