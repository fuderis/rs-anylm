use crate::prelude::*;
use std::net::SocketAddr;

pub const LMSTUDIO_HOST: &str = "http://localhost:1234";
pub const OPENAI_HOST: &str = "https://api.openai.com";
pub const CEREBRAS_HOST: &str = "https://api.cerebras.ai";
pub const OPENROUTER_HOST: &str = "https://openrouter.ai/api";
pub const PERPLEXITY_HOST: &str = "https://api.perplexity.ai";
pub const ANTHROPIC_HOST: &str = "https://api.anthropic.com";
pub const VOYAGE_HOST: &str = "https://api.voyageai.com";

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
    /// Open AI models (OpenAI compatible)
    ChatGpt,
    /// Cerebras models (OpenAI compatible)
    Cerebras,
    /// OpenRouter models (OpenAI compatible)
    OpenRouter,
    /// Perplexity models (OpenAI compatible)
    Perplexity,
    /// Anthropic models (Antropic compatible)
    Claude,
    /// Embeddings models (instead Anthropic embeddings)
    Voyage,
}

impl ApiKind {
    /// Returns true is it's OpenAI API standart
    pub fn is_openai_standart(&self) -> bool {
        use ApiKind::*;
        *self != Anthropic && *self != Claude
    }

    /// Returns true is it's Anthropic API standart
    pub fn is_anthropic_standart(&self) -> bool {
        use ApiKind::*;
        *self == Anthropic || *self == Claude
    }

    /// Returns LM API host
    pub fn host(&self) -> &'static str {
        use ApiKind::*;

        match *self {
            OpenAI | ChatGpt => OPENAI_HOST,
            Anthropic | Claude => ANTHROPIC_HOST,
            LmStudio => LMSTUDIO_HOST,
            Cerebras => CEREBRAS_HOST,
            OpenRouter => OPENROUTER_HOST,
            Perplexity => PERPLEXITY_HOST,
            Voyage => VOYAGE_HOST,
        }
    }

    /// Returns completions path
    pub fn completions(&self) -> &'static str {
        if self.is_anthropic_standart() {
            "v1/messages"
        } else {
            "v1/chat/completions"
        }
    }
    /// Returns embeddings path
    pub fn embeddings(&self) -> &'static str {
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
            "http{}://{}/{}",
            if https { "s" } else { "" },
            addr.into(),
            self.completions()
        )
    }
    /// Returns embeddings URL
    pub fn custom_embeddings_url(&self, addr: impl Into<SocketAddr>, https: bool) -> String {
        fmt!(
            "http{}://{}/{}",
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
