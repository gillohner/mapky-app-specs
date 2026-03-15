#[cfg(target_arch = "wasm32")]
use js_sys::Date;

use url::Url;

/// Returns the current timestamp in microseconds since the UNIX epoch.
#[cfg(target_arch = "wasm32")]
pub fn timestamp() -> i64 {
    let ms = Date::now() as i64;
    ms * 1_000
}

#[cfg(not(target_arch = "wasm32"))]
use std::time::{SystemTime, UNIX_EPOCH};

#[cfg(not(target_arch = "wasm32"))]
pub fn timestamp() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_micros() as i64
}

/// Trims whitespace and normalizes a URL if valid
pub fn sanitize_url(input: &str) -> String {
    let trimmed = input.trim();
    match Url::parse(trimmed) {
        Ok(parsed_url) => parsed_url.to_string(),
        Err(_) => trimmed.to_string(),
    }
}

/// Sanitizes a single tag label by trimming whitespace and converting to lowercase.
pub fn sanitize_tag_label(tag: &str) -> String {
    tag.trim().to_lowercase()
}

/// Validates a single tag label.
pub fn validate_tag_label(
    tag: &str,
    max_len: usize,
    min_len: usize,
    invalid_chars: &[char],
) -> Result<(), String> {
    let tag_len = tag.chars().count();

    if tag_len > max_len {
        return Err(format!(
            "Validation Error: Tag '{}' exceeds maximum length of {} characters",
            tag, max_len
        ));
    }
    if tag_len < min_len {
        return Err(format!(
            "Validation Error: Tag '{}' is shorter than minimum length of {} character",
            tag, min_len
        ));
    }

    if tag.chars().any(|c| c.is_whitespace()) {
        return Err(format!(
            "Validation Error: Tag '{}' contains whitespace characters",
            tag
        ));
    }

    if let Some(c) = tag.chars().find(|c| invalid_chars.contains(c)) {
        return Err(format!(
            "Validation Error: Tag '{}' contains invalid character: {}",
            tag, c
        ));
    }

    Ok(())
}
