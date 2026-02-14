// use crate::prelude::*;

/// Tokenizes text
pub fn tokenize(text: &str) -> Vec<u32> {
    let tokenizer = tiktoken_rs::cl100k_base().expect("Failed to create tokenizer");
    tokenizer.encode_with_special_tokens(text)
}

/// Returns tokens count in string
pub fn count_tokens(text: &str) -> usize {
    tokenize(text).len()
}
