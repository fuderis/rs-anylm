use crate::prelude::*;

pub const LMSTUDIO_HOST: &str = "http://localhost:1234";
pub const OPENAI_HOST: &str = "https://api.openai.com";
pub const CEREBRAS_HOST: &str = "https://api.cerebras.ai";
pub const OPENROUTER_HOST: &str = "https://openrouter.ai/api";
pub const PERPLEXITY_HOST: &str = "https://api.perplexity.ai";
pub const ANTHROPIC_HOST: &str = "https://api.anthropic.com";
pub const VOYAGE_HOST: &str = "https://api.voyageai.com";
pub const GOOGLE_HOST: &str = "https://generativelanguage.googleapis.com";

/// The AI API type
#[derive(Clone, Debug, Display, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum ApiKind {
    /// ------- API STANDARTS: ---------
    /// OpenAI API (/v1/chat/completions, /v1/embeddings)
    OpenAI,
    /// Anthropic API (/v1/messages, /v0/embeddings)
    Anthropic,
    /// Google API
    Google,

    // ------- SPECIFIED SERVICES: ---------
    /// Local LM models (OpenAI compatible)
    LmStudio,
    /// Open AI models (OpenAI compatible)
    ChatGpt,
    /// Google models (Google compatible)
    Gemini,
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
    /// Returns true if it's OpenAI API standart
    pub fn is_openai(&self) -> bool {
        !self.is_anthropic() && !self.is_google()
    }

    /// Returns true if it's Anthropic API standart
    pub fn is_anthropic(&self) -> bool {
        matches!(self, Self::Anthropic | Self::Claude)
    }

    /// Returns true if it's Google API standart
    pub fn is_google(&self) -> bool {
        matches!(self, Self::Google | Self::Gemini)
    }

    /// Returns true if it's LM Studio API
    pub fn is_lmstudio(&self) -> bool {
        matches!(self, Self::LmStudio)
    }

    /// Returns LM API host
    pub fn host(&self) -> &'static str {
        match *self {
            Self::OpenAI | Self::ChatGpt => OPENAI_HOST,
            Self::Anthropic | Self::Claude => ANTHROPIC_HOST,
            Self::Google | Self::Gemini => GOOGLE_HOST,
            Self::LmStudio => LMSTUDIO_HOST,
            Self::Cerebras => CEREBRAS_HOST,
            Self::OpenRouter => OPENROUTER_HOST,
            Self::Perplexity => PERPLEXITY_HOST,
            Self::Voyage => VOYAGE_HOST,
        }
    }

    /// Returns completions path (for Google, we need model name)
    pub fn completions_path(&self, model: &str) -> String {
        if self.is_google() {
            str!("v1beta/models/{}:streamGenerateContent?alt=sse", model)
        } else if self.is_anthropic() {
            str!("v1/messages")
        } else {
            str!("v1/chat/completions")
        }
    }

    /// Returns embeddings path
    pub fn embeddings_path(&self, model: &str) -> String {
        if self.is_google() {
            str!("v1beta/models/{}:embedContent", model)
        } else {
            str!("v1/embeddings")
        }
    }
}

impl Default for ApiKind {
    fn default() -> Self {
        Self::OpenAI
    }
}
