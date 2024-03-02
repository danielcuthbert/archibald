use super::StatusCode;
use std::io::{Result as IoResult, Write};

#[derive(Debug)]
pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
    binary_body: Option<Vec<u8>>,
    headers: Vec<(String, String)>, // Added field to store headers
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Response {
            status_code,
            body,
            binary_body: None,
            headers: Vec::new(), // Initialize headers as empty
        }
    }

    pub fn new_with_binary(status_code: StatusCode, binary_body: Vec<u8>) -> Self {
        Response {
            status_code,
            body: None,
            binary_body: Some(binary_body),
            headers: Vec::new(), // Initialize headers as empty
        }
    }

    // Method to add a header to the response
    pub fn add_header(mut self, name: &str, value: &str) -> Self {
        self.headers.push((name.to_string(), value.to_string()));
        self
    }

    // Sends the response to the client, including headers
    pub fn send(&self, stream: &mut impl Write) -> IoResult<()> {
        write!(
            stream,
            "HTTP/1.1 {} {}\r\n",
            self.status_code as u16,
            self.status_code.http_status_reason_phrase(),
        )?;

        // Iterate over headers and write them to the stream
        for (name, value) in &self.headers {
            write!(stream, "{}: {}\r\n", name, value)?;
        }

        // Content-Length is determined and set here
        let content_length = self.body.as_ref().map_or(0, |b| b.len())
            + self.binary_body.as_ref().map_or(0, |b| b.len());
        write!(stream, "Content-Length: {}\r\n\r\n", content_length)?;

        if let Some(body) = &self.body {
            write!(stream, "{}", body)?;
        } else if let Some(binary_body) = &self.binary_body {
            stream.write_all(binary_body)?;
        }

        Ok(())
    }

    // Adjusted send_error method to fit with the new structure
    pub fn send_error(status_code: StatusCode) -> Self {
        let (content, content_type) = match status_code {
            StatusCode::NotFound => (
                include_str!("../../static_content/404.html").to_string(),
                "text/html; charset=utf-8",
            ),
            StatusCode::InternalServerError => (
                include_str!("../../static_content/500.html").to_string(),
                "text/html; charset=utf-8",
            ),
            _ => (
                "An unexpected error has occurred.".to_string(),
                "text/plain; charset=utf-8",
            ),
        };

        Response::new(status_code, Some(content)).add_header("Content-Type", &content_type)
    }
}
