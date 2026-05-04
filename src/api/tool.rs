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
    pub fn new(name: impl Into<String>, descr: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: match descr.into() {
                s if !s.is_empty() => Some(s),
                _ => None,
            },
            parameters: Schema::object(""),
        }
    }

    /// Adds an argument
    pub fn property(mut self, name: impl Into<String>, schema: Schema, optional: bool) -> Self {
        self.parameters = self.parameters.property(name, schema, optional);
        self
    }

    /// Adds a required argument
    pub fn required_property(mut self, name: impl Into<String>, schema: Schema) -> Self {
        self.parameters = self.parameters.required_property(name, schema);
        self
    }

    /// Adds an optional argument
    pub fn optional_property(mut self, name: impl Into<String>, schema: Schema) -> Self {
        self.parameters = self.parameters.optional_property(name, schema);
        self
    }
}

impl Tool {
    /// Converts into `OpenAI` format: {"type": "function", "function": {...}}
    pub fn to_openai_format(&self) -> Result<JsonValue> {
        let tool_json = self.to_json_tool()?;
        Ok(json!({
            "type": "function",
            "function": tool_json
        }))
    }

    /// Converts into `Anthropic` format: replace "parameters" to "input_schema"
    pub fn to_anthropic_format(&self) -> Result<JsonValue> {
        let mut tool_json = self.to_json_tool()?;

        if let Some(obj) = tool_json.as_object_mut() {
            if let Some(params) = obj.remove("parameters") {
                obj.insert("input_schema".to_string(), params);
            }
        }

        Ok(tool_json)
    }

    /// Converts into `Google` format: {"function_declarations": [...]}
    pub fn to_google_format(&self) -> Result<JsonValue> {
        let tool_json = self.to_json_tool()?;
        Ok(json!({
            "function_declarations": [ tool_json ]
        }))
    }
}

impl Tool {
    /// Converts into valid JSON-format
    pub fn to_json_tool(&self) -> Result<JsonValue> {
        let mut v = serde_json::to_value(self)?;

        if let Some(params) = v.get_mut("parameters") {
            Schema::sanitize_json_schema(params);
        }

        Ok(v)
    }
}
