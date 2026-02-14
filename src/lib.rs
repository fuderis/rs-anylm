#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]
pub mod error;
pub use error::{Error, Result, StdResult};
pub mod prelude;
pub mod utils;

pub mod api;
pub use api::{ApiKind, Chunk, Completions, Message, Role, Stream};
