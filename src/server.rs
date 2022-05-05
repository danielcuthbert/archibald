/*
* Archibald: a loyal web server
* HTTP server module
* Author: @danielcuthbert
*
*/

// Start of the server struct. This is the main struct that will be used to run the server.
// Every file in Rust is treated as a module.

// Todo:
// TCP socket is not implemented yet. we can use the net module to create a TCP socket.
// Logging is not implemented yet.

pub mod archibaldserver {
    // by default all mods are private so we need to make this public
    pub struct Server {
        // The address we're listening on is stored in a string.
        address: String,
    }

    // We need an implementation block to hold the implementation of the Server struct
    // This holds all the functionality we want to use in the server.
    impl Server {
        pub fn new(address: String) -> Self {
            // We need to return a new Server struct with the address we're listening on
            Self { address: address }
        }
        // We now need a run method to start the server.
        // This will be called by the main function.
        // self just points to the instance of the struct
        pub fn run(self) {
            use std::net::TcpListener;
            println!("Starting server on {}", self.address);
            let listener = TcpListener::bind(&self.address); // we need to pass a ref otherwise we wont be able to use it again
        }
    }
}
