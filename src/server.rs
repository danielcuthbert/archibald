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

            let mut buffer = [0; 1024];
            match stream.read(&mut buffer) {
                Ok(_) => {}
                Err(e) => {
                    println!("Error reading from the stream: {}", e);
                    let response = handler.handle_bad_request(&ParseError::InvalidRequest);
                    let _ = response.send(&mut stream);
                    continue;
                }
            };

            let request = match Requests::try_from(&buffer[..]) {
                Ok(req) => req,
                Err(e) => {
                    let response = handler.handle_bad_request(&e);
                    let _ = response.send(&mut stream);
                    continue;
                }
            };

            let response = handler
                .handle_request_internal(&request)
                .unwrap_or_else(|e| handler.handle_bad_request(&e));
            let _ = response.send(&mut stream);
        }
    }
}
