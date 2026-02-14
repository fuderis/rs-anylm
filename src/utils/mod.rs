// use crate::prelude::*;

/// Returns tokens count in string
pub fn count_tokens(text: &str) -> usize {
    let tokenizer = tiktoken_rs::cl100k_base().expect("Failed to create tokenizer");
    tokenizer.encode_with_special_tokens(text).len()
}
