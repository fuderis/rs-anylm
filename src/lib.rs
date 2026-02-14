#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]
pub mod error;
pub use error::{Error, Result, StdResult};
pub mod prelude;
pub mod utils;
pub use utils::*;

pub mod image;

pub mod api;
pub use api::{
    ApiKind, Chunk, Completions, Content, Embedded, Embedding, Embeddings, Message, Role, Stream,
    Usage,
};
