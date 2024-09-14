// src/server.rs

use crate::http::errors::ParseError;
use crate::http::{arch_requests::Requests, Response};
use std::convert::TryFrom;
use std::io::{Read, Write};
use std::net::TcpListener;

pub trait ServerHandler {
    fn handle_request(&mut self, request: &Requests) -> Response;
    fn handle_bad_request(&mut self, e: &ParseError) -> Response;
}

pub struct Server {
    address: String,
}

impl Server {
    pub fn new<T: Into<String>>(address: T) -> Self {
        Self {
            address: address.into(),
        }
    }

    pub fn run(self, mut handler: impl ServerHandler + Send) {
        println!("[*] Archibald: Starting to serve you on {}", self.address);

        let listener = TcpListener::bind(&self.address).expect("Failed to bind to address");

        for stream in listener.incoming() {
            let mut stream = match stream {
                Ok(s) => s,
                Err(e) => {
                    println!("Failed to establish a connection: {}", e);
                    continue;
                }
            };

            // Handle the connection
            if let Err(e) = Self::handle_connection(&mut stream, &mut handler) {
                println!("Failed to handle connection: {}", e);
            }
        }
    }

    fn handle_connection(
        stream: &mut (impl Read + Write),
        handler: &mut impl ServerHandler,
    ) -> std::io::Result<()> {
        let mut buffer = [0; 1024];
        let bytes_read = stream.read(&mut buffer)?;

        if bytes_read == 0 {
            // Handle client disconnect if necessary
            return Ok(());
        }

        // Log the number of bytes read for debugging
        log::debug!("Bytes read from stream: {}", bytes_read);

        // Pass only the valid bytes to the parser
        let request_bytes = &buffer[..bytes_read];

        let request = match Requests::try_from(request_bytes) {
            Ok(req) => req,
            Err(e) => {
                log::warn!("Failed to parse request: {:?}", e);
                let response = handler.handle_bad_request(&e);
                response.send(stream)?;
                return Ok(());
            }
        };

        let response = handler.handle_request(&request);
        response.send(stream)?;

        Ok(())
    }
}
