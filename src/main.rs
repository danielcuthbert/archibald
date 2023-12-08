use archibald_handler::ArchibaldHandler;

use http::methods::Allowedmethods;


use server::Server;

mod archibald_handler;
mod http;
mod server;
mod settings;
use settings::Settings;
use serde::{Deserialize};
use std::{fs, process::exit};

#[derive(Deserialize)]
struct ArchibaldConfig {
    static_path: String,
    default_path: String,
    ip: String,
    port: u16,
}

#[derive(Deserialize)]
struct Data {
    config: ArchibaldConfig,
}

fn main() {
    // Load settings using the `Settings::new()` method
    let settings = Settings::new().expect("Config loading failed");

    println!("Loading Archibald on: {}", settings.server.address);
    println!("Listening port is: {}", settings.server.port);

    // Initialize your server using settings
    let archibald = Server::new(format!("{}:{}", settings.server.address, settings.server.port));
    
    // Initialize your handler (ArchibaldHandler) using settings
    // Assuming ArchibaldHandler::new() takes the path to the web root directory
    archibald.run(ArchibaldHandler::new(settings.web.static_root));
}

