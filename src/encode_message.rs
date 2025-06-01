use serde_json::Result;

pub fn encode_message(message: &str) -> Result<String> {
    let content = serde_json::to_string(message)?;

    Ok(format!(
        "Content-Length: {}\r\n\r\n{}",
        content.len(),
        content
    ))
}
