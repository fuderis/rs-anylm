use anylm::{Chunk, Completions, prelude::*};

#[tokio::main]
async fn main() -> Result<()> {
    // send request:
    let mut response = Completions::lmstudio("qwen2.5-coder-3b-instruct")
        .user_message("Hello, how are you doing?")
        .send()
        .await?;

    // read response stream:
    while let Some(Ok(Chunk { text })) = response.next().await {
        eprint!("{text}");
    }
    println!();

    Ok(())
}
