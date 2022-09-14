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
    // the contents of these 3 fields get copied to new strings on the heap when we create a new response
    //pub statuscode: u16,
    //statuscode_raw: StatusCode,
    // pub statusmessage: String,
    body: Option<String>,
    status_code: StatusCode, //there might be no body so we can use Option<>
}

// impl Response {
//     // here we use a new public function that takes the above struct and returns a string
//     pub fn new<T: Into<u16>>(
//         statuscode_raw: T,
//         // statusmessage_raw: Option<String>,
//         body: Option<String>,
//     ) -> Self {
//         Response {
//             statuscode: statuscode_raw.into(),
//             // statusmessage: statusmessage_raw.unwrap_or_default(),
//             body,
//             statuscode_raw: todo!(),
//         }
//     }

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Response { body, status_code }
    }

    pub fn send(&self, stream: &mut impl Write) -> IoResult<()> {
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

//     pub fn send(&self, stream: &mut impl Write) -> IoResult<()> {
//         // write the status line
//         stream.write_all(format!("HTTP/1.1 {} \r\n", self.statuscode,).as_bytes())?;
//         // write the headers
//         stream.write_all(
//             format!(
//                 "Content-Type: text/html\r\nContent-Length: {}\r\n\r\n",
//                 self.body.as_ref().map(|s| s.len()).unwrap_or(0)
//             )
//             .as_bytes(),
//         )?;
//         // write the body
//         if let Some(body) = &self.body {
//             stream.write_all(body.as_bytes())?;
//         }
//         Ok(())
//         // We need to return the status code and the body
//     }
// }
