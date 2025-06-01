use serde::Serialize;
use serde_json::Result;

/// # Content Part
///
/// [Reference](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#contentPart)
///
/// ## Example
///
/// ```text
/// Content-Length: <content_length>\r\n
/// \r\n
/// {
/// 	"jsonrpc": "2.0",
/// 	"id": 1,
/// 	"method": "textDocument/completion",
/// 	"params": {
/// 		...
/// 	}
/// }
/// ```
pub fn encode_message<T: Serialize>(message: T) -> Result<String> {
    let content = serde_json::to_string(&message)?;

    Ok(format!(
        "Content-Length: {}\r\n\r\n{}",
        content.len(),
        content
    ))
}

#[cfg(test)]
mod encode_message {
    use serde::Serialize;

    use super::*;

    #[test]
    fn expected() {
        #[derive(Serialize)]
        struct Content {
            id: usize,
        }

        assert_eq!(
            "Content-Length: 8\r\n\r\n{\"id\":1}",
            encode_message(Content { id: 1 }).unwrap()
        );
    }
}
