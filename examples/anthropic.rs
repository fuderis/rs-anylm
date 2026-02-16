use anylm::{Chunk, Completions, prelude::*};

#[tokio::main]
async fn main() -> Result<()> {
    let api_key = std::env::var("ANTHROPIC_API_KEY")?;

    // send request:
    let mut response = Completions::anthropic(api_key, "claude-opus-4-6")
        .proxy(reqwest::Proxy::all("socks5://127.0.0.1:1080")?)
        .user_message(vec!["Hello, how are you doing?".into()])
        .send()
        .await?;

    // read response stream:
    while let Some(Ok(Chunk { text })) = response.next().await {
        eprint!("{text}");
    }
    println!();

    Ok(())
}
