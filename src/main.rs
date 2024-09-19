use archibald_handler::ArchibaldHandler;
use server::Server;
mod archibald_handler;
mod http;
mod server;
mod settings;
use log::LevelFilter;
use settings::Settings;
use simplelog::*;

use std::fs::OpenOptions;

fn main() {
    // Load settings
    let settings = Settings::new().expect("Config loading failed");

    // Open log file in append mode
    let log_file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(&settings.log.file_name) // Use the file name from settings
        .expect("Unable to open log file");

    // Convert log level from settings to LevelFilter
    let log_level = match settings.log.level.as_str() {
        "debug" => LevelFilter::Debug,
        "info" => LevelFilter::Info,
        "warn" => LevelFilter::Warn,
        "error" => LevelFilter::Error,
        _ => LevelFilter::Info, // Default to Info, CHANGE ME IF U WANT
    };

    // Initialize logging (to terminal and file)
    CombinedLogger::init(vec![
        TermLogger::new(
            log_level,                // Use dynamic level from settings
            Config::default(),        // Default config for terminal logging
            TerminalMode::Mixed,      // Log to both stdout and stderr
            ColorChoice::Auto,        // Use color automatically if supported
        ),
        WriteLogger::new(
            log_level,                // Use dynamic level from settings
            Config::default(),        // Default config for file logging
            log_file,                 // Log file we just opened
        ),
    ])
    .unwrap();

    // Log some basic info about the server start
    log::info!("Starting Archibald on: {}", settings.server.address);
    log::info!("Listening port is: {}", settings.server.port);

    // Initialize server using settings
    let archibald = Server::new(format!(
        "{}:{}",
        settings.server.address, settings.server.port
    ));

    // Initialize handler (ArchibaldHandler) using settings
    archibald.run(ArchibaldHandler::new(settings.web.static_root));
}
