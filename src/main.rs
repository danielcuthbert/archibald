/// Archibald is a web server written in Rust
///
/// This is the main file for the server
/// Author: @danielcuthbert
///
/// The corresponding threat model for this can be found in the same repo
use http::methods::Allowedmethods;
//use http::requests::Request;
use archibald_handler::ArchibaldHandler;

use log::{info, warn, LevelFilter};
use server::archibaldserver::Server;
mod archibald_handler;
mod http;
mod server;

use std::env;

// Start of the main function
fn main() {
    #[cfg(debug_assertions)]
    // before we start, we need to read environment variables
    // for now, we need to pass this like so: ARCHIBALD_STATIC_PATH=~/Code/Archibald/static_content cargo run
    let default_path = format!("{}/static_content", env!("CARGO_MANIFEST_DIR")); // this is the path to the cargo manifest directory
    let static_path = env::var("ARCHIBALD_STATIC_PATH").unwrap_or(default_path); // this is the path to the static content
    log::info!("STATIC PATH: {}", static_path);
    // We need to tell it what to listen on. We'll use the default port, but we can change it
    // We should support IPv6 and IPv4 at the same time. We'll use the default IPv4 for now
    // we need to convert the ip/port to a string
    let archibald = Server::new("127.0.0.1:8080");

    // Tell the server what HTTP methods we want to support
    let _get = Allowedmethods::GET;
    let _post = Allowedmethods::POST;

    // Start the server, this will always run
    // we need to read the path from an environment variable
    archibald.run(ArchibaldHandler::new(static_path));
}

// In order to accept and process incoming requests, we need to store them somewhere.
// For this we can use a struct
