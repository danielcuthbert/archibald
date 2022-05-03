/*
* Archibald: a loyal web server
* Written with love and care
* Author: Daniel
* The corresponding threat model for this can be found in the same repo

*/

// Start of the main function
fn main() {
    // We need to tell it what to listen on. We'll use the default port, but we can change it
    // We should support IPv6 and IPv4 at the same time. We'll use the default IPv4 for now
    // we need to convert the ip/port to a string
    let archibald = Server::new("0.0.0.0:80".to_string());

    // Start the server, this will always run
    archibald.run();
}

// Start of the server struct. This is the main struct that will be used to run the server
struct Server {
    // The address we're listening on is stored in a string.
    address: String,
}

// We need an implementation block to hold the implementation of the Server struct
// This holds all the functionality we want to use in the server.
impl Server {
    fn new(address: String) -> Self {
        // We need to return a new Server struct with the address we're listening on
        Self { address: address }
    }
    // We now need a run method to start the server.
    // This will be called by the main function.
    // self just points to the instance of the struct
    fn run(self) {
        println!("Starting server on {}", self.address);
    }
}
