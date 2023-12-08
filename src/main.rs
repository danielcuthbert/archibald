use archibald_handler::ArchibaldHandler;

use http::methods::Allowedmethods;


use server::Server;

mod archibald_handler;
mod http;
mod server;
mod settings;
use settings::Settings;
use serde::Deserialize;
use std::{fs, process::exit};
use std::fs::File;

use log::LevelFilter;
use simplelog::*;


// #[derive(Deserialize)]
// struct ArchibaldConfig {
//     static_path: String,
//     default_path: String,
//     ip: String,
//     port: u16,
// }

// #[derive(Deserialize)]
// struct Data {
//     config: ArchibaldConfig,
// }

fn main() {
    

    // Load settings
    let settings = Settings::new().expect("Config loading failed");

    // Create a log file
    let log_file = File::create("archibald_server.log").unwrap();

    // Initialize combined logger
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
            WriteLogger::new(LevelFilter::Info, Config::default(), log_file),
        ]
    ).unwrap();

    println!("Loading Archibald on: {}", settings.server.address);
    println!("Listening port is: {}", settings.server.port);

    // Initialize your server using settings
    let archibald = Server::new(format!("{}:{}", settings.server.address, settings.server.port));
    
    // Initialize handler (ArchibaldHandler) using settings
    archibald.run(ArchibaldHandler::new(settings.web.static_root));
}

