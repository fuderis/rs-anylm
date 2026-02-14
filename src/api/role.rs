use crate::prelude::*;

/// The message role
#[derive(Clone, Debug, Display, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    User,
    Assistant,
}

impl Role {
    /// Returns true if it's the system prompt message
    pub fn is_system(&self) -> bool {
        Self::System == *self
    }

    /// Returns true if it's the user message
    pub fn is_user(&self) -> bool {
        Self::User == *self
    }

    /// Returns true if it's the assistant message
    pub fn is_assistant(&self) -> bool {
        Self::Assistant == *self
    }
}
