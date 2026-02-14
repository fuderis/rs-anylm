use anylm::{Chunk, Completions, prelude::*};

#[tokio::main]
async fn main() -> Result<()> {
    // send request:
    let mut response = Completions::lmstudio(1234, "qwen2.5-coder-3b-instruct")
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
