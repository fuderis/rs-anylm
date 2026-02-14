use super::*;
use crate::{prelude::*, utils};
use futures::StreamExt;
use reqwest::{Client, header};
use std::net::SocketAddr;
use tokio::sync::mpsc;

/// The completions response stream reader
#[derive(Debug)]
pub struct Stream {
    rx: mpsc::UnboundedReceiver<Result<Chunk>>,
}

impl Stream {
    /// Read a next completions response chunk
    pub async fn next(&mut self) -> Option<Result<Chunk>> {
        self.rx.recv().await
    }
}

/// The completions response chunk
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Chunk {
    pub text: String,
}

/// The LM API chat completions request
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Completions {
    /// The API standart
    #[serde(skip)]
    pub api_kind: ApiKind,
    /// The API authorization key
    #[serde(skip)]
    pub api_key: String,
    /// The custom server host
    #[serde(skip)]
    pub server: Option<(SocketAddr, bool)>,
    /// The AI model name
    pub model: String,
    /// The request messages
    pub messages: Vec<Message>,
    /// The maximum tokens count
    pub max_tokens: Option<usize>,
    /// The AI generation temperature
    pub temperature: f32,
    /// The summary tokens count
    pub tokens_count: usize,
}

impl Completions {
    /// Creates a new LM chat completions request
    pub fn new(kind: ApiKind, key: impl Into<String>, model: impl Into<String>) -> Self {
        Self {
            api_kind: kind,
            api_key: key.into(),
            server: None,
            model: model.into(),
            messages: Vec::new(),
            max_tokens: None,
            temperature: 0.6,
            tokens_count: 0,
        }
    }

    /// Creates a new OpenAI (ChatGPT) request
    pub fn openai(key: impl Into<String>, model: impl Into<String>) -> Self {
        Self::new(ApiKind::OpenAI, key, model)
    }

    /// Creates a new Anthropic (Claude) request
    pub fn anthropic(key: impl Into<String>, model: impl Into<String>) -> Self {
        Self::new(ApiKind::Anthropic, key, model)
    }

    /// Creates a new LM Studio request
    pub fn lmstudio(model: impl Into<String>) -> Self {
        Self::new(ApiKind::LmStudio, String::new(), model)
    }

    /// Creates a new ChatGPT request
    pub fn chatgpt(key: impl Into<String>, model: impl Into<String>) -> Self {
        Self::new(ApiKind::ChatGpt, key, model)
    }

    /// Creates a new Cerebras AI request
    pub fn cerebras(key: impl Into<String>, model: impl Into<String>) -> Self {
        Self::new(ApiKind::Cerebras, key, model)
    }

    /// Creates a new Claude AI request
    pub fn claude(key: impl Into<String>, model: impl Into<String>) -> Self {
        Self::new(ApiKind::Claude, key, model)
    }

    /// Creates a new OpenRouter AI request
    pub fn openrouter(key: impl Into<String>, model: impl Into<String>) -> Self {
        Self::new(ApiKind::OpenRouter, key, model)
    }

    /// Creates a new Perplexity AI request
    pub fn perplexity(key: impl Into<String>, model: impl Into<String>) -> Self {
        Self::new(ApiKind::Perplexity, key, model)
    }

    /// Sets the LM API authorization key
    pub fn key(mut self, key: impl Into<String>) -> Self {
        self.api_key = key.into();
        self
    }
    /// Sets the LM API authorization key
    pub fn set_key(&mut self, key: impl Into<String>) {
        self.api_key = key.into();
    }

    /// Sets the custom LM API server host
    pub fn server(mut self, addr: impl Into<SocketAddr>, https: bool) -> Self {
        self.server = Some((addr.into(), https));
        self
    }
    /// Sets the custom LM API server host
    pub fn set_server(&mut self, addr: impl Into<SocketAddr>, https: bool) {
        self.server = Some((addr.into(), https));
    }

    /// Adds a message to request
    pub fn message(mut self, msg: impl Into<Message>) -> Self {
        let msg = msg.into();
        self.tokens_count += utils::count_tokens(&msg.content);
        self.messages.push(msg);
        self
    }
    /// Adds a system message to request
    pub fn system_message(self, msg: impl Into<String>) -> Self {
        self.message(Message::user(msg))
    }
    /// Adds a user message to request
    pub fn user_message(self, msg: impl Into<String>) -> Self {
        self.message(Message::user(msg))
    }
    /// Adds a assistant message to request
    pub fn assistant_message(self, msg: impl Into<String>) -> Self {
        self.message(Message::user(msg))
    }

    /// Adds a message to request
    pub fn add_message(&mut self, msg: impl Into<Message>) {
        let msg = msg.into();
        self.tokens_count += utils::count_tokens(&msg.content);
        self.messages.push(msg);
    }
    /// Adds a system message to request
    pub fn add_system_message(&mut self, msg: impl Into<String>) {
        self.add_message(Message::user(msg))
    }
    /// Adds a user message to request
    pub fn add_user_message(&mut self, msg: impl Into<String>) {
        self.add_message(Message::user(msg))
    }
    /// Adds a assistant message to request
    pub fn add_assistant_message(&mut self, msg: impl Into<String>) {
        self.add_message(Message::user(msg))
    }

    /// Sets the LM model name
    pub fn model(mut self, model: impl Into<String>) -> Self {
        self.model = model.into();
        self
    }
    /// Sets the LM model name
    pub fn set_model(&mut self, model: impl Into<String>) {
        self.model = model.into();
    }

    /// Sets the AI generation temperature
    pub fn temperature(mut self, temperature: f32) -> Self {
        self.temperature = temperature;
        self
    }
    /// Sets the AI generation temperature
    pub fn set_temperature(&mut self, temperature: f32) {
        self.temperature = temperature;
    }

    /// Sets the max context tokens count
    pub fn max_tokens(mut self, count: usize) -> Self {
        self.max_tokens = Some(count);
        self
    }
    /// Sets the max context tokens count
    pub fn set_max_tokens(&mut self, count: usize) {
        self.max_tokens = Some(count);
    }

    /// Sends the request to LM server
    pub async fn send(&mut self) -> Result<Stream> {
        // generate URL:
        let url = if let Some((host, https)) = self.server {
            self.api_kind.custom_completions_url(host, https)
        } else {
            self.api_kind.completions_url()
        };

        // remove extra context:
        if let Some(max_count) = self.max_tokens {
            let mut idx = 0;
            let last_idx = self.messages.len() - 1;

            // removing non-system old messages to free context size:
            while self.tokens_count > max_count && idx < last_idx {
                let msg = &self.messages[idx];
                if !msg.role.is_system() {
                    self.tokens_count -= utils::count_tokens(&msg.content);
                }
                idx += 1;
            }

            // check last message role for user query:
            if let Some(msg) = self.messages.last()
                && msg.role.is_assistant()
            {
                return Err(Error::IncorrectContext.into());
            }
        }

        // serialize request data:
        let mut data = json::to_value(&self).map_err(Error::from)?;
        let obj = data.as_object_mut().unwrap();
        obj.remove("tokens_count");
        obj.insert(str!("stream"), JsonValue::Bool(true));

        // send request:
        let client = Client::new();
        let response = client
            .post(&url)
            .header(header::AUTHORIZATION, &self.api_key)
            .json(&obj)
            .send()
            .await?;

        let (tx, rx) = mpsc::unbounded_channel::<Result<Chunk>>();

        // spawn stream reader:
        tokio::spawn(async move {
            let mut stream = response.bytes_stream();

            #[derive(Deserialize)]
            struct StreamChunk {
                pub choices: Vec<ChunkChoice>,
            }

            #[derive(Deserialize)]
            struct ChunkChoice {
                pub delta: Delta,
                // pub finish_reason: Option<String>,
            }

            #[derive(Deserialize)]
            struct Delta {
                #[serde(default)]
                pub content: Option<String>,
            }

            // read chunks:
            while let Some(item) = stream.next().await {
                match item {
                    Ok(bytes) => {
                        // parsing lines:
                        for line in String::from_utf8_lossy(&bytes).lines().into_iter() {
                            if line.starts_with("data: ") {
                                let data = &line[6..];
                                if data == "[DONE]" {
                                    return;
                                }

                                // parse chunk:
                                let chunk: StreamChunk =
                                    match json::from_str(data).map_err(Error::from) {
                                        Ok(r) => r,
                                        Err(_) => return,
                                    };

                                // collect choices:
                                let mut text = str!();
                                for choice in chunk.choices {
                                    if let Some(content) = choice.delta.content {
                                        text.push_str(&content);
                                    }
                                }

                                // send chunk to receiver:
                                tx.send(Ok(Chunk { text })).ok();
                            }
                        }
                    }
                    Err(e) => {
                        tx.send(Err(e.into())).ok();
                        break;
                    }
                }
            }
        });

        Ok(Stream { rx })
    }
}
