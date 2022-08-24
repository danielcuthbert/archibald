/*
* Archibald: a loyal web server
* Written with love and care
* Author: @danielcuthbert
* The corresponding threat model for this can be found in the same repo
*/

use http::methods::Allowedmethods;
//use http::requests::Request;
use archibald_handler::ArchibaldHandler;
use log::{info, warn, LevelFilter};
use server::archibaldserver::Server;
use simplelog::{ColorChoice, CombinedLogger, Config, TermLogger, TerminalMode, WriteLogger};
mod archibald_handler;
mod http;
mod server;
use std::{env, fs::File};

// Start of the main function
fn main() {
    // Set up the logging using Simple log and the TermLogger to write to console and a log file in the main root
    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Warn,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            LevelFilter::Info,
            Config::default(),
            File::create("archibald_server.log").unwrap(),
        ),
    ])
    .unwrap();

    // before we start, we need to read environment variables
    // for now, we need to pass this like so: ARCHIBALD_STATIC_PATH=~/Code/Archibald/static_content cargo run
    let default_path = format!("{}/static_content", env!("CARGO_MANIFEST_DIR")); // this is the path to the cargo manifest directory
    let static_path = env::var("ARCHIBALD_STATIC_PATH").unwrap_or(default_path); // this is the path to the static content
    info!("STATIC PATH: {}", static_path);
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
