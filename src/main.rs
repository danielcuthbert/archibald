use archibald_handler::ArchibaldHandler;
use server::Server;
mod archibald_handler;
mod http;
mod server;
mod settings;
use settings::Settings;
use log::LevelFilter;
use simplelog::*;
use std::fs::File;
use std::fs::OpenOptions;



fn main() {
    // Load settings
    let settings = Settings::new().expect("Config loading failed");

    let log_file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(&settings.log.file_name) // Use the file name from settings
        .expect("Unable to open log file");

    // Open log file in append mode
    let log_file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("archibald_server.log")
        .expect("Unable to open log file");

    // Convert log level from settings to LevelFilter
    let log_level = match settings.log.level.as_str() {
        "debug" => LevelFilter::Debug,
        "info" => LevelFilter::Info,
        "warn" => LevelFilter::Warn,
        "error" => LevelFilter::Error,
        _ => LevelFilter::Info, // Default to Info
    };

    CombinedLogger::init(vec![
        TermLogger::new(
            log_level, // Use dynamic level
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(log_level, Config::default(), log_file), // Use dynamic level
    ])
    .unwrap();

    println!("Loading Archibald on: {}", settings.server.address);
    println!("Listening port is: {}", settings.server.port);

    // Initialize your server using settings
    let archibald = Server::new(format!(
        "{}:{}",
        settings.server.address, settings.server.port
    ));

    // Initialize handler (ArchibaldHandler) using settings
    archibald.run(ArchibaldHandler::new(settings.web.static_root));
}
