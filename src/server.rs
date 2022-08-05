/*
* Archibald: a loyal web server
* HTTP server module
* Author: @danielcuthbert
*
*/

// Start of the server struct. This is the main struct that will be used to run the server.
// Every file in Rust is treated as a module.

// We want to use custom traits to return a response to the client.

pub mod archibaldserver {

    use crate::http::errors::ParseError;
    use crate::http::response;
    use crate::http::{requests::Request, Response, StatusCode};

    use std::convert::TryFrom;

    // use crate::http::{requests::Request, Response, StatusCode};

    use std::fmt::write;
    use std::io::{Read, Write};
    use std::net::TcpListener;

    pub trait ServerHandler {
        fn handle_request(&mut self, request: &Request) -> Response;

        // we also need a bad request handler here

        fn handle_bad_request(&mut self, e: &ParseError) -> Response;
        //    println!("{}", e);
        //Response::new(StatusCode::BadRequest, None)
    }

    //use crate::http::errors;

    // by default all mods are private so we need to make this public
    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub struct Server {
        // The address we're listening on is stored in a string.
        address: String,
    }

    // We need an implementation block to hold the implementation of the Server struct
    // This holds all the functionality we want to use in the server.
    impl Server {
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
            // If we cannot bind to the supplied address, we will return an unrecoverable error
            let listener = TcpListener::bind(&self.address).unwrap();
            // we need a loop to continually listen for requests

            loop {
                // the listener has an accept method, so we can use this to check for incoming connections.
                // this could be a DoS condition as the socket will be closed when the value is dropped but what if the client never drops it?
                // let incomingresult = listener.accept();
                match listener.accept() {
                    Ok((mut stream, addr)) => {
                        println!("[*] Archibald: Oh hello {}", addr);
                        // we need to read the request from the client
                        // we use an array to store the request, filled with zeros initially and then the length. 
                        // this is because we don't know how long the request will be. however this could also cause issues if the size is too large.
                        let mut buffer = [0; 1024];
                        stream.read(&mut buffer);
                        // we need to convert the buffer to a string
                        let request = String::from_utf8(buffer.to_vec()).unwrap();
                        // we need to print the request to the console
                        println!("[*] Archibald: My Lord, you asked me: {}", request);
                        //using the requests function to parse the request
                        //the buffer doesn't know how to handle the array so adding [..] includes the entire array
                        //
                        let response = match Request::try_from(&buffer[..]){
                            Ok(request) => {
                                match Request::try_from(&buffer[..]) {
                                    Ok(request) => {
                                        handler.handle_request(&request)
                                    },
                                    Err(e) => {
                                        handler.handle_bad_request(&e)},
                                }.send(&mut stream).expect("Stream write failed"); //this uses the send function in response.rs to send the response to the client
                            }
                            Err(_) => todo!(),
                        };
                    },
                    Err(e) => println!("[!] Archibald: Terribly sorry old boy, I'm unable to accept the incoming connection: {}", e),
                }
            }
        }
    }
}

// End of the server struct.
