use super::ApiKind;
use crate::prelude::*;
use reqwest::{Client, Proxy, header};
use std::net::SocketAddr;
use std::time::Duration;

/// The embeddings usage info
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Usage {
    pub total_tokens: usize,
}

/// The embeddings response
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Embedded {
    pub object: String,
    pub data: Vec<Embedding>,
    pub model: String,
    pub usage: Usage,
}

/// The embedding chunk
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Embedding {
    pub object: String,
    pub index: usize,
    pub embedding: Vec<f32>,
}

/// The LM API embeddings request
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Embeddings {
    #[serde(skip)]
    pub api_kind: ApiKind,
    #[serde(skip)]
    pub api_key: String,
    #[serde(skip)]
    pub server: Option<(SocketAddr, bool)>,
    #[serde(skip)]
    proxy: Option<Proxy>,
    #[serde(skip)]
    timeout: Duration,
    pub model: String,
    pub input: Vec<String>,
}

impl Embeddings {
    /// Creates a new LM embeddings request
    pub fn new(kind: ApiKind, key: impl Into<String>, model: impl Into<String>) -> Self {
        Self {
            api_kind: kind,
            api_key: key.into(),
            server: None,
            proxy: None,
            timeout: Duration::from_secs(30),
            model: model.into(),
            input: Vec::new(),
        }
    }

    /// Creates a new OpenAI (ChatGPT) embeddings request
    pub fn openai(key: impl Into<String>, model: impl Into<String>) -> Self {
        Self::new(ApiKind::OpenAI, key, model)
    }

    /// Creates a new Anthropic (Voyage) embeddings request
    pub fn anthropic(key: impl Into<String>, model: impl Into<String>) -> Self {
        Self::new(ApiKind::Voyage, key, model)
    }

    /// Creates a new LM Studio embeddings request
    pub fn lmstudio(port: u16, model: impl Into<String>) -> Self {
        Self::new(ApiKind::LmStudio, String::new(), model).server(([127, 0, 0, 1], port), false)
    }

    /// Creates a new ChatGPT embeddings request
    pub fn chatgpt(key: impl Into<String>, model: impl Into<String>) -> Self {
        Self::new(ApiKind::ChatGpt, key, model)
    }

    /// Creates a new Cerebras embeddings request
    pub fn cerebras(key: impl Into<String>, model: impl Into<String>) -> Self {
        Self::new(ApiKind::Cerebras, key, model)
    }

    /// Creates a new Claude (Voyage) embeddings request
    pub fn claude(key: impl Into<String>, model: impl Into<String>) -> Self {
        Self::new(ApiKind::Voyage, key, model)
    }

    /// Creates a new OpenRouter embeddings request
    pub fn openrouter(key: impl Into<String>, model: impl Into<String>) -> Self {
        Self::new(ApiKind::OpenRouter, key, model)
    }

    /// Creates a new Perplexity AI embeddings request
    pub fn perplexity(key: impl Into<String>, model: impl Into<String>) -> Self {
        Self::new(ApiKind::Perplexity, key, model)
    }

    /// Creates a new Voyage embeddings request
    pub fn voyage(key: impl Into<String>, model: impl Into<String>) -> Self {
        Self::new(ApiKind::Voyage, key, model)
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

    /// Adds embedding input
    pub fn input(mut self, input: impl Into<String>) -> Self {
        self.input.push(input.into());
        self
    }

    /// Adds embedding input
    pub fn add_input(&mut self, input: impl Into<String>) {
        self.input.push(input.into());
    }

    /// Sends the request to LM server
    pub async fn send(&mut self) -> Result<Embedded> {
        // generate URL:
        let url = if let Some((host, https)) = self.server {
            self.api_kind.custom_embeddings_url(host, https)
        } else {
            self.api_kind.embeddings_url()
        };

        // serialize request data:
        let mut data = json::to_value(&self).map_err(Error::from)?;
        let obj = data.as_object_mut().unwrap();

        // create client:
        let mut client = Client::builder().timeout(self.timeout);

        // set proxy:
        if let Some(proxy) = self.proxy.take() {
            client = client.proxy(proxy);
            client = client.danger_accept_invalid_certs(true); // VPN SSL
        }

        // send request:
        let response = client
            .build()?
            .post(&url)
            .header(header::CONTENT_TYPE, "application/json")
            .header(header::AUTHORIZATION, &self.api_key)
            .json(&obj)
            .send()
            .await?
            .error_for_status()?
            .json::<Embedded>()
            .await
            .map_err(Error::from)?;

        Ok(response)
    }
}
