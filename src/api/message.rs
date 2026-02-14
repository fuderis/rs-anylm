use super::Role;
use crate::prelude::*;

/// The request message
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct Message {
    pub role: Role,
    pub content: String,
}

impl Message {
    /// The system prompt message
    pub fn system(msg: impl Into<String>) -> Self {
        Self {
            role: Role::System,
            content: msg.into(),
        }
    }

    /// The user prompt message
    pub fn user(msg: impl Into<String>) -> Self {
        Self {
            role: Role::User,
            content: msg.into(),
        }
    }

    /// The assistant prompt message
    pub fn assistant(msg: impl Into<String>) -> Self {
        Self {
            role: Role::Assistant,
            content: msg.into(),
        }
    }
}

impl From<String> for Message {
    fn from(s: String) -> Self {
        Self::user(s)
    }
}

impl From<&str> for Message {
    fn from(s: &str) -> Self {
        Self::user(s)
    }
}

impl From<Bytes> for Message {
    fn from(s: Bytes) -> Self {
        Self::user(String::from_utf8_lossy(&s))
    }
}
