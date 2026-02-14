[![github]](https://github.com/fuderis/rs-anylm)&ensp;
[![crates-io]](https://crates.io/crates/anylm)&ensp;
[![docs-rs]](https://docs.rs/anylm)

[github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
[crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
[docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs

# AnyLM - Universal API for Every AI

**Sick of juggling separate APIs for each AI model—wrestling with their quirky syntax and endless docs?**<br>

I was too. That's why I built `AnyLM`: learn one intuitive API once, then unleash it across any service—LLMs, embeddings, vision models, you name it. Seamless, powerful, done.

## Supported:

* **Standarts**: Supported `OpenAI` and `Anthropic` API standarts (what 90% of AI uses).
* **Services**: `LM Studio`, `ChatGPT`, `Cerebras`, `OpenRouter`, `Perplexity`, `Claude` and `Voyage`.
* **Stream-Response**: Allows you to read the LM response in parts without waiting for the full completion.
* **Context Control**: Automatic trimming of the dialog context when exceeding the token limits.
* **Embeddings**: Text embeddings support for fast text analysis.
* **Image-View**: Image analysis support with reading from files and directly via `base64 url`.
* **Proxy Support**: Support for using proxy/vpn request tunneling.
* **Is something missing?**: Write to me and I will add it too. (`Telegram`: [@fuderis](https://t.me/fuderis)).

## Examples:

### Cerebras:
```rust
use anylm::{Chunk, Completions, prelude::*};

#[tokio::main]
async fn main() -> Result<()> {
    let api_key = std::env::var("CEREBRAS_API_KEY")?;

    // send request:
    let mut response = Completions::cerebras(api_key, "llama3.1-8b")
        //.proxy(reqwest::Proxy::all("socks5://127.0.0.1:1080")?)
        .user_message(vec!["Hello, how are you doing?".into()])
        .send()
        .await?;

    // read response stream:
    while let Some(chunk) = response.next().await {
        match chunk {
            Ok(Chunk { text }) => eprint!("{text}"),
            Err(e) => eprintln!("\n{e}"),
        };
    }
    println!();

    Ok(())
}
```

### Claude:
```rust
use anylm::{Chunk, Completions, prelude::*};

#[tokio::main]
async fn main() -> Result<()> {
    let api_key = std::env::var("ANTHROPIC_API_KEY")?;

    // send request:
    let mut response = Completions::claude(api_key, "claude-opus-4-6", None)
        //.proxy(reqwest::Proxy::all("socks5://127.0.0.1:1080")?)
        .user_message(vec!["Hello, how are you doing?".into()])
        .send()
        .await?;

    // read response stream:
    while let Some(chunk) = response.next().await {
        match chunk {
            Ok(Chunk { text }) => eprint!("{text}"),
            Err(e) => eprintln!("\n{e}"),
        };
    }
    println!();

    Ok(())
}
```

### ImageView (LM Studio as example):
```rust
use anylm::{Chunk, Completions, prelude::*};

#[tokio::main]
async fn main() -> Result<()> {
    // send request:
    let mut response = Completions::lmstudio(1234, "qwen/qwen3-vl-4b")
        .user_message(vec![
            Path::new("test-image.png").into(),
            "What's on the picture?".into(),
        ])
        .send()
        .await?;

    // read response stream:
    while let Some(chunk) = response.next().await {
        match chunk {
            Ok(Chunk { text }) => eprint!("{text}"),
            Err(e) => eprintln!("\n{e}"),
        };
    }
    println!();

    Ok(())
}
```

### Embeddings (LM Studio as example):
```rust
use anylm::{Embeddings, prelude::*};

#[tokio::main]
async fn main() -> Result<()> {
    // send request:
    let response = Embeddings::lmstudio(1234, "nomic-ai/nomic-embed-text-v1.5")
        .input("Hello, how are you doing?")
        .send()
        .await?;

    // print response:
    dbg!(response);

    Ok(())
}
```

> And etc., it all has the same logic..

## License & Credits:

* **License**: Distributed under the [*MIT*](https://github.com/fuderis/rs-anylm/blob/main/LICENSE.md) license.
* **Contacts**:
  [*GitHub*](https://github.com/fuderis),
  [*Behance*](https://behance.net/fuderis),
  [*Telegram*](https://t.me/fuderis),
  [*Telegram Channel*](https://t.me/fuderis_club),
  [*VKontakte*](https://vk.com/fuderis).
* **Donat**: `TON: UQB_r6IFgMYTJUKkhZNgjXcgp4aIJYwB6Gfiiukzg2lIC_Kc`

> Thank you for your support! Don't forget to check out my other projects as well. [*GitHub*](https://github.com/fuderis)<br>
**P.s.**: This software is actively evolving, and your suggestions and feedback are always welcome!
