/*
* Archibald: a loyal web server
* HTTP server module
* Author: @danielcuthbert
*
*/

// Every file in Rust is treated as a module.

// We want to use custom traits to return a response to the client.

pub mod archibaldserver {
    /// This is the main struct that will be used to run the server.
    use crate::http::errors::ParseError;
    
    use crate::http::{arch_requests::Requests, Response};

    use std::convert::TryFrom;

    // use crate::http::{requests::Request, Response, StatusCode};

    use std::fmt::write;
    use std::io::{Read, Write};
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
            let listener = TcpListener::bind(&self.address).unwrap();
    
            loop {
                match listener.accept() {
                    Ok((mut stream, addr)) => {
                        println!("[*] Archibald: Oh hello {}", addr);
                        let mut buffer = [0; 1024];
                        let bytes_read = stream.read(&mut buffer).unwrap();
                        println!("[*] Archibald: I read {} bytes", bytes_read);
    
                        let human_request = String::from_utf8(buffer.to_vec()).unwrap();
    
                        let human_request = Requests::try_from(human_request.as_bytes()).unwrap();
                        println!("[*] Archibald: My Lord, you asked me: {}", human_request);
    
                        let response = handler.handle_request_internal(&human_request);
    
                        if let Err(e) = response.send(&mut stream) {
                            println!("Failed to send response: {}", e);
                        }
                    },
                    Err(_) => todo!(),
                }
            }
        }
    }
    
    