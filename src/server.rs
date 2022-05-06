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
    
    use std::fmt::Result;
    use std::net::TcpListener;
    use std::io::Read;
    //use crate::http::errors;

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
        
    }
    Err(e) => println!("[!] Archibald: Terribly sorry old boy, I'm unable to accept the incoming connection: {}", e),

}
            }
        }
    }
}
        
    

// End of the server struct.
