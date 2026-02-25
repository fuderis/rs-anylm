use anylm::{Chunk, Completions, Proxy, prelude::*};

#[tokio::main]
async fn main() -> Result<()> {
    let api_key = std::env::var("OPENROUTER_API_KEY")?;

    // send request:
    let mut response = Completions::openrouter(api_key, "qwen/qwen3-vl-30b-a3b-thinking")
        .proxy(Proxy::all("socks5://127.0.0.1:1080")?)
        .user_message(vec!["Hello, how are you doing?".into()])
        .send()
        .await?;

    // read response stream:
    while let Some(chunk) = response.next().await {
        if let Chunk::Text(text) = chunk? {
            eprint!("{text}");
        };
    }
    println!();

    Ok(())
}
