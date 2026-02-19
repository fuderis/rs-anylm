use super::Schema;
use crate::prelude::*;

/// The tool call structure
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Tool {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    parameters: Schema,
}

impl Tool {
    /// Creates a new tool call
    pub fn new(name: impl Into<String>, descr: impl Into<String>, schema: Schema) -> Self {
        Self {
            name: name.into(),
            description: match descr.into() {
                s if !s.is_empty() => Some(s),
                _ => None,
            },
            parameters: schema,
        }
    }
}
