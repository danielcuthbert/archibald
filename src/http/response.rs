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
use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    net::TcpStream,
}; // same as FMT so we call it something else here

#[derive(Debug)]
pub struct Response {
    /// the contents of these 3 fields get copied to new strings on the heap when we create a new response
    //pub statuscode: u16,
    //statuscode_raw: StatusCode,
    statusmessage: String,
    body: Option<String>,
    status_code: StatusCode, //there might be no body so we can use Option<>
}

impl Response {
    /// This allows us to create a new response, we can use this to create a response with a body or without
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Response {
            body,
            status_code,
            statusmessage: status_code.to_string(),
        }
    }

    pub fn send(&self, stream: &mut impl Write) -> IoResult<()> {
        /// This allows us to send the response to the client
        let body = match &self.body {
            Some(b) => b,
            None => "",
        };

        write!(
            stream,
            "HTTP/1.1 {} {}\r\n\r\n{}",
            self.status_code,
            self.status_code.http_status_reason_phrase(),
            body
        )
    }
}

// I used TCPstream before but I think it's better to use the Write trait https://doc.rust-lang.org/std/io/trait.Write.html
// this way we can send what ever we want and be more generic and not have to worry about the type of data we are sending
