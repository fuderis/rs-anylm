use crate::prelude::*;
use base64::{Engine as _, engine};
use std::{fs, path::Path};

/// Validates a base64 URL
pub fn validate_base64(base64_url: &str) -> bool {
    engine::general_purpose::STANDARD.decode(base64_url).is_ok()
}

/// Creates a new base64 image url (example: "data:image/png;base64,iVBORw0KGgoA...")
pub fn base64(base64_url: impl Into<String>) -> Result<String> {
    let base64_url = base64_url.into();

    if validate_base64(&base64_url.split_once(",").ok_or(Error::InvalidBase64Url)?.1) {
        Ok(base64_url)
    } else {
        Err(Error::InvalidBase64Url.into())
    }
}

/// Creates a new base64 image from file path
pub fn read(file_path: impl AsRef<Path>) -> Result<String> {
    // reading file:
    let file_path = file_path.as_ref();
    let file_content = fs::read(&file_path)?;

    // reading mime-type:
    let mime_type = match file_path.extension().and_then(|e| e.to_str()) {
        Some("png") => "image/png",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("gif") => "image/gif",
        _ => "application/octet-stream",
    };

    // encoding into base64:
    let encoded = engine::general_purpose::STANDARD.encode(&file_content);
    let base64_url = fmt!("data:{};base64,{}", mime_type, encoded);

    Ok(base64_url)
}
