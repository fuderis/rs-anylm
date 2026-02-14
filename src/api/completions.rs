use super::*;
use crate::prelude::*;
use futures::StreamExt;
use reqwest::{Client, Proxy, header};
use std::net::SocketAddr;
use std::time::Duration;
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
    /// The API version
    #[serde(skip)]
    pub api_version: Option<String>,
    /// The API standart
    #[serde(skip)]
    pub api_kind: ApiKind,
    /// The API authorization key
    #[serde(skip)]
    pub api_key: String,
    /// The custom server host
    #[serde(skip)]
    pub server: Option<(SocketAddr, bool)>,
    /// The proxy tunnel settings
    #[serde(skip)]
    pub proxy: Option<Proxy>,
    /// The connection timeout
    #[serde(skip)]
    pub timeout: Duration,
    /// The AI model name
    pub model: String,
    /// The request messages
    pub messages: Vec<Message>,
    /// The maximum tokens count
    pub max_tokens: i32,
    /// The AI generation temperature
    pub temperature: f32,
    /// The summary tokens count
    pub tokens_count: usize,
}

impl Completions {
    /// Creates a new LM chat completions request
    pub fn new(kind: ApiKind, key: impl Into<String>, model: impl Into<String>) -> Self {
        Self {
            api_version: if kind.is_anthropic_standart() {
                Some(fmt!("2023-06-01"))
            } else {
                None
            },
            api_kind: kind,
            api_key: key.into(),
            server: None,
            proxy: None,
            timeout: Duration::from_secs(30),
            model: model.into(),
            messages: Vec::new(),
            max_tokens: -1,
            temperature: 0.6,
            tokens_count: 0,
        }
    }

    /// Sets the API version
    pub fn version(mut self, version: impl Into<String>) -> Self {
        self.api_version = Some(version.into());
        self
    }
    /// Sets the API version
    pub fn set_version(&mut self, version: impl Into<String>) {
        self.api_version = Some(version.into());
    }

    /// Creates a new OpenAI (ChatGPT) request
    pub fn openai(key: impl Into<String>, model: impl Into<String>) -> Self {
        Self::new(ApiKind::OpenAI, key, model)
    }

    /// Creates a new Anthropic (Claude) request
    pub fn anthropic(
        key: impl Into<String>,
        model: impl Into<String>,
        version: Option<String>,
    ) -> Self {
        let this = Self::new(ApiKind::Anthropic, key, model);
        if let Some(v) = version {
            this.version(v)
        } else {
            this
        }
    }

    /// Creates a new LM Studio request
    pub fn lmstudio(port: u16, model: impl Into<String>) -> Self {
        Self::new(ApiKind::LmStudio, String::new(), model).server(([127, 0, 0, 1], port), false)
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

    /// Sets a proxy tunnel settings
    pub fn proxy(mut self, proxy: Proxy) -> Self {
        self.proxy = Some(proxy);
        self
    }
    /// Sets a proxy tunnel settings
    pub fn set_proxy(&mut self, proxy: Proxy) {
        self.proxy = Some(proxy);
    }

    /// Sets a connection timeout
    pub fn timeout(mut self, secs: u64) -> Self {
        self.timeout = Duration::from_secs(secs);
        self
    }
    /// Sets a connection timeout
    pub fn set_timeout(&mut self, secs: u64) {
        self.timeout = Duration::from_secs(secs);
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

    /// Adds a message to request
    pub fn message(mut self, role: Role, content: Vec<Content>) -> Self {
        let msg = Message::new(role, content);
        self.tokens_count += msg.tokens_count;
        self.messages.push(msg);
        self
    }
    /// Adds a system message to request
    pub fn system_message(self, content: Vec<Content>) -> Self {
        self.message(Role::System, content)
    }
    /// Adds a user message to request
    pub fn user_message(self, content: Vec<Content>) -> Self {
        self.message(Role::User, content)
    }
    /// Adds a assistant message to request
    pub fn assistant_message(self, content: Vec<Content>) -> Self {
        self.message(Role::Assistant, content)
    }

    /// Adds a message to request
    pub fn add_message(&mut self, role: Role, content: Vec<Content>) {
        let msg = Message::new(role, content);
        self.tokens_count += msg.tokens_count;
        self.messages.push(msg);
    }
    /// Adds a system message to request
    pub fn add_system_message(&mut self, content: Vec<Content>) {
        self.add_message(Role::System, content)
    }
    /// Adds a user message to request
    pub fn add_user_message(&mut self, content: Vec<Content>) {
        self.add_message(Role::User, content)
    }
    /// Adds a assistant message to request
    pub fn add_assistant_message(&mut self, content: Vec<Content>) {
        self.add_message(Role::Assistant, content)
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
    pub fn max_tokens(mut self, count: i32) -> Self {
        self.max_tokens = count;
        self
    }
    /// Sets the max context tokens count
    pub fn set_max_tokens(&mut self, count: i32) {
        self.max_tokens = count;
    }

    /// Sends the request to LM server
    pub async fn send(&mut self) -> Result<Stream> {
        let is_openai_standart = self.api_kind.is_openai_standart();
        let is_anthropic_standart = !is_openai_standart;

        // generate URL:
        let url = if let Some((host, https)) = self.server {
            self.api_kind.custom_completions_url(host, https)
        } else {
            self.api_kind.completions_url()
        };

        // add tokens limit:
        if self.max_tokens <= 0 && is_anthropic_standart {
            self.max_tokens = 8096;
        }

        // remove extra context:
        if self.max_tokens > 0 {
            let mut idx = 0;
            let mut max_idx = self.messages.len() - 1;
            let max_count = self.max_tokens as usize;

            // removing non-system old messages to free context size:
            while self.tokens_count > max_count && idx < max_idx {
                let msg = &self.messages[idx];
                if msg.role.is_system() {
                    idx += 1;
                    continue;
                }

                self.tokens_count -= msg.tokens_count;
                self.messages.remove(idx);
                max_idx -= 1;
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
        let data = data.as_object_mut().unwrap();
        data.remove("tokens_count");
        data.insert(str!("stream"), JsonValue::Bool(true));

        if let Some(messages) = data.get_mut("messages").and_then(|v| v.as_array_mut()) {
            for msg in messages {
                if let Some(msg_obj) = msg.as_object_mut() {
                    msg_obj.remove("tokens_count");
                }
            }
        }

        // create client:
        let mut client = Client::builder().timeout(self.timeout);

        // set proxy:
        if let Some(proxy) = self.proxy.take() {
            client = client.proxy(proxy);
            client = client.danger_accept_invalid_certs(true); // VPN SSL
        }

        // send request:
        let mut request = client
            .build()?
            .post(&url)
            .header(header::CONTENT_TYPE, "application/json")
            .header(header::ACCEPT, "text/event-stream")
            .json(&data);

        // OpenAI, LM Studio, OpenRouter, etc
        if is_openai_standart {
            request = request.header(header::AUTHORIZATION, fmt!("Bearer {}", self.api_key));
        }
        // Anthropic, Claude
        else {
            request = request.header("x-api-key", &self.api_key);
            request = request.header(
                "anthropic-version",
                self.api_version.take().unwrap_or(str!("2023-06-01")),
            );
        }

        //dbg!(&request);

        // send request:
        let response = request.send().await?;
        let (tx, rx) = mpsc::unbounded_channel::<Result<Chunk>>();

        // spawn stream reader:
        tokio::spawn(async move {
            let mut buffer = String::new();
            let mut stream = response.bytes_stream();

            // read chunks:
            while let Some(chunk) = stream.next().await {
                match chunk {
                    Ok(bytes) => {
                        buffer.push_str(&String::from_utf8_lossy(&bytes));
                        // dbg!(&buffer);

                        while let Some(pos) = buffer.find("\n\n") {
                            let event = buffer.drain(..=pos).collect::<String>();
                            if let Some(data_line) =
                                event.lines().find(|line| line.starts_with("data: "))
                            {
                                let json_data = &data_line[6..]; // cut "data: "
                                if json_data == "[DONE]" {
                                    break;
                                }
                                let mut text = String::new();

                                // OpenAI API standart:
                                if is_openai_standart {
                                    #[derive(Deserialize)]
                                    struct OpenAIChunk {
                                        pub choices: Vec<OpenAIChoice>,
                                    }
                                    #[derive(Deserialize)]
                                    struct OpenAIChoice {
                                        pub delta: OpenAIDelta,
                                        // pub finish_reason: Option<String>,
                                    }
                                    #[derive(Deserialize)]
                                    struct OpenAIDelta {
                                        #[serde(default)]
                                        pub content: Option<String>,
                                    }

                                    // parse chunk:
                                    if let Ok(OpenAIChunk { choices }) =
                                        json::from_str::<OpenAIChunk>(json_data)
                                            .map_err(Error::from)
                                    {
                                        for choice in choices {
                                            if let Some(content) = choice.delta.content {
                                                text.push_str(&content);
                                            }
                                        }
                                    }
                                }
                                // Anthropic API standart:
                                else {
                                    #[derive(Deserialize)]
                                    struct AnthropicChunk {
                                        r#type: String,
                                        delta: AnthropicDelta,
                                    }
                                    #[derive(Deserialize)]
                                    struct AnthropicDelta {
                                        // r#type: String,
                                        text: Option<String>,
                                    }

                                    // parse chunk:
                                    if let Ok(AnthropicChunk { r#type, delta }) =
                                        json::from_str::<AnthropicChunk>(json_data)
                                        && r#type == "content_block_delta"
                                    {
                                        if let Some(content) = delta.text {
                                            text.push_str(&content);
                                        }
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
