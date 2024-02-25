use super::StatusCode;
use std::io::{Result as IoResult, Write};

#[derive(Debug)]
pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
    binary_body: Option<Vec<u8>>,
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Response {
            status_code,
            body,
            binary_body: None,
        }
    }

    pub fn new_with_binary(status_code: StatusCode, binary_body: Vec<u8>) -> Self {
        Response {
            status_code,
            body: None,
            binary_body: Some(binary_body),
        }
    }

    // Sends the response to the client
    pub fn send(&self, stream: &mut impl Write) -> IoResult<()> {
        let content_length = if let Some(body) = &self.body {
            write!(
                stream,
                "HTTP/1.1 {} {}\r\nContent-Length: {}\r\n\r\n{}",
                self.status_code as u16, // Convert StatusCode to u16 if necessary
                self.status_code.http_status_reason_phrase(), // Correct method name
                body.len(),
                body
            )?;
            body.len()
        } else if let Some(binary_body) = &self.binary_body {
            write!(
                stream,
                "HTTP/1.1 {} {}\r\nContent-Length: {}\r\n\r\n",
                self.status_code as u16, // Convert StatusCode to u16 if necessary
                self.status_code.http_status_reason_phrase(), // Correct method name
                binary_body.len()
            )?;
            stream.write_all(binary_body)?;
            binary_body.len()
        } else {
            write!(
                stream,
                "HTTP/1.1 {} {}\r\nContent-Length: 0\r\n\r\n",
                self.status_code as u16, // Convert StatusCode to u16 if necessary
                self.status_code.http_status_reason_phrase(), // Correct method name
            )?;
            0
        };

        Ok(())
    }

    // New method to send predefined error pages
    pub fn send_error(status_code: StatusCode, stream: &mut impl Write) -> IoResult<()> {
        let (content, reason_phrase) = match status_code {
            StatusCode::NotFound => (
                include_str!("../../static_content/404.html"), // Adjust path as necessary
                "Not Found",
            ),
            StatusCode::InternalServerError => (
                include_str!("../../static_content/500.html"), // Adjust path as necessary
                "Internal Server Error",
            ),
            _ => (
                "An unexpected error has occurred.", // Fallback message
                "Error",
            ),
        };

        write!(
            stream,
            "HTTP/1.1 {} {}\r\nContent-Length: {}\r\n\r\n{}",
            status_code as u16, // Convert StatusCode to u16 if necessary
            reason_phrase,
            content.len(),
            content
        )
    }
}
