use anylm::Embeddings;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;

#[tokio::main]
async fn main() -> Result<()> {
    // send request:
    let embeddings = Embeddings::lmstudio("", "nomic-ai/nomic-embed-text-v1.5")
        .input("Hello, how are you doing?")
        .send()
        .await?;

    // print response:
    println!("Embeddings: {:?}", embeddings.data);

    Ok(())
}
