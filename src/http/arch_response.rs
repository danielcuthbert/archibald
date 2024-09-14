// src/http/arch_response.rs

use crate::http::statuscodes::StatusCode;
use std::collections::HashMap;
use std::io::Write;

pub struct Response {
    pub status_code: StatusCode,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

impl Response {
    /// Creates a new `Response` with the given status code, body, and content type.
    pub fn new(status_code: StatusCode, body: Vec<u8>, content_type: &str) -> Self {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), content_type.to_string());
        headers.insert("Content-Length".to_string(), body.len().to_string());
        headers.insert("Connection".to_string(), "close".to_string());
        Response {
            status_code,
            headers,
            body,
        }
    }

    /// Adds a header to the response.
    pub fn add_header(mut self, key: &str, value: &str) -> Self {
        self.headers.insert(key.to_string(), value.to_string());
        self
    }

    /// Creates a new `Response` for text content.
    pub fn new_with_text(status_code: StatusCode, body: &str, content_type: &str) -> Self {
        Self::new(status_code, body.as_bytes().to_vec(), content_type)
    }

    /// Sends the HTTP response over the provided writable stream.
    pub fn send(&self, stream: &mut impl Write) -> std::io::Result<()> {
        // Write the status line
        write!(
            stream,
            "HTTP/1.1 {} {}\r\n",
            self.status_code as u16,
            self.status_code.reason_phrase()
        )?;

        // Write the headers
        for (key, value) in &self.headers {
            write!(stream, "{}: {}\r\n", key, value)?;
        }

        // End of headers
        write!(stream, "\r\n")?;

        // Write the body
        stream.write_all(&self.body)?;
        Ok(())
    }
}
