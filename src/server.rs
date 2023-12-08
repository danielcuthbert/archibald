use crate::http::errors::ParseError;
use crate::http::{arch_requests::Requests, Response};
use std::convert::TryFrom;
use std::io::Read;
use std::net::TcpListener;

pub trait ServerHandler {
    fn handle_request(&mut self, request: &Requests) -> Response;
    fn handle_bad_request(&mut self, e: &ParseError) -> Response;
    fn handle_request_internal(&mut self, request: &Requests) -> Result<Response, ParseError>;
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

    pub fn run(self, mut handler: impl ServerHandler) {
        println!("[*] Archibald: Starting to serve you on {}", self.address);

        let listener = match TcpListener::bind(&self.address) {
            Ok(listener) => listener,
            Err(e) => {
                println!("Failed to bind to address {}: {}", self.address, e);
                return;
            },
        };

        loop {
            match listener.accept() {
                Ok((mut stream, addr)) => {
                    println!("[*] Archibald: Oh hello {}", addr);
                    let mut buffer = [0; 1024];
                    let bytes_read = match stream.read(&mut buffer) {
                        Ok(size) => size,
                        Err(e) => {
                            println!("Error reading from stream: {}", e);
                            continue;
                        },
                    };

                    let human_request = match String::from_utf8(buffer[..bytes_read].to_vec()) {
                        Ok(req) => req,
                        Err(e) => {
                            println!("Invalid UTF-8 sequence: {}", e);
                            continue;
                        },
                    };

                    let human_request = match Requests::try_from(human_request.as_bytes()) {
                        Ok(req) => req,
                        Err(e) => {
                            // You might want to send a bad request response here
                            println!("Failed to parse request: {}", e);
                            continue;
                        },
                    };

                    match handler.handle_request_internal(&human_request) {
                        Ok(response) => {
                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send response: {}", e);
                            }
                        },
                        Err(e) => {
                            // Handle the bad request
                            let bad_response = handler.handle_bad_request(&e);
                            if let Err(err) = bad_response.send(&mut stream) {
                                println!("Failed to send error response: {}", err);
                            }
                        },
                    }
                },
                Err(e) => {
                    println!("Failed to accept connection: {}", e);
                    continue;
                },
            }
        }
    }
}
