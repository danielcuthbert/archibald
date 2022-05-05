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
            println!("[*] Archibald: Starting to serve you on {}", self.address);
            // If we cannot bind to the supplied address, we will return an unrecoverable error
            let listener = TcpListener::bind(&self.address).unwrap();
            // we need a loop to continually listen for requests

            loop {
                // the listener has an accept method, so we can use this to check for incoming connections.
                // this could be a DoS condition as the socket will be closed when the value is dropped but what if the client never drops it?

                let incomingresult = listener.accept();

                if incomingresult.is_err() {
                    // if we cannot accept the connection, we will return an unrecoverable error
                    println!("[!] Archibald: Terribly sorry old boy, I'm unable to accept the incoming connection");
                    continue;
                }

                let (error, _) = incomingresult.unwrap();
            }
        }
    }
}
