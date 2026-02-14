use crate::prelude::*;
use std::net::SocketAddr;

/// The AI API type
#[derive(Clone, Debug, Display, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum ApiKind {
    /// ------- API STANDARTS: ---------
    /// OpenAI API (/v1/chat/completions, /v1/embeddings)
    OpenAI,
    /// Anthropic API (/v1/messages, /v0/embeddings)
    Anthropic,

    // ------- SPECIFIED SERVICES: ---------
    /// Local LM models (OpenAI compatible)
    LmStudio,
    /// Anthropic models (Antropic compatible)
    Claude,
    /// Cerebras models (OpenAI compatible)
    Cerebras,
    /// OpenRouter models (OpenAI compatible)
    OpenRouter,
    /// Open AI models (OpenAI compatible)
    ChatGpt,
    /// Perplexity models (OpenAI compatible)
    Perplexity,
}

impl ApiKind {
    /// Returns LM API host
    pub const fn host(&self) -> &'static str {
        use ApiKind::*;

        match *self {
            OpenAI | ChatGpt => "https://api.openai.com",
            Anthropic | Claude => "https://api.anthropic.com",
            LmStudio => "http://localhost:1234",
            Cerebras => "https://api.cerebras.ai",
            OpenRouter => "https://openrouter.ai/api",
            Perplexity => "https://api.perplexity.ai",
        }
    }

    /// Returns completions path
    pub const fn completions(&self) -> &'static str {
        use ApiKind::*;

        match *self {
            Anthropic | Claude => "v1/messages",
            _ => "v1/chat/completions",
        }
    }
    /// Returns embeddings path
    pub const fn embeddings(&self) -> &'static str {
        "v1/embeddings"
    }

    /// Returns completions URL
    pub fn completions_url(&self) -> String {
        fmt!("{}/{}", self.host(), self.completions())
    }
    /// Returns embeddings URL
    pub fn embeddings_url(&self) -> String {
        fmt!("{}/{}", self.host(), self.embeddings())
    }

    /// Returns completions URL
    pub fn custom_completions_url(&self, addr: impl Into<SocketAddr>, https: bool) -> String {
        fmt!(
            "http{}{}/{}",
            if https { "s" } else { "" },
            addr.into(),
            self.completions()
        )
    }
    /// Returns embeddings URL
    pub fn custom_embeddings_url(&self, addr: impl Into<SocketAddr>, https: bool) -> String {
        fmt!(
            "http{}{}/{}",
            if https { "s" } else { "" },
            addr.into(),
            self.embeddings()
        )
    }
}

impl Default for ApiKind {
    fn default() -> Self {
        Self::OpenAI
    }
}
