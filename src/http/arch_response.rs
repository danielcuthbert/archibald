/*
* Archibald: a loyal web server
* Main response module
* Author: @danielcuthbert
*
* This code serves as the response function.
*/

// We define a struct to hold all the data we need to send back to the client
use super::StatusCode;
use std::io::{Result as IoResult, Write};
 // same as FMT so we call it something else here


#[derive(Debug)]
pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Response {
            status_code,
            body,
        }
    }

    pub fn send(&self, stream: &mut impl Write) -> IoResult<()> {
        let body = self.body.as_deref().unwrap_or("");
        let content_length = body.len();

        write!(
            stream,
            "HTTP/1.1 {} {}\r\nContent-Length: {}\r\n\r\n{}",
            self.status_code,
            self.status_code.http_status_reason_phrase(),
            content_length,
            body
        )
    }
}

// I used TCPstream before but I think it's better to use the Write trait https://doc.rust-lang.org/std/io/trait.Write.html
// this way we can send what ever we want and be more generic and not have to worry about the type of data we are sending
