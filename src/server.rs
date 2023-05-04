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
    use crate::http::response;
    use crate::http::{requests::Request, Response, StatusCode};

    use std::convert::TryFrom;

    // use crate::http::{requests::Request, Response, StatusCode};

    use std::fmt::write;
    use std::io::{Read, Write};
    use std::net::TcpListener;

    pub trait ServerHandler {
        /// This is the main handler for the server. It will take a request and return a response.
        fn handle_request(&mut self, request: &Request) -> Response;

        /// This is the error handler for the server. It will take a request and return a response.

        fn handle_bad_request(&mut self, e: &ParseError) -> Response;
        //    println!("{}", e);
        //Response::new(StatusCode::BadRequest, None)
    }

    //use crate::http::errors;

    // by default all mods are private so we need to make this public
    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub struct Server {
        /// This is the address we're listening on and it is stored in a string.
        address: String,
    }

    impl Server {
        /// We need an implementation block to hold the implementation of the Server struct
        /// This holds all the functionality we want to use in the server.

        pub fn new<T: Into<String>>(address: T) -> Self {
            // We need to return a new Server struct with the address we're listening on
            Self {
                address: address.into(),
            }
        }
        // We now need a run method to start the server.
        // This will be called by the main function.
        // self just points to the instance of the struct
        pub fn run(self, mut handler: impl ServerHandler) {
            println!("[*] Archibald: Starting to serve you on {}", self.address);
            let listener = TcpListener::bind(&self.address).unwrap();

            loop {
                match listener.accept() {
            Ok((mut stream, addr)) => {
                println!("[*] Archibald: Oh hello {}", addr);
                let mut buffer = [0; 1024];
                stream.read(&mut buffer);
                let request = String::from_utf8(buffer.to_vec()).unwrap();
                println!("[*] Archibald: My Lord, you asked me: {}", request);

                let response = match Request::try_from(&buffer[..]) {
                    Ok(request) => match handler.handle_request(&request) {
                        Ok(response) => response.send(&mut stream).expect("Stream write failed"),
                        Err(e) => handler.handle_bad_request(&e),
                    },
                    Err(_) => todo!(),
                };

                if let Err(e) = response {
                    println!("Failed to send response: {}", e);
                }
            }
            Err(e) => println!("[!] Archibald: Terribly sorry old boy, I'm unable to accept the incoming connection: {}", e),
        }
            }
        }
    }
}

// End of the server struct.
